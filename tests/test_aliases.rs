use sm::library::commands::execute;

mod common;

async fn execute_test(echo_value: &str, test_name: &str) {
    let (project_config, config, secrets) = common::setup().await.unwrap();

    let _ = execute(
        project_config,
        config,
        secrets,
        &format!("echo {echo_value} > tests/test_results/{test_name}.txt"),
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
