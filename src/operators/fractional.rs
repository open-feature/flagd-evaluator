//! Fractional operator for percentage-based bucket assignment.
//!
//! The fractional operator uses consistent hashing to assign users to buckets
//! for A/B testing scenarios.

use super::common::OperatorResult;
use datalogic_rs::{ContextStack, Error as DataLogicError, Evaluator, Operator};
use murmurhash3::murmurhash3_x86_32;
use serde_json::Value;

/// Custom operator for fractional/percentage-based bucket assignment.
///
/// The fractional operator uses consistent hashing to assign users to buckets
/// for A/B testing scenarios.
pub struct FractionalOperator;

impl Operator for FractionalOperator {
    fn evaluate(
        &self,
        args: &[Value],
        context: &mut ContextStack,
        evaluator: &dyn Evaluator,
    ) -> OperatorResult<Value> {
        if args.is_empty() {
            return Err(DataLogicError::InvalidArguments(
                "fractional operator requires at least one bucket definition".into(),
            ));
        }

        // Evaluate the first argument to determine bucketing key logic.
        // If the first arg is an Array literal, treat all args as buckets (no explicit seed).
        // If the first arg is an expression that evaluates to a String, use it as seed.
        // If the first arg is an expression that evaluates to null/non-string, return an error
        // so the flag engine falls back to the defaultVariant.
        let evaluated_first = evaluator.evaluate(&args[0], context)?;
        let (bucket_key, start_index) = match (&args[0], &evaluated_first) {
            (_, Value::String(s)) => (s.clone(), 1),
            (Value::Array(_), _) => {
                // First arg is explicitly an array — no seed provided, use flagKey+targetingKey
                let data = context.root().data().clone();
                let targeting_key = data
                    .get("targetingKey")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let flag_key = data
                    .get("$flagd")
                    .and_then(|v| v.get("flagKey"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                (format!("{}{}", flag_key, targeting_key), 0)
            }
            _ => {
                // Expression resolved to null or non-string — no valid seed; signal fallback
                return Err(DataLogicError::Custom(
                    "fractional: bucket key expression resolved to a non-string value".into(),
                ));
            }
        };

        // Parse bucket definitions from remaining arguments.
        // Each inner element (name and weight) is evaluated through JSON Logic so that
        // nested expressions like {"if": [...]} and {"var": "..."} are resolved first.
        let mut bucket_values: Vec<Value> = Vec::new();

        if start_index == 1 && args.len() == 2 {
            // Single array format: ["key", ["bucket1", 50, "bucket2", 50]]
            let evaluated_buckets = evaluator.evaluate(&args[1], context)?;
            if let Some(arr) = evaluated_buckets.as_array() {
                bucket_values.extend_from_slice(arr);
            } else {
                return Err(DataLogicError::InvalidArguments(
                    "Second argument must be an array of bucket definitions".into(),
                ));
            }
        } else {
            // Multiple array format: ["key", ["bucket1", 50], ["bucket2", 50]]
            // or shorthand: [["bucket1"], ["bucket2", weight]]
            for arg in &args[start_index..] {
                let evaluated = evaluator.evaluate(arg, context)?;
                if let Some(bucket_def) = evaluated.as_array() {
                    // Each bucket is [name, weight] or [name] (weight=1).
                    // Evaluate each inner element so nested JSON Logic is resolved.
                    if bucket_def.len() >= 2 {
                        bucket_values.push(evaluator.evaluate(&bucket_def[0], context)?);
                        bucket_values.push(evaluator.evaluate(&bucket_def[1], context)?);
                    } else if bucket_def.len() == 1 {
                        // Shorthand: [name] implies weight of 1
                        bucket_values.push(evaluator.evaluate(&bucket_def[0], context)?);
                        bucket_values.push(Value::Number(1.into()));
                    }
                } else {
                    return Err(DataLogicError::InvalidArguments(format!(
                        "Bucket definition must be an array, got: {:?}",
                        evaluated
                    )));
                }
            }
        }

        match fractional(&bucket_key, &bucket_values) {
            Ok(value) => Ok(value),
            Err(e) => Err(DataLogicError::Custom(e)),
        }
    }
}

/// Evaluates the fractional operator for consistent bucket assignment.
///
/// The fractional operator takes a bucket key (typically a user ID) and
/// a list of bucket definitions with integer weights. It uses consistent hashing
/// to always assign the same bucket key to the same bucket.
///
/// Bucket names may be any JSON scalar (string, boolean, number). The name is
/// serialised to its JSON representation for display/return, while the hash is
/// computed on the bucket key string only.
///
/// Negative weights are clamped to zero (the bucket still participates in the
/// name list but never receives any traffic).
///
/// # Algorithm
///
/// Uses high-resolution integer arithmetic instead of float-based percentage
/// bucketing. The MurmurHash3 value is mapped uniformly into `[0, totalWeight)`
/// using a single multiply-and-shift:
///
/// ```text
/// bucket = (u64(hash) * u64(totalWeight)) >> 32
/// ```
///
/// Weights must sum to at most `i32::MAX` (2,147,483,647).
///
/// # Arguments
/// * `bucket_key` - The key to use for bucket assignment (e.g., user ID)
/// * `buckets` - Array of [name, weight, name, weight, ...] values
///
/// # Returns
/// The value of the selected bucket as a `serde_json::Value`, or an error if
/// the input is invalid.
pub fn fractional(bucket_key: &str, buckets: &[Value]) -> Result<Value, String> {
    if buckets.is_empty() {
        return Err("Fractional operator requires at least one bucket".to_string());
    }

    // Parse bucket definitions: [name1, weight1, name2, weight2, ...]
    let mut bucket_defs: Vec<(Value, u64)> = Vec::new();
    let mut total_weight: u64 = 0;

    let mut i = 0;
    while i < buckets.len() {
        // Accept any scalar JSON value as a bucket name.
        let name_value = buckets[i].clone();
        if matches!(name_value, Value::Object(_) | Value::Array(_)) {
            return Err(format!(
                "Bucket name at index {} must be a scalar value (string, boolean, or number), got a complex type",
                i
            ));
        }

        i += 1;

        // Get bucket weight — negative weights are clamped to zero.
        if i >= buckets.len() {
            return Err(format!("Missing weight for bucket at index {}", i - 1));
        }

        let weight: u64 = match &buckets[i] {
            Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    u
                } else if let Some(signed) = n.as_i64() {
                    // Negative integer — clamp to zero
                    signed.max(0) as u64
                } else {
                    // Float — round down, clamp to zero
                    n.as_f64().unwrap_or(0.0).max(0.0) as u64
                }
            }
            _ => {
                return Err(format!(
                    "Weight for bucket at index {} must be a number",
                    i
                ))
            }
        };

        total_weight = total_weight
            .checked_add(weight)
            .ok_or_else(|| "Total weight overflow".to_string())?;

        bucket_defs.push((name_value, weight));
        i += 1;
    }

