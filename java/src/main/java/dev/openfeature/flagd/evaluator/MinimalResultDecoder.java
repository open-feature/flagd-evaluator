package dev.openfeature.flagd.evaluator;

import java.nio.charset.StandardCharsets;

/**
 * Minimal, allocation-light decoder for the WASM evaluation result JSON, used to bypass
 * Jackson on the hot path.
 *
 * <p>The WASM module serializes a small, flat result object
 * ({@code {"value":..,"variant":"..","reason":".."}} plus optional error/metadata fields).
 * For the common primitive case — a Boolean or String value with only the
 * {@code value}/{@code variant}/{@code reason} fields and no escape sequences — this scans
 * the raw UTF-8 bytes directly and builds the {@link EvaluationResult} without Jackson.
 *
 * <p>It is deliberately conservative: <b>any</b> deviation from that simple shape
 * (object/array values, numeric/{@code Value} types, {@code errorCode}/{@code errorMessage}/
 * {@code flagMetadata} fields, string escapes, or malformed input) returns {@code null}, and
 * the caller falls back to the full Jackson parse over the same bytes. This keeps behavior
 * identical to Jackson for everything it does not handle.
 */
final class MinimalResultDecoder {

    private MinimalResultDecoder() {
    }

    /**
     * Attempts to decode the result. Returns {@code null} if the caller should fall back to Jackson.
     *
     * @param type the expected value type (only Boolean and String are fast-pathed)
     * @param buf  the buffer holding the result JSON (UTF-8)
     * @param len  the number of valid bytes in {@code buf}
     */
    @SuppressWarnings("unchecked")
    static <T> EvaluationResult<T> decode(Class<T> type, byte[] buf, int len) {
        if (type != Boolean.class && type != String.class) {
            return null;
        }
        try {
            Scanner s = new Scanner(buf, len);
            s.ws();
            if (!s.eat('{')) {
                return null;
            }

            Object value = null;
            boolean haveValue = false;
            String variant = null;
            String reason = null;
            boolean first = true;

            s.ws();
            while (s.peek() != '}') {
                if (!first) {
                    if (!s.eat(',')) {
                        return null;
                    }
                    s.ws();
                }
                first = false;

                String key = s.string();
                if (key == null) {
                    return null;
                }
                s.ws();
                if (!s.eat(':')) {
                    return null;
                }
                s.ws();

                switch (key) {
                    case "value":
                        if (type == Boolean.class) {
                            Boolean b = s.bool();
                            if (b == null) {
                                return null;
                            }
                            value = b;
                        } else { // String
                            if (s.peek() != '"') {
                                return null;
                            }
                            String sv = s.string();
                            if (sv == null) {
                                return null;
                            }
                            value = sv;
                        }
                        haveValue = true;
                        break;
                    case "variant":
                        variant = s.string();
                        if (variant == null) {
                            return null;
                        }
                        break;
                    case "reason":
                        reason = s.string();
                        if (reason == null) {
                            return null;
                        }
                        break;
                    default:
                        // errorCode / errorMessage / flagMetadata / unknown -> Jackson fallback
                        return null;
                }
                s.ws();
            }
            s.next(); // consume '}'
            s.ws();
            if (!s.atEnd()) {
                return null;
            }

            EvaluationResult<T> result = new EvaluationResult<>();
            if (haveValue) {
                result.setValue((T) value);
            }
            if (variant != null) {
                result.setVariant(variant);
            }
            if (reason != null) {
                result.setReason(reason);
            }
            return result;
        } catch (RuntimeException e) {
            // Any unexpected input shape -> fall back to Jackson.
            return null;
        }
    }

    /** Tiny forward-only byte scanner over a UTF-8 JSON buffer. */
    private static final class Scanner {
        private final byte[] buf;
        private final int len;
        private int pos;

        Scanner(byte[] buf, int len) {
            this.buf = buf;
            this.len = len;
        }

        boolean atEnd() {
            return pos >= len;
        }

        int peek() {
            return pos < len ? (buf[pos] & 0xFF) : -1;
        }

        void next() {
            pos++;
        }

        boolean eat(char c) {
            if (pos < len && buf[pos] == (byte) c) {
                pos++;
                return true;
            }
            return false;
        }

        void ws() {
            while (pos < len) {
                byte b = buf[pos];
                if (b == ' ' || b == '\t' || b == '\n' || b == '\r') {
                    pos++;
                } else {
                    break;
                }
            }
        }

        /** Reads a JSON string with no escape sequences; returns null on escapes or malformed input. */
        String string() {
            if (pos >= len || buf[pos] != '"') {
                return null;
            }
            pos++;
            int start = pos;
            while (pos < len) {
                byte b = buf[pos];
                if (b == '"') {
                    String out = new String(buf, start, pos - start, StandardCharsets.UTF_8);
                    pos++;
                    return out;
                }
                if (b == '\\') {
                    return null; // escapes -> fall back to Jackson
                }
                pos++;
            }
            return null; // unterminated
        }

        /** Reads a JSON boolean literal; returns null if the next token is not true/false. */
        Boolean bool() {
            if (matches("true")) {
                pos += 4;
                return Boolean.TRUE;
            }
            if (matches("false")) {
                pos += 5;
                return Boolean.FALSE;
            }
            return null;
        }

        private boolean matches(String literal) {
            int n = literal.length();
            if (pos + n > len) {
                return false;
            }
            for (int i = 0; i < n; i++) {
                if (buf[pos + i] != (byte) literal.charAt(i)) {
                    return false;
                }
            }
            return true;
        }
    }
}
