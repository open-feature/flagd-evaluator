//! Integration tests for the flagd-evaluator library.
//!
//! These tests verify the complete evaluation flow including memory management,
//! JSON parsing, custom operators, and error handling.

use flagd_evaluator::{
    alloc, dealloc, pack_ptr_len, unpack_ptr_len, FlagEvaluator, ValidationMode,
};

// ============================================================================
// Memory Management
// ============================================================================

#[test]
fn test_alloc_dealloc() {
    let ptr = alloc(100);
    assert!(!ptr.is_null());
    dealloc(ptr, 100);
}

#[test]
fn test_alloc_zero_bytes() {
    let ptr = alloc(0);
    assert!(ptr.is_null());
}

#[test]
fn test_multiple_allocations() {
    let mut pointers = Vec::new();

    for size in [10, 100, 1000, 10000] {
        let ptr = alloc(size);
        assert!(!ptr.is_null());
        pointers.push((ptr, size));
    }

    for (ptr, size) in pointers {
        dealloc(ptr, size);
    }
}

#[test]
fn test_pack_unpack_ptr_len() {
    let original_ptr = 0x12345678 as *const u8;
    let original_len = 999u32;

    let packed = pack_ptr_len(original_ptr, original_len);
    let (unpacked_ptr, unpacked_len) = unpack_ptr_len(packed);

    assert_eq!(unpacked_ptr, original_ptr);
    assert_eq!(unpacked_len, original_len);
}

// ============================================================================
// update_state integration tests
// ============================================================================

#[test]
fn test_update_state_success() {
    let config = r#"{
        "flags": {
            "testFlag": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {
                    "on": true,
                    "off": false
                }
            }
        }
    }"#;
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    evaluator
        .update_state(config)
        .expect("expect to be updating");
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let response = evaluator.update_state(config).unwrap();
    assert!(response.success);

    // Verify the state was actually stored
    let state = evaluator.get_state();
    assert!(state.is_some());
    let state = state.unwrap();
    assert_eq!(state.flags.len(), 1);
    assert!(state.flags.contains_key("testFlag"));
}

#[test]
fn test_update_state_invalid_json() {
    let config = "not valid json";
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let response = evaluator.update_state(config).unwrap();
    assert!(!response.success);
    let err = response.error.unwrap();
    // Error should be JSON format with validation errors
    assert!(err.contains("Invalid JSON") || err.contains("\"valid\":false"));
}

#[test]
fn test_update_state_missing_flags_field() {
    let config = r#"{"other": "data"}"#;
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let response = evaluator.update_state(config).unwrap();
    assert!(!response.success);
    let err = response.error.unwrap();
    // Error should indicate missing required field or invalid schema
    assert!(err.contains("\"valid\":false") || err.contains("required"));
}

#[test]
fn test_update_state_replaces_existing_state() {
    // First configuration
    let config1 = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            }
        }
    }"#;
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let response = evaluator.update_state(config1).unwrap();
    assert!(response.success);

    // Verify first state
    let state = evaluator.get_state().unwrap();
    assert!(state.flags.contains_key("flag1"));

    // Second configuration should replace the first
    let config2 = r#"{
        "flags": {
            "flag2": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"off": false}
            }
        }
    }"#;
    let response = evaluator.update_state(config2).unwrap();
    assert!(response.success);

    // Verify state was replaced
    let state = evaluator.get_state().unwrap();
    assert!(!state.flags.contains_key("flag1"));
    assert!(state.flags.contains_key("flag2"));
    assert_eq!(state.flags.len(), 1);
}

#[test]
fn test_update_state_with_targeting() {
    let config = r#"{
        "flags": {
            "complexFlag": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {
                    "on": true,
                    "off": false
                },
                "targeting": {
                    "if": [
                        {">=": [{"var": "age"}, 18]},
                        "on",
                        "off"
                    ]
                }
            }
        }
    }"#;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let response = evaluator.update_state(config).unwrap();
    assert!(response.success);

    let state = evaluator.get_state().unwrap();
    let flag = state.flags.get("complexFlag").unwrap();
    assert!(flag.targeting.is_some());
}

#[test]
fn test_update_state_with_metadata() {
    let config = r#"{
        "$schema": "https://flagd.dev/schema/v0/flags.json",
        "metadata": {
            "environment": "test",
            "version": 1
        },
        "$evaluators": {
            "emailWithFaas": {
                "in": ["@faas.com", {"var": ["email"]}]
            }
        },
        "flags": {
            "myFlag": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            }
        }
    }"#;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let response = evaluator.update_state(config).unwrap();
    assert!(response.success);

    let state = evaluator.get_state().unwrap();
    // $schema and $evaluators should NOT be in flag_set_metadata
    assert!(!state.flag_set_metadata.contains_key("$schema"));
    assert!(!state.flag_set_metadata.contains_key("$evaluators"));
    // But the flattened metadata should be there
    assert_eq!(
        state.flag_set_metadata.get("environment"),
        Some(&serde_json::json!("test"))
    );
    assert_eq!(
        state.flag_set_metadata.get("version"),
        Some(&serde_json::json!(1))
    );
}

