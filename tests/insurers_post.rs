#![allow(unused)] // For beginning only

use anyhow::Result;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use axum::extract::Path;


#[tokio::test]
async fn insurer_post() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc
        .do_post(
            "/create_insurer",
            json!({
                "insurer_name": "BCBS",
                "insurer_phone": "402-555-5555",
                "insurer_phone": "main@bcbs.com"
            }),
        )
        .await?;

    res.print().await?;

    Ok(())
}



