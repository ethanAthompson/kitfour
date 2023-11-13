use axum::routing::get;
use axum::Router;

use self::index::catimages;

pub mod index;

pub fn routes_catimages() -> Router {
    Router::new().route("/catimages", get(catimages))
}
