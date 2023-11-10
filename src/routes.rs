use axum::prelude::*;
use crate::apis::v1::products::product_routes;
//use crate::apis::v1::categories::category_routes;
use axum::{Router};

pub fn project_routes() -> Router {
    .route("/products",get(product_routes::configure()))
}





