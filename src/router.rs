

use std::collections::HashMap;
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    extract::{Path, Query}, routing::{get, post, Route}, Json, Router
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Value, json};

use crate::controller;

use tokio::task_local;


task_local! {
    pub static USER: CurrentUser;
}


pub fn init()  -> Router {
    let  route = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/path/:user_id", get(controller::path))
    .route("/query", get(controller::query))
    .route("/json", post(controller::postjson))
    .route(
        "/addusers",
        post({
            move |body| controller::create_user(body)
        }),
    )
    // .route_layer(middleware::from_fn(auth))
    ;
    route
}

// header鉴权
async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if let Some(current_user) = authorize_current_user(auth_header).await {
        // State is setup here in the middleware
        Ok(USER.scope(current_user, next.run(req)).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    print!("request token {}", auth_token);
    Some(CurrentUser {
        session: auth_token.to_string(),
    })
}

#[derive(Clone)]
struct CurrentUser {
    session: String,
}