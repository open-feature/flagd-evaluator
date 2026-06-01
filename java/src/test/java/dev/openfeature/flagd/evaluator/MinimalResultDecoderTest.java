package dev.openfeature.flagd.evaluator;

import static org.assertj.core.api.Assertions.assertThat;

import java.nio.charset.StandardCharsets;
import org.junit.jupiter.api.Test;

/**
 * Verifies the fast-path {@link MinimalResultDecoder} matches the field values Jackson would
 * produce for the supported primitive cases, and declines (returns null) for everything else so
 * the caller falls back to Jackson.
 */
class MinimalResultDecoderTest {

    private static byte[] b(String s) {
        return s.getBytes(StandardCharsets.UTF_8);
    }

    private static <T> EvaluationResult<T> decode(Class<T> type, String json) {
        byte[] buf = b(json);
        return MinimalResultDecoder.decode(type, buf, buf.length);
    }

    @Test
    void decodesBooleanTargetingMatch() {
        EvaluationResult<Boolean> r =
                decode(Boolean.class, "{\"value\":true,\"variant\":\"on\",\"reason\":\"TARGETING_MATCH\"}");
        assertThat(r).isNotNull();
        assertThat(r.getValue()).isEqualTo(Boolean.TRUE);
        assertThat(r.getVariant()).isEqualTo("on");
        assertThat(r.getReason()).isEqualTo("TARGETING_MATCH");
        assertThat(r.getErrorCode()).isNull();
    }

    @Test
    void decodesBooleanDefaultFalse() {
        EvaluationResult<Boolean> r =
                decode(Boolean.class, "{\"value\":false,\"variant\":\"off\",\"reason\":\"DEFAULT\"}");
        assertThat(r).isNotNull();
        assertThat(r.getValue()).isEqualTo(Boolean.FALSE);
        assertThat(r.getVariant()).isEqualTo("off");
        assertThat(r.getReason()).isEqualTo("DEFAULT");
    }

    @Test
    void decodesStringValue() {
        EvaluationResult<String> r =
                decode(String.class, "{\"value\":\"blue\",\"variant\":\"b\",\"reason\":\"STATIC\"}");
        assertThat(r).isNotNull();
        assertThat(r.getValue()).isEqualTo("blue");
        assertThat(r.getVariant()).isEqualTo("b");
        assertThat(r.getReason()).isEqualTo("STATIC");
    }

    @Test
    void declinesObjectValue() {
        assertThat(decode(Boolean.class, "{\"value\":{\"a\":1},\"variant\":\"x\",\"reason\":\"STATIC\"}"))
                .isNull();
    }

    @Test
    void declinesWhenErrorFieldsPresent() {
        assertThat(decode(
                        Boolean.class,
                        "{\"value\":false,\"variant\":\"off\",\"reason\":\"ERROR\","
                                + "\"errorCode\":\"FLAG_NOT_FOUND\",\"errorMessage\":\"nope\"}"))
                .isNull();
    }

    @Test
    void declinesWhenFlagMetadataPresent() {
        assertThat(decode(
                        Boolean.class,
                        "{\"value\":true,\"variant\":\"on\",\"reason\":\"STATIC\",\"flagMetadata\":{\"x\":1}}"))
                .isNull();
    }

    @Test
    void declinesUnsupportedType() {
        assertThat(decode(Integer.class, "{\"value\":42,\"variant\":\"v\",\"reason\":\"STATIC\"}"))
                .isNull();
    }

    @Test
    void declinesStringWithEscape() {
        assertThat(decode(String.class, "{\"value\":\"a\\\"b\",\"variant\":\"v\",\"reason\":\"STATIC\"}"))
                .isNull();
    }

    @Test
    void declinesMalformed() {
        assertThat(decode(Boolean.class, "{\"value\":tru")).isNull();
        assertThat(decode(Boolean.class, "not json")).isNull();
    }
}
