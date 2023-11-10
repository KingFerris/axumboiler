use axum::prelude::*;

pub fn configure() -> Router {
    route("/", get(get_products).post(create_product))
}

async fn get_products() -> &'static str {
    "List of products"
}

async fn create_product() -> &'static str {
    "Create a product"
}
