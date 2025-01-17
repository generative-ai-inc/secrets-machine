use sm::library::commands::execute;

mod common;

async fn execute_test(echo_value: &str, test_name: &str) {
    let (config, secrets) = common::setup().await.unwrap();

    let _ = execute(
        &config,
        &secrets,
        &format!("echo {echo_value} > tests/test_results/{test_name}.txt"),
    )
    .await;
}

#[tokio::test]
async fn test_execution_simple() {
    let test_name = "test_execution_simple";
    execute_test("hello $TEST_ENV_VAR $HELLO", test_name).await;
    common::assert_text_result(test_name, "hello beautiful world")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_execution_single_quotes() {
    let test_name = "test_execution_single_quotes";
    execute_test("'Hello $TEST_ENV_VAR $HELLO'", test_name).await;
    common::assert_text_result(test_name, "Hello $TEST_ENV_VAR $HELLO")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_execution_double_quotes() {
    let test_name = "test_execution_double_quotes";
    execute_test("\"Hello $TEST_ENV_VAR $HELLO\"", test_name).await;
    common::assert_text_result(test_name, "Hello beautiful world")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_execution_simple_braces() {
    let test_name = "test_execution_simple_braces";
    execute_test("hello ${TEST_ENV_VAR} ${HELLO}", test_name).await;
    common::assert_text_result(test_name, "hello beautiful world")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_execution_single_quotes_braces() {
    let test_name = "test_execution_single_quotes_braces";
    execute_test("'Hello ${TEST_ENV_VAR} ${HELLO}'", test_name).await;
    common::assert_text_result(test_name, "Hello ${TEST_ENV_VAR} ${HELLO}")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_execution_double_quotes_braces() {
    let test_name = "test_execution_double_quotes_braces";
    execute_test("\"Hello ${TEST_ENV_VAR} ${HELLO}\"", test_name).await;
    common::assert_text_result(test_name, "Hello beautiful world")
        .await
        .unwrap();
}
