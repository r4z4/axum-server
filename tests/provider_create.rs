#![allow(unused)] // For beginning only

use anyhow::Result;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use axum::extract::Path;


#[tokio::test]
async fn provider_create() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc
        .do_post(
            "/create_provider",
            json!({
                "provider_name": "Test Office",
                "provider_phone": "402-333-4444",
                "provider_zip": "68144",
                "provider_address_1": "123 Test Ave.",
                "provider_address_2": "Test Ste.",
                "provider_contact_f_name": "Ralph",
                "provider_contact_l_name": "Testerman",
            }),
        )
        .await?;

    res.print().await?;

    Ok(())
}