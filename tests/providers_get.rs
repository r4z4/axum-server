#![allow(unused)] // For beginning only

use anyhow::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use axum::extract::Path;

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

#[tokio::test]
async fn provider_get_plus() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.do_get("/get_provider/1").await?;
    let status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    let auth_token = res.res_cookie_value("auth-token"); // Option<String>
    let content_type = res.header("content_type"); // Option<&String>

    res.print().await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::test]
async fn route_params_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.do_get("/path_var/531").await?;
    assert_eq!(res.text_body().expect("531").parse::<i32>().unwrap(), 531);

    Ok(())
}



