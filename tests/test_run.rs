use sm::library::commands::run;

mod common;

async fn run_test(test_name: &str) {
    let (commands_config, config, secrets) = common::setup().await.unwrap();

    let _ = run(
        commands_config,
        config,
        secrets,
        test_name.to_string(),
        String::new(),
    )
    .await;
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
