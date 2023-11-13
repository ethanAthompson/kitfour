pub mod index;

use axum::{routing::get, Router};

use self::index::*;

pub fn routes_catfacts() -> Router {
    Router::new()
        .route("/catfacts/eng-us", get(english))
        .route("/catfacts/ces-cz", get(czech))
        .route("/catfacts/ger-de", get(german))
        .route("/catfacts/ben-in", get(bengali))
        .route("/catfacts/esp-es", get(spanishes))
        .route("/catfacts/esp-mx", get(spanishmex))
        .route("/catfacts/rus-ru", get(russian))
        .route("/catfacts/por-br", get(portuguese))
        .route("/catfacts/fil-tl", get(filipino))
        .route("/catfacts/ukr-ua", get(ukrainian))
        .route("/catfacts/urd-ud", get(urdu))
        .route("/catfacts/ita-it", get(italian))
        .route("/catfacts/zho-tw", get(chinese))
        .route("/catfacts/kor-ko", get(korean))
}
