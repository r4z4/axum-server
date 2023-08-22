#![allow(unused)] // For beginning only

use anyhow::Result;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use axum::extract::Path;


#[tokio::test]
async fn create_eligible_case() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc
        .do_post(
            "/create_eligible_case",
            json!({
                "provider_id": 1,
                "patient_id": 1,
                "iro_id": 1,
                "insurer_id": 1,
                "denial_reason": "Not Covered by Formulary",
                "expedited": 1,
                "date_forwarded": "2023-10-10",
            }),
        )
        .await?;

    res.print().await?;

    Ok(())
}