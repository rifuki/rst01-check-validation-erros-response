use actix_web::{web, Responder, HttpResponse};
use sqlx::MySqlPool;
use validator::Validate;
use crate::{users::model::{
    User,
    CreateUser
}, validation_errors_response};

pub async fn get_users(pool: web::Data<MySqlPool>) -> impl Responder {
    let query = sqlx::query_as!(User, "SELECT username, password FROM users")
        .fetch_all(pool.get_ref())
        .await;

    match query {
        Ok(users) => {
            if users.len() > 0 {
                HttpResponse::Ok()
                    .json(serde_json::json!({
                        "msg": "success",
                        "data": users 
                    }))
            } else {
                HttpResponse::NoContent()
                    .json(serde_json::json!({
                        "status": "success",
                        "data": "Users is empty"
                    }))
            }
        }
        Err(error) => HttpResponse::InternalServerError().json(
            serde_json::json!({
                "status": "failed",
                "details": error.to_string() 
            })
        )
    }
}

pub async fn get_user(path: web::Path<String>, pool: web::Data<MySqlPool>) -> impl Responder {
    let username = path.into_inner();

    let query = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = ?",
        &username
    )
        .fetch_optional(pool.get_ref())
        .await;

    match query {
        Ok(ok) => match ok {
            Some(user) => {
                HttpResponse::Ok()
                    .json(serde_json::json!({
                        "status": "success",
                        "data": user
                    }))
            }
            None => {
                HttpResponse::Ok()
                    .json(serde_json::json!({
                        "status": "failed",
                        "data": format!("{} not exists", &username)
                    }))
            }
        }
        Err(error) => {
            HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "status": "failed",
                    "details": error.to_string()
                })
            )
        }
    }
}

pub async fn create_user(payload: web::Json<CreateUser>, pool: web::Data<MySqlPool>) -> impl Responder {
    let payload_user = payload.into_inner();

    if let Err(validation_errors) = payload_user.validate() {
        return validation_errors_response(&validation_errors);
    }

    let query = sqlx::query_as!(
        CreateUser,
        "INSERT INTO users (username, password) VALUES (?, ?);",
        payload_user.username,
        payload_user.password
    )
        .execute(pool.get_ref())
        .await;

    match query {
        Ok(_) => {
            let success_response = serde_json::json!({
                "status": "success",
                "msg": format!("record {} created successfully", payload_user.username)
            });
            HttpResponse::Created().json(success_response)
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "status": "false",
                    "msg": format!("failed to create {}", payload_user.username),
                    "details": err.to_string()
                })
            )
        }
    }
}

// pub async fn update_user(path: web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
//     HttpResponse::Ok().body("update user")
// }

// pub async fn delete_user(path: web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
//     HttpResponse::Ok().body("delete user")
// }