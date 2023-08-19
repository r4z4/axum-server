#![allow(unsused)] // For beginning only

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/").await?.print().await?;

    Ok(())
}