#[test]
fn test_update_state_empty_flags() {
    let config = r#"{"flags": {}}"#;
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let result = evaluator.update_state(config);
    assert!(result.is_ok());

    let state = evaluator.get_state().unwrap();
    assert_eq!(state.flags.len(), 0);
}

#[test]
fn test_update_state_multiple_flags() {
    let config = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true, "off": false}
            },
            "flag2": {
                "state": "DISABLED",
                "defaultVariant": "red",
                "variants": {"red": "red", "blue": "blue"}
            },
            "flag3": {
                "state": "ENABLED",
                "defaultVariant": "default",
                "variants": {"default": {"key": "value"}}
            }
        }
    }"#;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let result = evaluator.update_state(config);
    assert!(result.is_ok());

    let state = evaluator.get_state().unwrap();
    assert_eq!(state.flags.len(), 3);
    assert!(state.flags.contains_key("flag1"));
    assert!(state.flags.contains_key("flag2"));
    assert!(state.flags.contains_key("flag3"));
}

#[test]
fn test_update_state_invalid_flag_structure() {
    let config = r#"{
        "flags": {
            "badFlag": {
                "state": "ENABLED"
            }
        }
    }"#;
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
    let response = evaluator.update_state(config).unwrap();
    assert!(!response.success);
    let err = response.error.unwrap();
    // Error should indicate validation failure due to missing required fields
    assert!(err.contains("\"valid\":false") || err.contains("required"));
}

// ============================================================================
// Tests for $evaluators and $ref resolution
// ============================================================================

#[test]
fn test_evaluators_simple_ref_evaluation() {
    use serde_json::json;
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    let config = r#"{
        "$evaluators": {
            "isAdmin": {
                "in": ["admin@", {"var": "email"}]
            }
        },
        "flags": {
            "adminFeature": {
                "state": "ENABLED",
                "variants": {
                    "on": true,
                    "off": false
                },
                "defaultVariant": "off",
                "targeting": {
                    "if": [
                        {"$ref": "isAdmin"},
                        "on",
                        "off"
                    ]
                }
            }
        }
    }"#;

    // Update state
    let result = evaluator.update_state(config);
    assert!(result.is_ok(), "Failed to update state: {:?}", result);

    // Test with admin email - should return true
    let context = json!({"email": "admin@example.com"});
    let eval_result = evaluator.evaluate_flag("adminFeature", context);
    assert_eq!(eval_result.value, json!(true));
    assert_eq!(eval_result.variant, Some("on".to_string()));

    // Test with non-admin email - should return false
    let context = json!({"email": "user@example.com"});
    let eval_result = evaluator.evaluate_flag("adminFeature", context);
    assert_eq!(eval_result.value, json!(false));
    assert_eq!(eval_result.variant, Some("off".to_string()));
}

#[test]
fn test_evaluators_nested_ref_evaluation() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    let config = r#"{
        "$evaluators": {
            "isAdmin": {
                "starts_with": [{"var": "email"}, "admin@"]
            },
            "isActive": {
                "==": [{"var": "status"}, "active"]
            },
            "isActiveAdmin": {
                "and": [
                    {"$ref": "isAdmin"},
                    {"$ref": "isActive"}
                ]
            }
        },
        "flags": {
            "premiumFeature": {
                "state": "ENABLED",
                "variants": {
                    "enabled": "premium",
                    "disabled": "free"
                },
                "defaultVariant": "disabled",
                "targeting": {
                    "if": [
                        {"$ref": "isActiveAdmin"},
                        "enabled",
                        "disabled"
                    ]
                }
            }
        }
    }"#;
    evaluator
        .update_state(config)
        .expect("state should be updated");

    // Test with active admin - should return premium
    let context = json!({"email": "admin@company.com", "status": "active"});
    let result = evaluator.evaluate_flag("premiumFeature", context);
    assert_eq!(result.value, json!("premium"));
    assert_eq!(result.variant, Some("enabled".to_string()));

    // Test with non-admin - should return free
    let context = json!({"email": "user@company.com", "status": "active"});
    let result = evaluator.evaluate_flag("premiumFeature", context);
    assert_eq!(result.value, json!("free"));
    assert_eq!(result.variant, Some("disabled".to_string()));

    // Test with admin but inactive - should return free
    let context = json!({"email": "admin@company.com", "status": "inactive"});
    let result = evaluator.evaluate_flag("premiumFeature", context);
    assert_eq!(result.value, json!("free"));
    assert_eq!(result.variant, Some("disabled".to_string()));
}

