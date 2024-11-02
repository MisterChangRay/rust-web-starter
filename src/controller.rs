

use std::collections::HashMap;
use tracing::{info_span, Span};
use axum::{
    extract::{Path, Query}, routing::get, routing::post, Json, Router
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Value, json};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct CreateUserPayload {
    name: String
}

#[derive(Deserialize, Serialize)]
pub struct BaseRes {
    pub code: u32,
    pub msg: String
}

// use json request & response
pub async fn create_user(Json(payload): Json<CreateUserPayload>) -> Json<BaseRes>{
    tracing::info!("请求名称 {}", payload.name);

    Json(BaseRes { code: 0, msg: payload.name })
}

// `Path` gives you the path parameters and deserializes them.
pub async fn path(Path(user_id): Path<u32>) -> Json<Value>{
    Json(json!({ "data": user_id }))
    
}

// `Query` gives you the query parameters and deserializes them.
pub async fn query(Query(params): Query<HashMap<String, String>>)  -> Json<Value>{
    Json(json!({ "data": params.get("name") }))
}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
pub async fn postjson(Json(payload): Json<serde_json::Value>) -> Json<Value> {
    tracing::info!("请求名称 {}", payload.get("name").unwrap());
    Json(json!({ "data": payload.get("name") }))
}

