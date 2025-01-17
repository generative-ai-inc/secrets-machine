use sm::library::commands::{execute, run};
mod common;

async fn run_test(test_name: &str) {
    let (config, mocked_keyring_env_vars_map) = common::setup(true).await.unwrap();

    let _ = run(&config, test_name, "", Some(mocked_keyring_env_vars_map)).await;
}

async fn execute_test(echo_value: &str, test_name: &str) {
    let (config, mocked_keyring_env_vars_map) = common::setup(true).await.unwrap();

    let _ = execute(
        &config,
        &format!("echo {echo_value} > tests/test_results/{test_name}.txt"),
        Some(mocked_keyring_env_vars_map),
    )
    .await;
}

#[tokio::test]
async fn test_run_bw_client_simple() {
    run_test("test_run_bw_client_simple").await;
    common::assert_text_result("test_run_bw_client_simple", "Client ID: abc-123")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_exec_bw_secret_simple() {
    let test_name = "test_exec_bw_secrets_simple";
    execute_test("Client Secret: $CLIENT_SECRET", test_name).await;
    common::assert_text_result(test_name, "Client Secret: secret-123")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_exec_bw_alias_simple() {
    let test_name = "test_exec_bw_alias_simple";
    execute_test("Client ID: $NEXT_PUBLIC_CLIENT_ID", test_name).await;
    common::assert_text_result(test_name, "Client ID: abc-123")
        .await
        .unwrap();
}