#[test]
fn test_evaluators_with_fractional_operator() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Permissive);

    evaluator.clear_state();

    let config = r#"{
        "$evaluators": {
            "abTestSplit": {
                "fractional": [
                    {"var": "userId"},
                    ["control", 50],
                    ["treatment", 50]
                ]
            }
        },
        "flags": {
            "experimentFlag": {
                "state": "ENABLED",
                "variants": {
                    "control": "control-experience",
                    "treatment": "treatment-experience"
                },
                "defaultVariant": "control",
                "targeting": {
                    "$ref": "abTestSplit"
                }
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");

    // Test with specific user ID - should consistently return same variant
    let context = json!({"userId": "user-123"});
    let result1 = evaluator.evaluate_flag("experimentFlag", context.clone());
    let result2 = evaluator.evaluate_flag("experimentFlag", context);
    assert_eq!(result1.value, result2.value);
    assert!(
        result1.value == json!("control-experience")
            || result1.value == json!("treatment-experience")
    );
}

#[test]
fn test_evaluators_complex_targeting() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    let config = r#"{
        "$evaluators": {
            "isPremiumUser": {
                "==": [{"var": "tier"}, "premium"]
            },
            "isHighValue": {
                ">=": [{"var": "lifetime_value"}, 1000]
            },
            "isVIPUser": {
                "or": [
                    {"$ref": "isPremiumUser"},
                    {"$ref": "isHighValue"}
                ]
            }
        },
        "flags": {
            "vipFeatures": {
                "state": "ENABLED",
                "variants": {
                    "vip": {"features": ["advanced", "priority_support", "custom_reports"]},
                    "standard": {"features": ["basic"]}
                },
                "defaultVariant": "standard",
                "targeting": {
                    "if": [
                        {
                            "and": [
                                {"$ref": "isVIPUser"},
                                {"==": [{"var": "active"}, true]}
                            ]
                        },
                        "vip",
                        "standard"
                    ]
                }
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");

    // Premium + active - should get VIP
    let context = json!({"tier": "premium", "lifetime_value": 500, "active": true});
    let result = evaluator.evaluate_flag("vipFeatures", context);
    assert_eq!(result.variant, Some("vip".to_string()));

    // High value + active - should get VIP
    let context = json!({"tier": "basic", "lifetime_value": 1500, "active": true});
    let result = evaluator.evaluate_flag("vipFeatures", context);
    assert_eq!(result.variant, Some("vip".to_string()));

    // Premium but inactive - should get standard
    let context = json!({"tier": "premium", "lifetime_value": 500, "active": false});
    let result = evaluator.evaluate_flag("vipFeatures", context);
    assert_eq!(result.variant, Some("standard".to_string()));

    // Neither premium nor high value - should get standard
    let context = json!({"tier": "basic", "lifetime_value": 100, "active": true});
    let result = evaluator.evaluate_flag("vipFeatures", context);
    assert_eq!(result.variant, Some("standard".to_string()));
}

#[test]
fn test_evaluators_missing_ref_in_storage() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    let config = r#"{
        "$evaluators": {
            "validRule": {
                "==": [{"var": "x"}, 1]
            }
        },
        "flags": {
            "testFlag": {
                "state": "ENABLED",
                "variants": {"on": true, "off": false},
                "defaultVariant": "off",
                "targeting": {
                    "$ref": "nonExistentRule"
                }
            }
        }
    }"#;

    let result = evaluator.update_state(config);
    let response = result.unwrap();
    assert!(!response.success);
    let err = response.error.unwrap();
    // The error is now a validation error from boon, not a parsing error
    // It should contain either "validation failed" or reference to the error
    assert!(err.contains("validation failed") || err.contains("nonExistentRule"));
}

#[test]
fn test_evaluators_multiple_refs_in_single_flag() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    let config = r#"{
        "$evaluators": {
            "isAdmin": {
                "starts_with": [{"var": "email"}, "admin@"]
            },
            "isManager": {
                "starts_with": [{"var": "email"}, "manager@"]
            }
        },
        "flags": {
            "accessFlag": {
                "state": "ENABLED",
                "variants": {
                    "full": "full-access",
                    "limited": "limited-access",
                    "none": "no-access"
                },
                "defaultVariant": "none",
                "targeting": {
                    "if": [
                        {"$ref": "isAdmin"},
                        "full",
                        {
                            "if": [
                                {"$ref": "isManager"},
                                "limited",
                                "none"
                            ]
                        }
                    ]
                }
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");

    // Admin gets full access
    let context = json!({"email": "admin@company.com"});
    let result = evaluator.evaluate_flag("accessFlag", context);
    assert_eq!(result.value, json!("full-access"));

    // Manager gets limited access
    let context = json!({"email": "manager@company.com"});
    let result = evaluator.evaluate_flag("accessFlag", context);
    assert_eq!(result.value, json!("limited-access"));

    // Regular user gets no access
    let context = json!({"email": "user@company.com"});
    let result = evaluator.evaluate_flag("accessFlag", context);
    assert_eq!(result.value, json!("no-access"));
}

// ============================================================================
// Tests for changed flags detection in update_state
// ============================================================================

#[test]
fn test_update_state_changed_flags_on_first_update() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    let config = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            },
            "flag2": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"off": false}
            }
        }
    }"#;

    let response = evaluator.update_state(config).unwrap();
    assert!(response.success);
    let changed = response.changed_flags.unwrap();
    assert_eq!(changed.len(), 2);
    assert!(changed.contains(&"flag1".to_string()));
    assert!(changed.contains(&"flag2".to_string()));
}

