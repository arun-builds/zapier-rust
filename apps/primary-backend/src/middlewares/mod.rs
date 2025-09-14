// use axum::{
//     extract::{FromRequest, Request},
//     middleware::{self, Next},
//     response::{IntoResponse, Response},
// };


// pub async fn auth_middleware(request: Request, next: Next) -> Result<impl IntoResponse, Response> {
//     // let request = buffer_request_body(request).await?;
//     println!("Middleware: logged");
//     // let request = buffer_request_body(request).await?;

//     Ok(next.run(request).await)
// }