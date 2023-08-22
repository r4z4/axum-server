#![allow(unused)] // For beginning only

use anyhow::Result;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use axum::extract::Path;

#[tokio::test]
async fn insurers_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/get_insurers").await?.print().await?;

    Ok(())
}

//
// These Require DB Records <------
//

#[tokio::test]
async fn insurer_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/get_insurer/1").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn insurer_get_plus() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.do_get("/get_insurer/1").await?;
    let status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    let auth_token = res.res_cookie_value("auth-token"); // Option<String>
    let content_type = res.header("content_type"); // Option<&String>

    res.print().await?;

    Ok(())
}

#[derive(Deserialize)]
struct Insurer {
    insurer_id: i32,
    insurer_name: String,
    insurer_phone: Option<String>,
    insurer_email: Option<String>,
    insurer_address_1: Option<String>,
    insurer_address_2: Option<String>,
    insurer_zip: Option<String>,
    insurer_contact_f_name: Option<String>,
    insurer_contact_l_name: Option<String>,
}

#[tokio::test]
async fn insurer_get_plus_params() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.get::<Insurer>("/get_insurer/1").await?;
    assert_eq!(res.insurer_name, "BCBS");

    Ok(())
}

#[tokio::test]
async fn insurer_get_plus_params_id() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.get::<Insurer>("/get_insurer/1").await?;
    assert_eq!(res.insurer_id, 1);

    Ok(())
}