#[test]
fn test_update_state_changed_flags_partial_update() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    // Initial config
    let config1 = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            },
            "flag2": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"off": false}
            }
        }
    }"#;
    evaluator
        .update_state(config1)
        .expect("state should be updated");

    // Update - modify flag1, keep flag2 same
    let config2 = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"on": true}
            },
            "flag2": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"off": false}
            }
        }
    }"#;

    let response = evaluator.update_state(config2).unwrap();
    assert!(response.success);
    let changed = response.changed_flags.unwrap();
    assert_eq!(changed.len(), 1);
    assert!(changed.contains(&"flag1".to_string()));
}

#[test]
fn test_update_state_changed_flags_targeting_change() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    // Initial config
    let config1 = r#"{
        "flags": {
            "featureFlag": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"on": true, "off": false},
                "targeting": {
                    "if": [
                        {"==": [{"var": "tier"}, "premium"]},
                        "on",
                        "off"
                    ]
                }
            }
        }
    }"#;
    evaluator
        .update_state(config1)
        .expect("state should be updated");

    // Update with different targeting rule
    let config2 = r#"{
        "flags": {
            "featureFlag": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"on": true, "off": false},
                "targeting": {
                    "if": [
                        {"==": [{"var": "tier"}, "enterprise"]},
                        "on",
                        "off"
                    ]
                }
            }
        }
    }"#;

    let response = evaluator.update_state(config2).unwrap();
    assert!(response.success);
    let changed = response.changed_flags.unwrap();
    assert_eq!(changed.len(), 1);
    assert!(changed.contains(&"featureFlag".to_string()));
}

#[test]
fn test_update_state_changed_flags_metadata_change() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    // Initial config
    let config1 = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true},
                "metadata": {
                    "description": "Original"
                }
            }
        }
    }"#;
    evaluator
        .update_state(config1)
        .expect("state should be updated");

    // Update with different metadata
    let config2 = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true},
                "metadata": {
                    "description": "Updated"
                }
            }
        }
    }"#;

    let response = evaluator.update_state(config2).unwrap();
    assert!(response.success);
    let changed = response.changed_flags.unwrap();
    assert_eq!(changed.len(), 1);
    assert!(changed.contains(&"flag1".to_string()));
}

#[test]
fn test_update_state_changed_flags_no_changes() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    let config = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            }
        }
    }"#;

    // First update
    evaluator
        .update_state(config)
        .expect("state should be updated");

    // Second update with same config
    let response = evaluator.update_state(config).unwrap();
    assert!(response.success);
    let changed = response.changed_flags.unwrap();
    assert_eq!(changed.len(), 0);
}

#[test]
fn test_update_state_changed_flags_add_and_remove() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    evaluator.clear_state();

    // Initial config
    let config1 = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            },
            "flag2": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"off": false}
            }
        }
    }"#;
    evaluator
        .update_state(config1)
        .expect("state should be updated");

    // Remove flag2, add flag3
    let config2 = r#"{
        "flags": {
            "flag1": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            },
            "flag3": {
                "state": "ENABLED",
                "defaultVariant": "red",
                "variants": {"red": "red"}
            }
        }
    }"#;

    let response = evaluator.update_state(config2).unwrap();
    assert!(response.success);
    let changed = response.changed_flags.unwrap();
    assert_eq!(changed.len(), 2);
    assert!(changed.contains(&"flag2".to_string())); // Removed
    assert!(changed.contains(&"flag3".to_string())); // Added
    assert!(!changed.contains(&"flag1".to_string())); // Unchanged
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_fractional_single_bucket() {
    use serde_json::json;

    // Single bucket with 100% weight should always return that bucket
    // Use permissive mode to allow single-bucket fractional
    let mut evaluator = FlagEvaluator::new(ValidationMode::Permissive);

    let config = r#"{
        "flags": {
            "singleBucket": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"on": true, "off": false},
                "targeting": {
                    "fractional": [
                        ["on", 100]
                    ]
                }
            }
        }
    }"#;

    let result = evaluator.update_state(config);
    assert!(
        result.is_ok(),
        "Should be able to update state: {:?}",
        result
    );

    // Any context should get "on" variant
    for i in 0..10 {
        let context = json!({"targetingKey": format!("user-{}", i)});
        let result = evaluator.evaluate_flag("singleBucket", context);
        assert_eq!(
            result.variant,
            Some("on".to_string()),
            "User {} should get 'on' variant",
            i
        );
    }
}

