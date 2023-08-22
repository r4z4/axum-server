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
            "/create_insurer",
            json!({
                "insurer_name": "Test Insurer",
                "insurer_phone": "402-333-4444",
                "insurer_email": "test@insurer.com",
                "insurer_zip": "68144",
                "insurer_address_1": "123 Insurance St..",
                "insurer_address_2": "# 12A",
                "insurer_contact_f_name": "Jackie",
                "insurer_contact_l_name": "Icover",
            }),
        )
        .await?;

    res.print().await?;

    Ok(())
}