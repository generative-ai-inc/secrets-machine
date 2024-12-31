use sm::library::utils::logging;

mod common;

#[tokio::test]
async fn test_teardown() {
    logging::info("Tearing down test environment").await;
    common::teardown().await;
}
