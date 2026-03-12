package evaluator

import "time"

// EvaluationResult contains the result of a flag evaluation.
type EvaluationResult struct {
	Value        interface{}            `json:"value"`
	Variant      string                 `json:"variant,omitempty"`
	Reason       string                 `json:"reason"`
	ErrorCode    string                 `json:"errorCode,omitempty"`
	ErrorMessage string                 `json:"errorMessage,omitempty"`
	FlagMetadata map[string]interface{} `json:"flagMetadata,omitempty"`
}

// IsError returns true if the evaluation resulted in an error.
func (r *EvaluationResult) IsError() bool {
	return r.ErrorCode != ""
}

// UpdateStateResult contains the result of updating flag state.
type UpdateStateResult struct {
	Success             bool                         `json:"success"`
	Error               string                       `json:"error,omitempty"`
	ChangedFlags        []string                     `json:"changedFlags,omitempty"`
	PreEvaluated        map[string]*EvaluationResult `json:"preEvaluated,omitempty"`
	RequiredContextKeys map[string][]string          `json:"requiredContextKeys,omitempty"`
	FlagIndices         map[string]uint32            `json:"flagIndices,omitempty"`
}

// Option configures a FlagEvaluator.
type Option func(*evaluatorConfig)

type evaluatorConfig struct {
	permissiveValidation bool
	poolSize             int
	evaluationTimeout    time.Duration
}

// WithPermissiveValidation configures the evaluator to accept invalid flag
// configurations with warnings instead of rejecting them.
func WithPermissiveValidation() Option {
	return func(c *evaluatorConfig) {
		c.permissiveValidation = true
	}
}

// WithPoolSize sets the number of WASM instances in the evaluation pool.
// More instances allow more parallel targeting evaluations.
// Defaults to runtime.NumCPU().
func WithPoolSize(n int) Option {
	return func(c *evaluatorConfig) {
		c.poolSize = n
	}
}

// WithEvaluationTimeout sets a per-call deadline for WASM evaluation.
// If a single EvaluateFlag call exceeds this duration, it is cancelled and
// returns an error. This prevents goroutines from hanging indefinitely inside
// the wazero interpreter under GC pressure or resource contention.
// A value of 0 (the default) disables the per-call timeout.
func WithEvaluationTimeout(d time.Duration) Option {
	return func(c *evaluatorConfig) {
		c.evaluationTimeout = d
	}
}

// Evaluation reasons
const (
	ReasonStatic         = "STATIC"
	ReasonDefault        = "DEFAULT"
	ReasonTargetingMatch = "TARGETING_MATCH"
	ReasonDisabled       = "DISABLED"
	ReasonError          = "ERROR"
	ReasonFlagNotFound   = "FLAG_NOT_FOUND"
)

// Error codes
const (
	ErrorFlagNotFound = "FLAG_NOT_FOUND"
	ErrorParseError   = "PARSE_ERROR"
	ErrorTypeMismatch = "TYPE_MISMATCH"
	ErrorGeneral      = "GENERAL"
)