#[test]
fn test_fractional_unequal_weights() {
    use serde_json::json;

    // 90/10 split - most users should get variant A
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    let config = r#"{
        "flags": {
            "heavyA": {
                "state": "ENABLED",
                "defaultVariant": "off",
                "variants": {"a": "variant-a", "b": "variant-b"},
                "targeting": {
                    "fractional": [
                        ["a", 90],
                        ["b", 10]
                    ]
                }
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");

    let mut a_count = 0;
    let mut b_count = 0;

    // Test with many users
    for i in 0..100 {
        let context = json!({"targetingKey": format!("test-user-{}", i)});
        let result = evaluator.evaluate_flag("heavyA", context);
        match result.variant.as_deref() {
            Some("a") => a_count += 1,
            Some("b") => b_count += 1,
            _ => panic!("Unexpected variant"),
        }
    }

    // With 100 users and 90/10 split, we expect roughly 90 "a" and 10 "b"
    // Allow some variance due to hash distribution
    assert!(
        a_count > 70,
        "Expected mostly 'a' variants, got {}",
        a_count
    );
    assert!(b_count > 0, "Expected some 'b' variants, got {}", b_count);
}

#[test]
fn test_unicode_flag_key() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    let config = r#"{
        "flags": {
            "日本語フラグ": {
                "state": "ENABLED",
                "defaultVariant": "オン",
                "variants": {"オン": true, "オフ": false}
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");
    let result = evaluator.evaluate_bool("日本語フラグ", json!({}));
    assert_eq!(result.value, json!(true));
    assert_eq!(result.variant, Some("オン".to_string()));
}

#[test]
fn test_unicode_in_context() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    let config = r#"{
        "flags": {
            "greetingFlag": {
                "state": "ENABLED",
                "defaultVariant": "hello",
                "variants": {"hello": "Hello", "nihao": "你好", "konnichiwa": "こんにちは"},
                "targeting": {
                    "if": [
                        {"==": [{"var": "language"}, "中文"]},
                        "nihao",
                        {"if": [
                            {"==": [{"var": "language"}, "日本語"]},
                            "konnichiwa",
                            "hello"
                        ]}
                    ]
                }
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");

    let context = json!({"language": "中文"});
    let result = evaluator.evaluate_flag("greetingFlag", context);
    assert_eq!(result.value, json!("你好"));

    let context = json!({"language": "日本語"});
    let result = evaluator.evaluate_flag("greetingFlag", context);
    assert_eq!(result.value, json!("こんにちは"));
}

#[test]
fn test_emoji_in_variant_values() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    let config = r#"{
        "flags": {
            "emojiFlag": {
                "state": "ENABLED",
                "defaultVariant": "happy",
                "variants": {"happy": "😀", "sad": "😢", "party": "🎉"}
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");
    let result = evaluator.evaluate_string("emojiFlag", json!({}));
    assert_eq!(result.value, json!("😀"));
}

#[test]
fn test_memory_large_allocation() {
    // Test allocation of a moderately large buffer
    let size = 1_000_000; // 1MB
    let ptr = alloc(size);
    assert!(!ptr.is_null(), "Should be able to allocate 1MB");
    dealloc(ptr, size);
}

#[test]
fn test_memory_consecutive_allocations() {
    // Test that consecutive allocations and deallocations work correctly
    for _ in 0..100 {
        let ptr = alloc(1024);
        assert!(!ptr.is_null());
        dealloc(ptr, 1024);
    }
}

#[test]
fn test_empty_variants_map() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Permissive);

    // Empty variants map - should use default handling
    let config = r#"{
        "flags": {
            "emptyVariants": {
                "state": "ENABLED",
                "defaultVariant": "default",
                "variants": {}
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");
    let result = evaluator.evaluate_flag("emptyVariants", json!({}));
    // Should return an error since variant doesn't exist
    assert!(result.error_code.is_some());
}

#[test]
fn test_deeply_nested_targeting() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    // Create deeply nested if/else targeting
    let config = r#"{
        "flags": {
            "nestedFlag": {
                "state": "ENABLED",
                "defaultVariant": "level0",
                "variants": {
                    "level0": 0,
                    "level1": 1,
                    "level2": 2,
                    "level3": 3,
                    "level4": 4,
                    "level5": 5
                },
                "targeting": {
                    "if": [
                        {">": [{"var": "level"}, 4]},
                        "level5",
                        {"if": [
                            {">": [{"var": "level"}, 3]},
                            "level4",
                            {"if": [
                                {">": [{"var": "level"}, 2]},
                                "level3",
                                {"if": [
                                    {">": [{"var": "level"}, 1]},
                                    "level2",
                                    {"if": [
                                        {">": [{"var": "level"}, 0]},
                                        "level1",
                                        "level0"
                                    ]}
                                ]}
                            ]}
                        ]}
                    ]
                }
            }
        }
    }"#;

    evaluator.update_state(config).expect("should be working");

    for level in 0..=5 {
        let context = json!({"level": level});
        let result = evaluator.evaluate_flag("nestedFlag", context);
        assert_eq!(
            result.value,
            json!(level),
            "Level {} should return {}",
            level,
            level
        );
    }
}

#[test]
fn test_flag_removal_and_readd() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    // Add flag
    let config1 = r#"{
        "flags": {
            "tempFlag": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            }
        }
    }"#;
    evaluator
        .update_state(config1)
        .expect("state should be updated");

    // Remove flag
    let config2 = r#"{"flags": {}}"#;
    let response = evaluator.update_state(config2).unwrap();
    let changed = response.changed_flags.unwrap();
    assert!(
        changed.contains(&"tempFlag".to_string()),
        "Removed flag should be in changed list"
    );

    // Re-add flag with same config
    let response = evaluator.update_state(config1).unwrap();
    let changed = response.changed_flags.unwrap();
    assert!(
        changed.contains(&"tempFlag".to_string()),
        "Re-added flag should be in changed list"
    );
}

