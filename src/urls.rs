pub fn create_routes(){
    Router::new().route("/", get(index));
}