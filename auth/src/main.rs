use actix_web::{error, post, web, App, HttpResponse, HttpServer};
use auth::{
    db::Authenticator,
    error::{AuthError, Response},
    LoginInfo, SessionToken,
};

#[post("/login")]
async fn login(
    authenticator: web::Data<Authenticator>,
    info: web::Json<LoginInfo>,
) -> Response<SessionToken> {
    authenticator.login(info.into_inner()).await.into()
}

#[post("/register")]
async fn register(
    authenticator: web::Data<Authenticator>,
    info: web::Json<LoginInfo>,
) -> Response<SessionToken> {
    authenticator.register(info.into_inner()).await.into()
}

#[post("/authenticate")]
async fn authenticate(
    authenticator: web::Data<Authenticator>,
    token: web::Json<SessionToken>,
) -> Response<()> {
    authenticator.authenticate(token.into_inner()).await.into()
}

#[post("/logout")]
async fn logout(
    authenticator: web::Data<Authenticator>,
    token: web::Json<SessionToken>,
) -> Response<()> {
    authenticator.logout(token.into_inner()).await.into()
}

/// Custom 404 handler to return JSON
async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(AuthError::NotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let authenticator =
        Authenticator::new("mongodb://localhost:27017".to_string(), "auth".to_string())
            .await
            .expect("Failed to connect to MongoDB");

    HttpServer::new(move || {
        App::new()
            .app_data(create_json_cfg())
            .app_data(web::Data::new(authenticator.clone()))
            .service(login)
            .service(register)
            .service(authenticate)
            .service(logout)
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
