use axum::{
    extract::Json,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CatImage {
    pub id: String,
    pub url: String,
    pub width: u16,
    pub height: u16,
}

async fn get_catimages() -> Result<Json<Vec<CatImage>>, reqwest::Error> {
    // Works by automatic refresh
    let url = "https://api.thecatapi.com/v1/images/search";
    let response = reqwest::get(url).await?;
    let cat_image: Vec<CatImage> = response.json().await?;
    Ok(Json(cat_image))
}

pub async fn catimages() -> impl IntoResponse {
    // Getting the img property from the {id:"", url: "", ...}
    let response = get_catimages()
        .await
        .map(|data| data.0.first().map(|img| img.url.clone()));

    let error_img =
        "https://t3.ftcdn.net/jpg/02/61/08/76/360_F_261087622_8eRI0TAwDAyabS1b0Uifx1wKqHzA41r3.jpg"
            .to_string();

    match response {
        Ok(Some(res)) => Html(res),
        _ => Html(error_img),
    }
}
