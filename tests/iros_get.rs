#![allow(unused)] // For beginning only

use anyhow::Result;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use axum::extract::Path;
use sea_orm::prelude::Date;

#[tokio::test]
async fn iros_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/get_iros").await?.print().await?;

    Ok(())
}

//
// These Require DB Records <------
//

#[tokio::test]
async fn iro_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/get_iro/1").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn iro_get_plus() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let res = hc.do_get("/get_iro/1").await?;
    let status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    let auth_token = res.res_cookie_value("auth-token"); // Option<String>
    let content_type = res.header("content_type"); // Option<&String>

    res.print().await?;

    Ok(())
}

#[derive(Deserialize)]
struct Iro {
    iro_id: i32,
    iro_name: String,
    iro_email: Option<String>,
    iro_address_1: Option<String>,
    iro_address_2: Option<String>,
    iro_zip: Option<String>,
    iro_contact_f_name: Option<String>,
    iro_contact_l_name: Option<String>,
    iro_license_expiration: Option<Date>,
}

// // // These assume rather stable records in DB. Disble for now. Re-enable when better data

// #[tokio::test]
// async fn iro_get_plus_params() -> Result<()> {
//     let hc = httpc_test::new_client("http://localhost:3000")?;

//     let res = hc.get::<Iro>("/get_iro/1").await?;
//     assert_eq!(res.iro_name, "Maximus Federal Services");

//     Ok(())
// }

// #[tokio::test]
// async fn iro_get_plus_params_id() -> Result<()> {
//     let hc = httpc_test::new_client("http://localhost:3000")?;

//     let res = hc.get::<Iro>("/get_iro/1").await?;
//     assert_eq!(res.iro_id, 1);

//     Ok(())
// }



