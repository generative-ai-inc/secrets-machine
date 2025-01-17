use sm::library::commands::run;

mod common;

async fn run_test(test_name: &str) {
    let (config, mocked_keyring_env_vars_map) = common::setup(false).await.unwrap();

    let _ = run(&config, test_name, "", Some(mocked_keyring_env_vars_map)).await;
}

#[tokio::test]
async fn test_run_simple() {
    let test_name = "test_run_simple";
    run_test(test_name).await;
    common::assert_text_result(test_name, "hello beautiful world")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_single_quotes() {
    let test_name = "test_run_single_quotes";
    run_test(test_name).await;
    common::assert_text_result(test_name, "hello $TEST_ENV_VAR $HELLO")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_double_quotes() {
    let test_name = "test_run_double_quotes";
    run_test(test_name).await;
    common::assert_text_result(test_name, "hello beautiful world")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_simple_braces() {
    let test_name = "test_run_simple_braces";
    run_test(test_name).await;
    common::assert_text_result(test_name, "hello beautiful world")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_single_quotes_braces() {
    let test_name = "test_run_single_quotes_braces";
    run_test(test_name).await;
    common::assert_text_result(test_name, "hello ${TEST_ENV_VAR} ${HELLO}")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_double_quotes_braces() {
    let test_name = "test_run_double_quotes_braces";
    run_test(test_name).await;
    common::assert_text_result(test_name, "hello beautiful world")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_pre_commands_simple() {
    let test_name = "test_run_pre_commands_simple";
    run_test(test_name).await;
    common::assert_text_result(test_name, "My name is\nPeter Parker")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_pre_commands_single_quotes() {
    let test_name = "test_run_pre_commands_single_quotes";
    run_test(test_name).await;
    common::assert_text_result(test_name, "My name is\n$SPIDER $MAN")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_pre_commands_double_quotes() {
    let test_name = "test_run_pre_commands_double_quotes";
    run_test(test_name).await;
    common::assert_text_result(test_name, "My name is\nPeter Parker")
        .await
        .unwrap();
}
