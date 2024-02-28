
use axum::{
    http::StatusCode, Json,
};
use serde::{Deserialize, Serialize};




pub async fn create_user(Json(payload): Json<CreateUser>) -> Result<(StatusCode, Json<User>), StatusCode>{
    let user = User {
        id: 1337,
        username: payload.username,
    };
        
    Ok((StatusCode::CREATED, Json(user)))  
}

pub async fn create_machine(Json(payload): Json<CreateMachine>) -> Result<(StatusCode, Json<Machine>), StatusCode>{
    let machine = Machine {
        id: payload.id,
        name: payload.name,
    };
    Ok((StatusCode::CREATED, Json(machine)))
}

pub async fn create_service(Json(payload): Json<CreateService>) -> Result<(StatusCode, Json<Service>), StatusCode>{
    let service = Service {
        id: payload.id,
        name: payload.name,
        machine_id: payload.machine_id,
    };
    Ok((StatusCode::CREATED, Json(service)))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize)]
pub struct CreateMachine {
    name: String,
    id : u64,
}

#[derive(Serialize)]
pub struct Machine {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
pub struct CreateService {
    id: u64,
    name: String,
    machine_id: u64,
}

#[derive(Serialize)]
pub struct Service {
    id: u64,
    name: String,
    machine_id: u64,
}
