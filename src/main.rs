use actix_cors::Cors;
use actix_web::{delete, http, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use chat_users::{
    create_chat_user::create_chat_user,
    delete_chat_user::delete_chat_user,
    read_chat_user::{check_for_users_password, read_all_chat_user},
    update_chat_user::update_chat_user,
};
use models::{ChatUsers, LoginChatUsers, MessageResponse, UpdateUserPassword};

pub mod chat_connectivity;
pub mod chat_users;
pub mod models;
pub mod schema;

#[post("/sign_up_user")]
pub async fn sign_up_user(data: Json<ChatUsers>) -> impl Responder {
    let data_to_create = ChatUsers {
        username: data.username.clone().to_uppercase(),
        userpassword: data.userpassword.clone(),
        email: data.email.clone().to_uppercase(),
    };
    let created_result = create_chat_user(data_to_create);
    match created_result {
        Ok(created_data) => {
            println!("Created: {:?} successfully", created_data.username);
            let return_message = MessageResponse {
                message: format!("true"),
            };
            HttpResponse::Ok().json(return_message)
        }
        Err(e) => {
            let return_message = MessageResponse {
                message: e.to_string(),
            };
            HttpResponse::Ok().json(return_message)
        }
    }
}

#[post("/login_user")]
pub async fn login_user(data: Json<LoginChatUsers>) -> impl Responder {
    let login_result = check_for_users_password(data.username.clone(), data.userpassword.clone());
    match login_result {
        true => {
            let return_message = MessageResponse {
                message: String::from("true"),
            };
            HttpResponse::Ok().json(return_message)
        }
        false => {
            let return_message = MessageResponse {
                message: String::from("false"),
            };
            HttpResponse::Ok().json(return_message)
        }
    }
}

#[patch("/update_user_password")]
pub async fn update_user_password(data: Json<UpdateUserPassword>) -> impl Responder {
    let field = String::from("userpassword");
    let islogged = check_for_users_password(
        data.username.to_uppercase().clone(),
        data.current_password.clone(),
    );
    if islogged {
        let udpated_result =
            update_chat_user(data.username.clone(), data.new_password.clone(), field);
        match udpated_result {
            Some(Ok(updated_data)) => HttpResponse::Ok().body(format!(
                "user {} password is updated successfully",
                updated_data.username
            )),
            Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
            None => HttpResponse::Ok().body("Please enter a valid field, =>  userpassword "),
        }
    } else {
        HttpResponse::Ok().body("Incorrect current password")
    }
}

#[delete("/delete_user")]
pub async fn delete_user(data: Json<UpdateUserPassword>) -> impl Responder {
    let deleted_result = delete_chat_user(data.username.to_uppercase().clone());
    match deleted_result {
        Ok(deleted_data) => {
            HttpResponse::Ok().body(format!("Deleted: {:?} successfully", deleted_data.username))
        }
        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let all_result = read_all_chat_user();
    match all_result {
        Ok(result) => {
            println!("{result:?}");
        }
        Err(e) => {
            println!("{e:?}")
        }
    }
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
                    .allowed_origin("http://192.168.137.210:10000")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(sign_up_user)
            .service(login_user)
            .service(update_user_password)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
