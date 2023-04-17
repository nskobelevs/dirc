use std::env;

use actix_web::{
    error::{self, ParseError},
    get,
    http::header::Header,
    post, put, web, App, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use auth::{
    db::Authenticator,
    error::{AuthError, Response},
    AuthenticateResult, LoginInfo, SessionToken, UserExistsParams,
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
) -> Response<AuthenticateResult> {
    let brearer_auth = {
        let parsed_auth = Authorization::<Bearer>::parse(&req);

        match parsed_auth {
            Ok(auth) => auth.into_scheme(),
            Err(ParseError::Header) => {
                return Response::Err(AuthError::AuthorizationHeaderError);
            }
            Err(_) => {
                return Response::Err(AuthError::AuthenticationError);
            }
        }
    };

    authenticator
        .authenticate(brearer_auth.token())
        .await
        .into()
}

#[post("/logout")]
async fn logout(
    authenticator: web::Data<Authenticator>,
    token: web::Json<SessionToken>,
) -> Response<()> {
    authenticator.logout(token.into_inner()).await.into()
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
    HttpResponse::NotFound().json(AuthError::NotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting auth server...");

    let mongodb_hostname = env::var("MONGODB_HOSTNAME").unwrap_or("localhost".to_string());

    let mongodb_url = format!("mongodb://{}:27017", mongodb_hostname);

    println!("MongoDB url: {}", mongodb_url);

    let authenticator = Authenticator::new(mongodb_url, "auth".to_string())
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
            .service(user_exists)
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", 8080))?
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
