

use std::collections::HashMap;
use tracing::{info_span, Span};
use axum::{
    extract::{self, FromRef, FromRequestParts, Path, Query, State}, http::{request::Parts, StatusCode}, routing::{get, post}, Json, Router
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Value, json};
use std::sync::Arc;
use sqlx::{mysql::{MySqlPool, MySqlPoolOptions}, types::chrono, Executor, MySql, Pool};

#[derive(Deserialize, Serialize)]
pub struct CreateUserPayload {
    name: String
}

#[derive(Deserialize, Serialize)]
pub struct BaseRes {
    pub code: u32,
    pub msg: String
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}


// 表t_user, 这里字段可以少不能多，字段类型需要匹配, 驼峰也不能自动转换
#[derive(sqlx::FromRow)]
struct User { 
    pub name: Option<String>, 
    pub id: i64 , 
    pub phone: String, 
    pub password: String,    
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub status : i16
}


/**
 * 
 *  演示了路径参数，数据库连接使用和 json参数获取
 * 
 * use json request & response
 *  注意 参数定义顺序
 */

pub async fn create_user( 
    Path(id): Path<u32>,
    State(pool): State<Pool<MySql>>,
    Json(payload): Json<CreateUserPayload>,
    ) -> Json<BaseRes>{
        
    tracing::info!("请求名称 {}", payload.name);

    let mut stream = sqlx::query_as::<_, User>("SELECT * FROM t_user where id  = ?")
    .bind(id)
    .fetch_all(&pool).await
    ;
    let default1 = String::from("value");
    for i in &stream.unwrap() {
        println!("{}, {}", i.name.clone().unwrap_or( default1.clone()), i.create_time);
    }
    

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