#[test]
fn test_sem_ver_edge_cases() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    let config = r#"{
        "flags": {
            "versionFlag": {
                "state": "ENABLED",
                "defaultVariant": "old",
                "variants": {"old": "old-version", "new": "new-version"},
                "targeting": {
                    "if": [
                        {"sem_ver": [{"var": "appVersion"}, ">=", "2.0.0"]},
                        "new",
                        "old"
                    ]
                }
            }
        }
    }"#;

    evaluator
        .update_state(config)
        .expect("state should be updated");

    // Test various version formats
    let test_cases = vec![
        ("1.0.0", "old"),
        ("1.9.9", "old"),
        ("2.0.0", "new"),
        ("2.0.1", "new"),
        ("10.0.0", "new"),
        ("2.0.0-alpha", "old"), // Pre-release is less than release
    ];

    for (version, expected) in test_cases {
        let context = json!({"appVersion": version});
        let result = evaluator.evaluate_flag("versionFlag", context);
        assert_eq!(
            result.variant,
            Some(expected.to_string()),
            "Version {} should map to variant {}",
            version,
            expected
        );
    }
}

mod yaml_tests {
    use flagd_evaluator::{FlagEvaluator, ValidationMode};

    const SIMPLE_YAML: &str = r#"
flags:
  bool-flag:
    state: ENABLED
    variants:
      "on": true
      "off": false
    defaultVariant: "on"
  string-flag:
    state: ENABLED
    variants:
      v1: hello
      v2: world
    defaultVariant: v1
"#;

    const YAML_WITH_TARGETING: &str = r#"
flags:
  targeted-flag:
    state: ENABLED
    variants:
      "yes": true
      "no": false
    defaultVariant: "no"
    targeting:
      if:
        - ==:
            - var: targetingKey
            - admin
        - "yes"
        - "no"
"#;

    #[test]
    fn test_yaml_to_json_conversion() {
        let json = flagd_evaluator::yaml::yaml_to_json(SIMPLE_YAML).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["flags"]["bool-flag"].is_object());
    }

    #[test]
    fn test_yaml_invalid_syntax_returns_error() {
        let result = flagd_evaluator::yaml::yaml_to_json("flags:\n  bad: [unclosed");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse YAML"));
    }

    #[test]
    fn test_update_state_from_yaml_loads_flags() {
        let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
        evaluator.update_state_from_yaml(SIMPLE_YAML).unwrap();

        let ctx = serde_json::json!({"targetingKey": "user-1"});
        let result = evaluator.evaluate_flag("bool-flag", ctx);
        assert_eq!(result.value, serde_json::json!(true));
        assert_eq!(
            result.reason,
            flagd_evaluator::types::ResolutionReason::Static
        );
    }

    #[test]
    fn test_update_state_from_yaml_evaluates_string_flag() {
        let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
        evaluator.update_state_from_yaml(SIMPLE_YAML).unwrap();

        let ctx = serde_json::json!({"targetingKey": "user-1"});
        let result = evaluator.evaluate_flag("string-flag", ctx);
        assert_eq!(result.value, serde_json::json!("hello"));
    }

    #[test]
    fn test_update_state_from_yaml_with_targeting() {
        let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
        evaluator
            .update_state_from_yaml(YAML_WITH_TARGETING)
            .unwrap();

        let ctx = serde_json::json!({"targetingKey": "admin"});
        let result = evaluator.evaluate_flag("targeted-flag", ctx);
        assert_eq!(result.value, serde_json::json!(true));

        let ctx2 = serde_json::json!({"targetingKey": "regular-user"});
        let result2 = evaluator.evaluate_flag("targeted-flag", ctx2);
        assert_eq!(result2.value, serde_json::json!(false));
    }

    #[test]
    fn test_update_state_from_yaml_invalid_json_structure_returns_error() {
        // Valid YAML but not a valid flagd config (missing 'flags' key in strict mode)
        // update_state returns Ok(response) with success=false for schema validation failures
        let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);
        let result = evaluator
            .update_state_from_yaml("foo: bar\nbaz: 42\n")
            .unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_yaml_to_json_preserves_boolean_values() {
        let yaml = "flags:\n  f:\n    state: ENABLED\n    variants:\n      on: true\n      off: false\n    defaultVariant: on\n";
        let json = flagd_evaluator::yaml::yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed["flags"]["f"]["variants"]["on"],
            serde_json::json!(true)
        );
        assert_eq!(
            parsed["flags"]["f"]["variants"]["off"],
            serde_json::json!(false)
        );
    }

    #[test]
    fn test_yaml_to_json_preserves_numeric_values() {
        let yaml = "flags:\n  f:\n    state: ENABLED\n    variants:\n      low: 10\n      high: 100\n    defaultVariant: low\n";
        let json = flagd_evaluator::yaml::yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed["flags"]["f"]["variants"]["low"],
            serde_json::json!(10)
        );
    }
}

