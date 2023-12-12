use actix_web::{HttpServer, App, middleware, web};
use rt01_check_validation_erros_response::{
    users::routes::scoped_users, 
    establish_connection, 
    ping,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // std::env::set_var("RUST_LOG", "DEBUG");
    // env_logger::init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!("DATABASE_URL must be set first [{}]", err);
        std::process::exit(1);
    });

    let pool = establish_connection(&db_url).await;
    

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(
                    pool.clone()
                )
            )
            .wrap(
                middleware::NormalizePath::trim()
            )
            .route("/ping", web::get().to(ping))
            .configure(scoped_users)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
