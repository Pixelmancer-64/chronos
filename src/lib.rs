use axum::{
    routing::get,
    Router,
};

pub async fn run(){

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn index() -> String {
    String::from("Oi Ana, olha só que louco!")
}
