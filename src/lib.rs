use actix_web::HttpResponse;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use validator::ValidationErrors;

pub mod users;

pub async fn ping() -> HttpResponse{
    HttpResponse::Ok().json(
        serde_json::json!({
            "msg": "pong"
        })
    )
}

pub async fn establish_connection(db_url: &str) -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .unwrap_or_else(|err| {
            eprintln!("failed to create mysql pool connection [{}]", err);
            std::process::exit(1);
        })
}

pub fn validation_errors_response(validation_errors: &ValidationErrors) -> HttpResponse {
    let mut cleaned_errors = serde_json::Map::new();

    for (field, field_errors) in validation_errors.field_errors().iter() {
        let mut cleaned_field_errors = Vec::new();
        // println!("field: {:#?}", field);
        // println!("field_errors: {:#?}", field_errors);
        for error in field_errors.iter() {
            // println!("error: {:#?}", error);
            let cleaned_error = serde_json::json!({
                "code": error.code,
                "message": error.message
            });
            cleaned_field_errors.push(cleaned_error);
        }
        // println!("cleaned_field_error: {:#?}", cleaned_field_errors);
        cleaned_errors.insert(
            field.to_string(),
            serde_json::Value::Array(cleaned_field_errors)
        );
    }
    // println!("ALLLL : {:#?}", validation_errors);
    let error_response = serde_json::json!({
        "error": "Validation Failed",
        "details": cleaned_errors
    });

    // println!("{:#?}", error_response);

    HttpResponse::BadRequest().json(error_response)
}