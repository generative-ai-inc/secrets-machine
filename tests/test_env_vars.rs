use std::{collections::BTreeMap, env};

use sm::library::utils::env_vars;

#[tokio::test]
async fn test_replace_env_vars() {
    unsafe {
        env::set_var("TEST_ENV_VAR", "beautiful");
        env::set_var("TEST_ENV_VAR_2", "world");
    }

    let mut env_vars_map = BTreeMap::new();
    env_vars_map.insert(
        "TEST_ENV_VAR".to_string(),
        ("beautiful".to_string(), "Test".to_string()),
    );
    env_vars_map.insert(
        "TEST_ENV_VAR_2".to_string(),
        ("world".to_string(), "Test".to_string()),
    );

    let test_string = "Hi there $TEST_ENV_VAR $TEST_ENV_VAR_2";

    let test_string_with_values = env_vars::replace(&env_vars_map, test_string, false).await;

    assert_eq!(test_string_with_values, "Hi there beautiful world");
}

#[tokio::test]
async fn test_replace_env_vars_brackets() {
    let mut env_vars_map = BTreeMap::new();
    env_vars_map.insert(
        "TEST_ENV_VAR".to_string(),
        ("beautiful".to_string(), "Test".to_string()),
    );
    env_vars_map.insert(
        "TEST_ENV_VAR_2".to_string(),
        ("world".to_string(), "Test".to_string()),
    );
    let test_string = "Hi there ${TEST_ENV_VAR} ${TEST_ENV_VAR_2}";

    let test_string_with_values = env_vars::replace(&env_vars_map, test_string, false).await;

    assert_eq!(test_string_with_values, "Hi there beautiful world");
}
