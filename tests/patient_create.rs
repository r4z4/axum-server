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
            "/create_patient",
            json!({
                "patient_f_name": "Mary",
                "patient_l_name": "Patient",
                "patient_email": "555-333-4444",
                "patient_zip": "55057",
                "patient_address_1": "9999 Patient Avenue",
                "patient_address_2": "Apt. #4",
                "patient_dob": "1975-11-11",
            }),
        )
        .await?;

    res.print().await?;

    Ok(())
}