use axum::{
    http::StatusCode, routing::{get, post}, Json, Router
};
/* use serde::{Deserialize, Serialize}; */

mod routes;

//server port : 5432

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(routes::create_user))
        .route("/machine", post(routes::create_machine))
        .route("/service", post(routes::create_service));
        


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3010").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str{
    return "Hello Axum!";
}

/* async fn create_user(Json(body): Json<CreateUser>) -> Result<(StatusCode, Json<User>), StatusCode>{
    let user = User {
        id: 1488,
        username: body.username,
    };
    
    Ok((StatusCode::CREATED, Json(user)))
    
} */

/* #[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
} */