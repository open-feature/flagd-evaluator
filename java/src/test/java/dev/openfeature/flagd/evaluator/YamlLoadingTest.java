package dev.openfeature.flagd.evaluator;

import dev.openfeature.sdk.EvaluationContext;
import dev.openfeature.sdk.MutableContext;
import org.junit.jupiter.api.Test;
import static org.assertj.core.api.Assertions.*;

class YamlLoadingTest {

    private static final String SIMPLE_YAML =
        "flags:\n" +
        "  bool-flag:\n" +
        "    state: ENABLED\n" +
        "    variants:\n" +
        "      'on': true\n" +
        "      'off': false\n" +
        "    defaultVariant: 'on'\n" +
        "  string-flag:\n" +
        "    state: ENABLED\n" +
        "    variants:\n" +
        "      v1: hello\n" +
        "      v2: world\n" +
        "    defaultVariant: v1\n";

    @Test
    void updateStateFromYamlLoadsBooleanFlag() throws Exception {
        FlagEvaluator evaluator = new FlagEvaluator();
        evaluator.updateStateFromYaml(SIMPLE_YAML);

        EvaluationContext ctx = new MutableContext("user-1");
        EvaluationResult<Boolean> result = evaluator.evaluateFlag(Boolean.class, "bool-flag", ctx);
        assertThat(result.getValue()).isTrue();
    }

    @Test
    void updateStateFromYamlLoadsStringFlag() throws Exception {
        FlagEvaluator evaluator = new FlagEvaluator();
        evaluator.updateStateFromYaml(SIMPLE_YAML);

        EvaluationContext ctx = new MutableContext("user-1");
        EvaluationResult<String> result = evaluator.evaluateFlag(String.class, "string-flag", ctx);
        assertThat(result.getValue()).isEqualTo("hello");
    }

    @Test
    void updateStateFromYamlInvalidYamlThrowsEvaluatorException() {
        FlagEvaluator evaluator = new FlagEvaluator();
        assertThatThrownBy(() -> evaluator.updateStateFromYaml("flags:\n  bad: [unclosed"))
            .isInstanceOf(EvaluatorException.class);
    }

    @Test
    void updateStateFromYamlAndJsonProduceSameResults() throws Exception {
        FlagEvaluator yamlEvaluator = new FlagEvaluator();
        yamlEvaluator.updateStateFromYaml(SIMPLE_YAML);

        FlagEvaluator jsonEvaluator = new FlagEvaluator();
        jsonEvaluator.updateState("{\"flags\":{\"bool-flag\":{\"state\":\"ENABLED\",\"variants\":{\"on\":true,\"off\":false},\"defaultVariant\":\"on\"}}}");

        EvaluationContext ctx = new MutableContext("user-1");
        EvaluationResult<Boolean> yamlResult = yamlEvaluator.evaluateFlag(Boolean.class, "bool-flag", ctx);
        EvaluationResult<Boolean> jsonResult = jsonEvaluator.evaluateFlag(Boolean.class, "bool-flag", ctx);
        assertThat(yamlResult.getValue()).isEqualTo(jsonResult.getValue());
        assertThat(yamlResult.getVariant()).isEqualTo(jsonResult.getVariant());
    }
}
