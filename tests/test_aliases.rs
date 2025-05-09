use sm::library::commands::execute;

mod common;

async fn execute_test(echo_value: &str, test_name: &str) {
    let (config, mocked_keyring_env_vars_map) = common::setup(false).await.unwrap();

    let _ = execute(
        &config,
        &format!("echo {echo_value} > tests/test_results/{test_name}.txt"),
        Some(mocked_keyring_env_vars_map),
    )
    .await;
}

#[tokio::test]
async fn test_aliases_simple() {
    let test_name = "test_aliases_simple";
    execute_test("$PETER $PARKER is awesome", test_name).await;
    common::assert_text_result(test_name, "El Hombre Araña is awesome")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_aliases_single_quotes() {
    let test_name = "test_aliases_single_quotes";
    execute_test("'$PETER $PARKER' is awesome", test_name).await;
    common::assert_text_result(test_name, "$PETER $PARKER is awesome")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_aliases_double_quotes() {
    let test_name = "test_aliases_double_quotes";
    execute_test("\"$PETER $PARKER\" is awesome", test_name).await;
    common::assert_text_result(test_name, "El Hombre Araña is awesome")
        .await
        .unwrap();
}
