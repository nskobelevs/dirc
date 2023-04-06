use actix_web::{error, post, web, App, HttpResponse, HttpServer};
use auth::{
    db::MongoDatabase,
    error::{AuthError, Response},
    Credentials, LoginInfo,
};

#[post("/login")]
async fn login(database: web::Data<MongoDatabase>, info: web::Json<LoginInfo>) -> Response<String> {
    AuthError::UserNotFound(info.username.clone()).into()
}

#[post("/register")]
async fn register(
    database: web::Data<MongoDatabase>,
    info: web::Json<LoginInfo>,
) -> Response<String> {
    let info = info.into_inner();

    let credentials = Credentials::new(info.clone());

    let token = database.attempt_register(credentials).await;

    match token {
        Some(token) => token.into(),
        None => AuthError::UsernameTaken(info.username.clone()).into(),
    }
}

/// Custom 404 handler to return JSON
async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(AuthError::NotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = MongoDatabase::new("mongodb://localhost:27017".to_string(), "auth".to_string())
        .await
        .expect("Failed to connect to MongoDB");

    HttpServer::new(move || {
        App::new()
            .app_data(create_json_cfg())
            .app_data(web::Data::new(database.clone()))
            .service(login)
            .service(register)
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", 8081))?
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