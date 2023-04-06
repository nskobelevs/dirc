use actix_web::{
    error::{self},
    post,
    web::{self},
    App, HttpResponse, HttpServer,
};
use auth::{
    error::{AuthError, Response},
    LoginInfo,
};

#[post("/login")]
async fn login(info: web::Json<LoginInfo>) -> Response<String> {
    AuthError::UserNotFound(info.username.clone()).into()
}

/// Custom 404 handler to return JSON
async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(AuthError::NotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(create_json_cfg())
            .service(login)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn create_json_cfg() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .content_type(|mime| mime == mime::TEXT_PLAIN || mime == mime::APPLICATION_JSON)
        .error_handler(|err, _req| {
            let error_str = err.to_string();

            error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(AuthError::JsonParsingError(error_str)),
            )
            .into()
        })
}
