#![allow(unused)] // For beginning only

use anyhow::Result;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use axum::extract::Path;

#[tokio::test]
async fn patients_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/get_patients").await?.print().await?;

    Ok(())
}

//
// These Require DB Records <------
//

#[tokio::test]
async fn patient_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/get_patient/1").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn patient_get_plus() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.do_get("/get_patient/1").await?;
    let status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    let auth_token = res.res_cookie_value("auth-token"); // Option<String>
    let content_type = res.header("content_type"); // Option<&String>

    res.print().await?;

    Ok(())
}

#[derive(Deserialize)]
struct Patient {
    patient_id: i32,
    patient_f_name: String,
    patient_l_name: String,
    patient_email: Option<String>
}

#[tokio::test]
async fn patient_get_plus_params() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.get::<Patient>("/get_patient/1").await?;
    assert_eq!(res.patient_f_name, "Robert");
    // assert_eq!(res.json_body_as::<Provider>().provider_name, "Main Provider");

    Ok(())
}

#[tokio::test]
async fn patient_get_plus_params_id() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.get::<Patient>("/get_patient/1").await?;
    assert_eq!(res.patient_id, 1);
    // assert_eq!(res.json_body_as::<Provider>().provider_name, "Main Provider");

    Ok(())
}



