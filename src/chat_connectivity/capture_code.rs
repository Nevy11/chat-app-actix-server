// #![allow(dead_code, unused_imports, unused_variables)]
// use std::{
//     convert::Infallible,
//     sync::{Arc, Mutex},
// };

// use actix_web::dev::Server;
// use hyper::{server, service::service_fn, Request, Response};
// use oauth2::AuthorizationCode;
// use reqwest::Body;
// use tokio::sync::oneshot;

// /// This function runs the local server and captures the authorization code.
// pub async fn capture_code(port: u16) {
//     let (tx, rx) = oneshot::channel::<AuthorizationCode>();
//     // Create a shared state to pass the channel sender to the handler
//     let state = Arc::new(Mutex::new(Some(tx)));

//     // Create the local server
//     let make_svc = service_fn(move |req: Request<Body>| {
//         let state = Arc::clone(&state);
//         async move {
//             // Extract the authorization code from the query parameters
//             if let Some(query) = req.uri().query() {
//                 if let Some(code) = query.split("&").find_map(|s| {
//                     if s.starts_with("code=") {
//                         Some(s.trim_start_matches("code=").to_string())
//                     } else {
//                         None
//                     }
//                 }) {
//                     // send the authorization code through the channel
//                     if let Ok(mut tx) = state.lock() {
//                         if let Some(sender) = tx.take() {
//                             let _ = sender.send(AuthorizationCode::new(code));
//                         }
//                     }

//                     // respond to the browser with a message
//                     return Ok::<_, Infallible>(Response::new(Body::from(
//                         "Authorization code received. you can close this tab",
//                     )));
//                 }
//             }
//             Ok::<_, Infallible>(Response::new(Body::from("No authoirzation code found.")))
//         }
//     });
// }
