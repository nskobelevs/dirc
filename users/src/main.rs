use std::env;

use actix_cors::Cors;
use actix_web::{
    get,
    http::{self},
    post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use core_rs::{
    create_json_cfg,
    error::{Response, ServiceError},
    ProfilePicture,
};
use users::{authenticate, db::Users, User};

#[get("/{username}/exists")]
async fn exists(users: web::Data<Users>, path: web::Path<String>) -> Response<bool> {
    users.exists(path.into_inner()).await.into()
}

#[get("/{username}/info")]
async fn info(users: web::Data<Users>, path: web::Path<String>) -> Response<User> {
    users.info(path.into_inner()).await.into()
}

#[post("/{username}/info")]
async fn put_info(
    users: web::Data<Users>,
    path: web::Path<String>,
    profile_picture: web::Json<ProfilePicture>,
    req: HttpRequest,
) -> Response<()> {
    let username = path.into_inner();

    if let Err(err) = authenticate(&req, &username).await {
        return Response::Err(err);
    }

    users
        .save_info(username, profile_picture.into_inner())
        .await
        .into()
}

// TODO: This would only be allowed by the auth service and not externally...
#[put("/{username}/info")]
async fn create_info(
    users: web::Data<Users>,
    path: web::Path<String>,
    profile_picture: web::Json<ProfilePicture>,
) -> Response<()> {
    let username = path.into_inner();

    users
        .create_info(username, profile_picture.into_inner())
        .await
        .into()
}
/// Custom 404 handler to return JSON
async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(ServiceError::NotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting users server...");

    let mongodb_hostname = env::var("MONGODB_HOSTNAME").unwrap_or_else(|_| "localhost".to_string());

    let mongodb_url = format!("mongodb://{}:27017", mongodb_hostname);

    println!("MongoDB url: {}", mongodb_url);

    let users = Users::new(mongodb_url, "users".to_string())
        .await
        .expect("Failed to connect to MongoDB");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://auth:8080")
            .allowed_methods(vec!["GET", "POST", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(create_json_cfg())
            .app_data(web::Data::new(users.clone()))
            .service(exists)
            .service(info)
            .service(put_info)
            .service(create_info)
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
