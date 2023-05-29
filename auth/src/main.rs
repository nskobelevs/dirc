use std::env;

use actix_cors::Cors;
use actix_web::{
    get,
    http::{self},
    middleware::Logger,
    post, put, web, App, HttpRequest, HttpResponse, HttpServer,
};
use auth::{db::Authenticator, extract_bearer_token, LoginInfo, SessionToken, UserExistsParams};

use core_rs::{
    create_json_cfg,
    error::{Response, ServiceError},
    Username,
};

#[post("/login")]
async fn login(
    authenticator: web::Data<Authenticator>,
    info: web::Json<LoginInfo>,
) -> Response<SessionToken> {
    authenticator.login(info.into_inner()).await.into()
}

#[put("/register")]
async fn register(
    authenticator: web::Data<Authenticator>,
    info: web::Json<LoginInfo>,
) -> Response<SessionToken> {
    authenticator.register(info.into_inner()).await.into()
}

#[get("/authenticate")]
async fn authenticate(
    authenticator: web::Data<Authenticator>,
    req: HttpRequest,
) -> Response<Username> {
    let bearer_auth = match extract_bearer_token(&req) {
        Ok(bearer_auth) => bearer_auth,
        Err(err) => return Response::Err(err),
    };

    authenticator.authenticate(&bearer_auth).await.into()
}

#[post("/authorize")]
async fn authorize(
    authenticator: web::Data<Authenticator>,
    username: web::Json<Username>,
    req: HttpRequest,
) -> Response<Username> {
    let bearer_auth = match extract_bearer_token(&req) {
        Ok(bearer_auth) => bearer_auth,
        Err(err) => return Response::Err(err),
    };

    let token_username = match authenticator.authenticate(&bearer_auth).await {
        Ok(token_username) => token_username,
        Err(err) => return Response::Err(err),
    };

    if token_username.username == username.into_inner().username {
        Ok(token_username).into()
    } else {
        Err(ServiceError::AuthorizationError).into()
    }
}

#[get("/logout")]
async fn logout(authenticator: web::Data<Authenticator>, req: HttpRequest) -> Response<()> {
    let bearer_auth = match extract_bearer_token(&req) {
        Ok(bearer_auth) => bearer_auth,
        Err(err) => return Response::Err(err),
    };

    authenticator.logout(&bearer_auth).await.into()
}

#[get("/user_exists")]
async fn user_exists(
    authenticator: web::Data<Authenticator>,
    params: web::Query<UserExistsParams>,
) -> Response<bool> {
    authenticator
        .user_exists(params.username.clone())
        .await
        .into()
}

/// Custom 404 handler to return JSON
async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ServiceError::NotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting auth server...");

    let mongodb_hostname = env::var("MONGODB_HOSTNAME").unwrap_or_else(|_| "localhost".to_string());

    let mongodb_url = format!("mongodb://{}:27017", mongodb_hostname);

    println!("MongoDB url: {}", mongodb_url);

    let authenticator = Authenticator::new(mongodb_url, "auth".to_string())
        .await
        .expect("Failed to connect to MongoDB");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        // let cors = Cors::permissive()
        // .allowed_origin("http://localhost:3000")
        // .allowed_origin("http://live-chat:8080")
        // .allowed_origin("http://api:8080")
        // .allowed_methods(vec!["GET", "POST", "PUT"])
        // .allowed_headers(vec![http::header::AUTHORIZATION])
        // .allowed_header(http::header::CONTENT_TYPE)
        // .max_age(3600);

        App::new()
            // .wrap(cors)
            .wrap(Logger::default())
            .app_data(create_json_cfg())
            .app_data(web::Data::new(authenticator.clone()))
            .service(login)
            .service(register)
            .service(authenticate)
            .service(authorize)
            .service(logout)
            .service(user_exists)
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
