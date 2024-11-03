
mod router;
mod utils;
mod controller;
use std::{collections::HashMap, fmt::Debug, time::Duration};

use axum::{
    extract::{Path, Query}, routing::get, routing::post, Json, Router
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Value, json};
use utils::httpUtils;
use std::sync::Arc;
use tracing::{info_span, instrument::WithSubscriber, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use dotenv::dotenv;
use std::env;


// 引用不同目录下的代码
fn testuee() {
    utils::httpUtils::get();
    httpUtils::User{};
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let version = env::var("VERSION").expect("VERSION 没有在 .env 文件里设置");
    let MYSQLURI = env::var("MYSQLURI").expect("VERSION 没有在 .env 文件里设置");

    let file_appender = tracing_appender::rolling::daily("./logs", "mylog");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);



    tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_ansi(false).with_line_number(true)
    .with_writer(non_blocking))
    .init();

    // build our application with a single route
    let  app1 = router::init(&MYSQLURI).await;
    
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}, version: {}", listener.local_addr().unwrap(), version);
    axum::serve(listener, app1).await.unwrap();
}
