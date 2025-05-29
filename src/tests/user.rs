use anyhow::Result;

use super::make_test_app;

mod create {
    use super::*;

    #[tokio::test]
    async fn new_user() -> Result<()> {
        let app = make_test_app().await.expect("Failed to create test app");

        Ok(())
    }
}