// ============================================================================
// Tests for flag-set metadata in UpdateStateResponse
// ============================================================================

#[test]
fn test_update_state_flag_set_metadata_present() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    let config = r#"{
        "metadata": {
            "flagSet": "my-flag-set",
            "version": "1.0.0",
            "environment": "production"
        },
        "flags": {
            "someFlag": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true, "off": false}
            }
        }
    }"#;

    let response = evaluator.update_state(config).unwrap();
    assert!(response.success);

    let metadata = response
        .flag_set_metadata
        .expect("flag_set_metadata should be present");
    assert_eq!(metadata.get("flagSet"), Some(&json!("my-flag-set")));
    assert_eq!(metadata.get("version"), Some(&json!("1.0.0")));
    assert_eq!(metadata.get("environment"), Some(&json!("production")));
}

#[test]
fn test_update_state_flag_set_metadata_absent() {
    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    let config = r#"{
        "flags": {
            "someFlag": {
                "state": "ENABLED",
                "defaultVariant": "on",
                "variants": {"on": true}
            }
        }
    }"#;

    let response = evaluator.update_state(config).unwrap();
    assert!(response.success);
    assert!(
        response.flag_set_metadata.is_none(),
        "flag_set_metadata should be None when no top-level metadata"
    );
}

#[test]
fn test_update_state_flag_set_metadata_replaces_on_second_call() {
    use serde_json::json;

    let mut evaluator = FlagEvaluator::new(ValidationMode::Strict);

    let config1 = r#"{
        "metadata": { "env": "staging" },
        "flags": {
            "f": { "state": "ENABLED", "defaultVariant": "on", "variants": {"on": true} }
        }
    }"#;

    let config2 = r#"{
        "metadata": { "env": "production", "owner": "team-a" },
        "flags": {
            "f": { "state": "ENABLED", "defaultVariant": "on", "variants": {"on": true} }
        }
    }"#;

    let r1 = evaluator.update_state(config1).unwrap();
    assert_eq!(
        r1.flag_set_metadata.as_ref().unwrap().get("env"),
        Some(&json!("staging"))
    );

    let r2 = evaluator.update_state(config2).unwrap();
    let m2 = r2.flag_set_metadata.unwrap();
    assert_eq!(m2.get("env"), Some(&json!("production")));
    assert_eq!(m2.get("owner"), Some(&json!("team-a")));
}

// ============================================================================
// Nested fractional argument evaluation (flagd#1676)
// ============================================================================

/// Helper: create a strict evaluator and load config, panicking on failure.
fn strict_eval_with(config: &str) -> FlagEvaluator {
    let mut eval = FlagEvaluator::new(ValidationMode::Strict);
    let resp = eval.update_state(config).unwrap();
    assert!(resp.success, "update_state failed: {:?}", resp.error);
    eval
}

#[test]
fn test_nested_if_in_bucket_variant_name() {
    use serde_json::json;
    // Variant name is a {"if": ...} expression — same email maps to the same bucket,
    // but the resolved variant depends on the locale context field.
    let config = json!({
        "flags": {
            "color-flag": {
                "state": "ENABLED",
                "variants": {"red": "#FF0000", "grey": "#808080", "blue": "#0000FF"},
                "defaultVariant": "grey",
                "targeting": {
                    "fractional": [
                        {"var": "email"},
                        [{"if": [{"in": [{"var": "locale"}, ["us", "ca"]]}, "red", "grey"]}, 50],
                        ["blue", 50]
                    ]
                }
            }
        }
    });
    let eval = strict_eval_with(&config.to_string());

    // jon@company.com hashes into the first bucket (verified by gherkin v2 test)
    let ctx_us = json!({"email": "jon@company.com", "locale": "us"});
    let ctx_de = json!({"email": "jon@company.com", "locale": "de"});

    let res_us = eval.evaluate_flag("color-flag", ctx_us);
    let res_de = eval.evaluate_flag("color-flag", ctx_de);

    // Same email → same bucket, but different locale → different resolved variant
    assert_eq!(
        res_us.variant.as_deref(),
        Some("red"),
        "US locale should resolve to 'red'"
    );
    assert_eq!(
        res_de.variant.as_deref(),
        Some("grey"),
        "DE locale should resolve to 'grey'"
    );
}

