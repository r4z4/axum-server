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
            "/create_iro",
            json!({
                "iro_name": "Test IRO",
                "iro_phone": "402-333-4444",
                "iro_email": "test@iro.com",
                "iro_zip": "68144",
                "iro_address_1": "123 Test Iro St.",
                "iro_address_2": "Ste. 333",
                "iro_contact_f_name": "Becca",
                "iro_contact_l_name": "Testerwoman",
                "iro_license_expiration": "2024-12-12",
            }),
        )
        .await?;

    res.print().await?;

    Ok(())
}