    if bucket_defs.is_empty() {
        return Err("No valid bucket definitions found".to_string());
    }

    if total_weight == 0 {
        return Err("Total weight must be greater than zero".to_string());
    }

    // Weights must not exceed MaxInt32 to ensure safe integer arithmetic
    if total_weight > i32::MAX as u64 {
        return Err(format!(
            "Total weight {} exceeds maximum allowed value of {}",
            total_weight,
            i32::MAX
        ));
    }

    // Hash the bucket key using MurmurHash3 (seed=0, matches Apache Commons MurmurHash3.hash32x86)
    let hash: u32 = murmurhash3_x86_32(bucket_key.as_bytes(), 0);

    // Map the 32-bit hash uniformly into [0, totalWeight) using integer arithmetic.
    let bucket_value: u64 = (hash as u64 * total_weight) >> 32;

    // Find which bucket this value falls into by accumulating weights.
    // Buckets with zero weight are skipped (their cumulative weight never advances).
    let mut cumulative_weight: u64 = 0;
    for (value, weight) in &bucket_defs {
        cumulative_weight += weight;
        if bucket_value < cumulative_weight {
            return Ok(value.clone());
        }
    }

    // Unreachable for valid inputs, but fall back to last non-zero bucket defensively.
    Ok(bucket_defs
        .iter()
        .rev()
        .find(|(_, w)| *w > 0)
        .map(|(v, _)| v.clone())
        .unwrap_or(Value::Null))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_fractional_50_50_consistent() {
        let buckets = vec![json!("control"), json!(50), json!("treatment"), json!(50)];

        // Same key must always yield the same bucket
        let result1 = fractional("user-123", &buckets).unwrap();
        let result2 = fractional("user-123", &buckets).unwrap();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_fractional_50_50_both_reachable() {
        let buckets = vec![json!("control"), json!(50), json!("treatment"), json!(50)];

        let mut seen_control = false;
        let mut seen_treatment = false;
        for i in 0..100 {
            match fractional(&format!("user-{}", i), &buckets).unwrap() {
                Value::String(s) if s == "control" => seen_control = true,
                Value::String(s) if s == "treatment" => seen_treatment = true,
                other => panic!("Unexpected bucket: {:?}", other),
            }
        }
        assert!(seen_control, "control bucket must be reachable");
        assert!(seen_treatment, "treatment bucket must be reachable");
    }

    #[test]
    fn test_fractional_unequal_weights() {
        let buckets = vec![json!("small"), json!(10), json!("large"), json!(90)];

        let mut small_count = 0u32;
        let mut large_count = 0u32;
        for i in 0..1000 {
            match fractional(&format!("user-{}", i), &buckets).unwrap() {
                Value::String(s) if s == "small" => small_count += 1,
                Value::String(s) if s == "large" => large_count += 1,
                other => panic!("Unexpected bucket: {:?}", other),
            }
        }
        // 90/10 split — large should dominate
        assert!(
            large_count > small_count * 3,
            "large ({}) should dominate small ({})",
            large_count,
            small_count
        );
    }

    #[test]
    fn test_fractional_high_resolution_weights() {
        // Weights well above 100 — impossible with old float/percentage approach
        let buckets = vec![
            json!("a"),
            json!(1000),
            json!("b"),
            json!(1000),
            json!("c"),
            json!(1000),
        ];
        let mut counts = std::collections::HashMap::new();
        for i in 0..3000 {
            let r = fractional(&format!("u-{}", i), &buckets).unwrap();
            *counts.entry(r.to_string()).or_insert(0u32) += 1;
        }
        for bucket in ["\"a\"", "\"b\"", "\"c\""] {
            let c = counts.get(bucket).copied().unwrap_or(0);
            // Each should get roughly 1/3; allow generous tolerance
            assert!(
                c > 500 && c < 1500,
                "bucket '{}' got {} assignments (expected ~1000)",
                bucket,
                c
            );
        }
    }

    #[test]
    fn test_fractional_max_int32_total_weight() {
        // Total weight exactly at the MaxInt32 boundary must succeed
        let half = i32::MAX as u64 / 2;
        let remainder = i32::MAX as u64 - half * 2;
        let buckets = vec![
            json!("a"),
            Value::Number(serde_json::Number::from(half)),
            json!("b"),
            Value::Number(serde_json::Number::from(half + remainder)),
        ];
        let result = fractional("any-key", &buckets);
        assert!(result.is_ok(), "MaxInt32 total weight must be accepted");
    }

    #[test]
    fn test_fractional_exceeds_max_int32_rejected() {
        let over: u64 = i32::MAX as u64 + 1;
        let buckets = vec![json!("a"), Value::Number(serde_json::Number::from(over))];
        let result = fractional("any-key", &buckets);
        assert!(result.is_err(), "Total weight > MaxInt32 must be rejected");
        assert!(
            result.unwrap_err().contains("exceeds maximum"),
            "Error message should mention maximum"
        );
    }

    #[test]
    fn test_fractional_empty_buckets() {
        assert!(fractional("user-123", &[]).is_err());
    }

    #[test]
    fn test_fractional_missing_weight() {
        let buckets = vec![json!("only-name")];
        assert!(fractional("user-123", &buckets).is_err());
    }

    #[test]
    fn test_fractional_invalid_name_type() {
        // Complex types (objects/arrays) are rejected as bucket names
        let buckets = vec![json!({"key": "value"}), json!(50)];
        assert!(fractional("user-123", &buckets).is_err());
    }

    #[test]
    fn test_fractional_invalid_weight_type() {
        let buckets = vec![json!("bucket"), json!("not-a-number")];
        assert!(fractional("user-123", &buckets).is_err());
    }

    #[test]
    fn test_fractional_single_bucket() {
        let buckets = vec![json!("only"), json!(100)];
        assert_eq!(fractional("any-key", &buckets).unwrap(), json!("only"));
        assert_eq!(fractional("another-key", &buckets).unwrap(), json!("only"));
    }

    #[test]
    fn test_fractional_integer_arithmetic_matches_spec() {
        // Verify the formula: bucket_value = (hash as u64 * totalWeight) >> 32
        let key = "test-key";
        let hash = murmurhash3_x86_32(key.as_bytes(), 0);
        let total_weight: u64 = 100;
        let expected_bucket_value = (hash as u64 * total_weight) >> 32;

        let buckets = vec![json!("low"), json!(50u64), json!("high"), json!(50u64)];
        let result = fractional(key, &buckets).unwrap();

        let expected = if expected_bucket_value < 50 {
            json!("low")
        } else {
            json!("high")
        };
        assert_eq!(result, expected, "bucket assignment must match the spec formula");
    }

    #[test]
    fn test_fractional_boolean_bucket_names() {
        // Boolean values are valid bucket names — used when fractional is a condition
        let buckets = vec![json!(false), json!(0u64), json!(true), json!(100u64)];
        let result = fractional("any-key", &buckets).unwrap();
        assert_eq!(result, json!(true), "100% weight on true must always select true");
    }

    #[test]
    fn test_fractional_negative_weight_clamped_to_zero() {
        // Negative weight is clamped to 0; the other bucket gets 100% of traffic
        let buckets = vec![
            json!("one"),
            Value::Number(serde_json::Number::from(-50i64)),
            json!("two"),
            json!(100),
        ];
        let result = fractional("any-key", &buckets).unwrap();
        assert_eq!(result, json!("two"), "negative weight must be clamped to 0");
    }

    #[test]
    fn test_fractional_all_zero_weights_error() {
        let buckets = vec![json!("one"), json!(0), json!("two"), json!(0)];
        assert!(fractional("any-key", &buckets).is_err());
    }
}
