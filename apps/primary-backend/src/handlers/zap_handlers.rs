// use axum::{
//     extract::{Path, State},
//     http::StatusCode,
//     response::Json,
// };
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
//     pub struct Zap{
//         pub id: String,
//         pub trigger_id: String,
//     }

// pub async fn create_zap() {
//     println!("I'am user signup handler!");
// }

// pub async fn get_zap() {
//     println!("I'am user login handler!");
// }

// pub async fn get_zap_by_id(
//     Path(zap_id): Path<String>,
// )  -> Result<Json<Zap>, StatusCode>{
//     println!("I'am user get zap by id handler!");
//     if zap_id == "123" {
//         let zap = Zap {
//             id: zap_id,
//             trigger_id: "Hello from Zap 123!".to_string(),
//         };
//         Ok(Json(zap))
//     } else {
//         // Return a 404 Not Found if the ID doesn't exist
//         Err(StatusCode::NOT_FOUND)
//     }

// }
