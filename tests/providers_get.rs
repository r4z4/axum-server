#![allow(unsused)] // For beginning only

use anyhow::Result;

#[tokio::test]
async fn providers_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/get_providers").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn provider_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/get_provider/2").await?.print().await?;

    Ok(())
}