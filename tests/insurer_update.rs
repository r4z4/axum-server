// #![allow(unused)] // For beginning only

// use anyhow::Result;
// use serde::{Deserialize};
// use serde::de::DeserializeOwned;
// use serde_json::{json, Value};
// use axum::extract::Path;

// // Something is wrong with the do_put method (I am pretty sure). Even when you run it with an invalud path (e.g. "/insurer") it still passes
// // Same info gets created on a do_post to the create_insurer route too, which makes me think ^


// #[tokio::test]
// async fn insurer_update() -> Result<()> {
//     let hc = httpc_test::new_client("http://localhost:3000")?;

//     let res = hc
//         .do_put(
//             "/insurer",
//             json!({
//                 "insurer_name": "Test Insurer Updated Name",
//                 "insurer_phone": "555-333-4444",
//                 "insurer_email": "test_updated@insurer_updated.com",
//                 "insurer_zip": "68144",
//                 "insurer_address_1": "123 Updated St..",
//                 "insurer_address_2": "# 9B",
//                 "insurer_contact_f_name": "Jacquline",
//                 "insurer_contact_l_name": "Stillcover"
//             }),
//         )
//         .await?;

//     res.print().await?;

//     Ok(())
// }



