// pub async fn auth_2() {
//     dotenv().ok();
//     let client_id = ClientId::new(env::var("CLIENT_ID").expect("CLIENT_ID MUST BE SET"));
//     let client_secret =
//         ClientSecret::new(env::var("CLIENT_SECRET").expect("CLIENT_SECRET MUST BE SET"));
//     let auth_url = AuthUrl::new(String::from("https://accounts.google.com/o/oauth2/auth")).unwrap();
//     let token_url = TokenUrl::new(String::from("https://oauth2.googleapis.com/token")).unwrap();
//     let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
//         .set_redirect_uri(RedirectUrl::new("http://localhost:4200".to_string()).unwrap());
//     let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
//     let (auth_url, _csrf_token) = client
//         .authorize_url(CsrfToken::new_random)
//         .add_scope(Scope::new(String::from(
//             "https://www.googleapis.com/auth/gmail.send",
//         )))
//         .set_pkce_challenge(pkce_challenge)
//         .url();
//     println!("Browse to: {}", auth_url);
//     println!("Enter the authorization code:");
//     let mut auth_code = String::new();
//     io::stdin().read_line(&mut auth_code).unwrap();
//     let auth_code = auth_code.trim_end().to_string();
//     let token: oauth2::StandardTokenResponse<
//         oauth2::EmptyExtraTokenFields,
//         oauth2::basic::BasicTokenType,
//     > = client
//         .exchange_code(AuthorizationCode::new(auth_code))
//         .set_pkce_verifier(pkce_verifier)
//         .request_async(oauth2::reqwest::async_http_client)
//         .await
//         .unwrap();
//     println!("Access token: {:?}", token.access_token().secret());
//     // Using acceess token for api request
//     let access_token = token.access_token().secret().as_str();
//     let client = Client::new();
//     let response = client
//         .post("https://gmail.googleapis.com/gmail/v1/users/me/messages/send")
//         .bearer_auth(access_token)
//         .json(&json!({
//             "raw": "your-base64-encoded-message"
//         }))
//         .send()
//         .await;

//     match response {
//         Ok(resp) if resp.status().is_success() => println!("Email sent successfully"),
//         Ok(resp) => println!("Failed to send email: {:?}", resp.text().await.unwrap()),
//         Err(e) => println!("Request failed: {:?}", e),
//     }
// }

// pub async fn capture_code(req: HttpRequest) -> impl Responder {
//     if let Some(query) = req.uri().query() {
//         // Parse the authorization code from query parameters
//         // Assume you extract the code as a string
//         let auth_code = query; // Simplified; in real life, you will parse it
//         let authorization_code = AuthorizationCode::new(auth_code.to_string());
//         return HttpResponse::Ok().body(format!(
//             "Authorization code received: {}",
//             authorization_code.secret()
//         ));
//     }
//     HttpResponse::BadRequest().body("Authorization code missing.")
// }