#[test]
fn test_nested_fractional_inside_fractional() {
    use serde_json::json;
    // Nested fractional: outer splits on email 50/50, inner splits on tier 50/50.
    let config = json!({
        "flags": {
            "experiment": {
                "state": "ENABLED",
                "variants": {"red": 1, "blue": 2, "green": 3, "yellow": 4},
                "defaultVariant": "red",
                "targeting": {
                    "fractional": [
                        {"var": "email"},
                        [{"fractional": [{"var": "tier"}, ["red", 50], ["blue", 50]]}, 50],
                        [{"fractional": [{"var": "tier"}, ["green", 50], ["yellow", 50]]}, 50]
                    ]
                }
            }
        }
    });
    let eval = strict_eval_with(&config.to_string());

    // All combinations should resolve to one of the four valid variants
    for email in ["a@x.com", "b@x.com", "c@x.com", "d@x.com", "e@x.com"] {
        for tier in ["free", "pro", "enterprise"] {
            let ctx = json!({"email": email, "tier": tier});
            let res = eval.evaluate_flag("experiment", ctx);
            assert!(
                ["red", "blue", "green", "yellow"].contains(&res.variant.as_deref().unwrap_or("")),
                "email={email} tier={tier} → unexpected variant {:?}",
                res.variant
            );
        }
    }
}

#[test]
fn test_nested_var_in_bucket_variant_name() {
    use serde_json::json;
    // Variant name is a {"var": "preferred_color"} — each user carries their own preference.
    let config = json!({
        "flags": {
            "theme-flag": {
                "state": "ENABLED",
                "variants": {"dark": "dark-theme", "light": "light-theme"},
                "defaultVariant": "light",
                "targeting": {
                    "fractional": [
                        {"var": "email"},
                        [{"var": "preferred_theme"}, 50],
                        ["light", 50]
                    ]
                }
            }
        }
    });
    let eval = strict_eval_with(&config.to_string());

    // jon@company.com hashes into first bucket; preferred_theme drives the variant
    let ctx_dark = json!({"email": "jon@company.com", "preferred_theme": "dark"});
    let ctx_light = json!({"email": "jon@company.com", "preferred_theme": "light"});

    let res_dark = eval.evaluate_flag("theme-flag", ctx_dark);
    let res_light = eval.evaluate_flag("theme-flag", ctx_light);

    assert_eq!(res_dark.variant.as_deref(), Some("dark"));
    assert_eq!(res_light.variant.as_deref(), Some("light"));
}

// ============================================================================
// Statistical distribution test (mirrors Java PR #1740 statistics() test)
// ============================================================================

#[test]
fn test_fractional_distribution_uniformity() {
    use flagd_evaluator::operators::fractional::fractional;
    use serde_json::json;

    // 16 equal-weight buckets totalling i32::MAX (matches Java test exactly)
    let total_weight: u64 = i32::MAX as u64;
    let buckets_count: u64 = 16;
    let weight = total_weight / buckets_count;
    let remainder = total_weight - weight * (buckets_count - 1);

    let mut bucket_defs: Vec<serde_json::Value> = Vec::new();
    for i in 0..buckets_count - 1 {
        bucket_defs.push(json!(format!("{}", i)));
        bucket_defs.push(serde_json::Value::Number(weight.into()));
    }
    bucket_defs.push(json!(format!("{}", buckets_count - 1)));
    bucket_defs.push(serde_json::Value::Number(remainder.into()));

    let mut hits = vec![0u64; buckets_count as usize];

    // 100k sequential keys — dense sample, fast, and tight enough for 10% tolerance.
    for i in 0u64..100_000 {
        let key = format!("{}", i);
        let bucket_str = fractional(&key, &bucket_defs).unwrap();
        let bucket_idx: usize = bucket_str.parse().unwrap();
        hits[bucket_idx] += 1;
    }

    let min = *hits.iter().min().unwrap();
    let max = *hits.iter().max().unwrap();
    let delta = max - min;

    let sample_size: u64 = 100_000;
    let expected_per_bucket = sample_size / buckets_count;
    let tolerance = expected_per_bucket / 10; // 10% tolerance

    assert!(
        delta < tolerance,
        "Distribution imbalance too large: max={max} min={min} delta={delta} (tolerance={tolerance}). hits={hits:?}"
    );
}

#[test]
fn test_fractional_boundary_hashes_do_not_panic() {
    use flagd_evaluator::operators::fractional::fractional;
    use serde_json::json;

    let buckets = vec![
        json!("a"),
        json!(4),
        json!("b"),
        json!(4),
        json!("c"),
        json!(4),
        json!("d"),
        json!(4),
    ];

    // Keys chosen to produce boundary-adjacent hash values
    for key in ["", "0", "a", "\0", "ffffffff"] {
        let result = fractional(key, &buckets);
        assert!(result.is_ok(), "key={key:?} should not error: {:?}", result);
        assert!(
            ["a", "b", "c", "d"].contains(&result.unwrap().as_str()),
            "key={key:?} must map to a valid bucket"
        );
    }
}
