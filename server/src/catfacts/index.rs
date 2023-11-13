use axum::{
    extract::Json,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CatFact {
    data: Vec<String>,
}

/// CatFact Implementation
///
/// - Makes a new route based off ISO 639-1 standard
///
impl CatFact {
    /// Creates new Fact with specified language
    async fn new(lang: String) -> Result<Json<CatFact>, reqwest::Error> {
        let url = format!("https://meowfacts.herokuapp.com/?lang={}", lang);
        let response = reqwest::get(url).await?;
        let cat_fact: CatFact = response.json().await?;
        Ok(Json(cat_fact))
    }

    /// Converts new fact into json into a string
    async fn iso(lang: &str) -> impl IntoResponse {
        let res = self::CatFact::new(lang.to_owned())
            .await
            .unwrap()
            .0
            .data
            .join(" ");

        return serde_json::json!(res).to_string();
    }

    /// Sets up the Html with iso language for each country
    async fn set_lang(lang: &str) -> impl IntoResponse {
        Html(self::CatFact::iso(lang))
            .0
            .await
            .into_response()
            .into_body()
    }
}

pub async fn english() -> impl IntoResponse {
    CatFact::set_lang("eng-us").await
}

pub async fn czech() -> impl IntoResponse {
    CatFact::set_lang("ces-cz").await
}

pub async fn german() -> impl IntoResponse {
    CatFact::set_lang("ger-de").await
}

pub async fn bengali() -> impl IntoResponse {
    CatFact::set_lang("ben-in").await
}

pub async fn spanishes() -> impl IntoResponse {
    CatFact::set_lang("esp-es").await
}

pub async fn spanishmex() -> impl IntoResponse {
    CatFact::set_lang("esp-mx").await
}

pub async fn russian() -> impl IntoResponse {
    CatFact::set_lang("rus-ru").await
}

pub async fn portuguese() -> impl IntoResponse {
    CatFact::set_lang("por-br").await
}

pub async fn filipino() -> impl IntoResponse {
    CatFact::set_lang("fil-tl").await
}

pub async fn ukrainian() -> impl IntoResponse {
    CatFact::set_lang("ukr-ua").await
}

pub async fn urdu() -> impl IntoResponse {
    CatFact::set_lang("urd-ud").await
}

pub async fn italian() -> impl IntoResponse {
    CatFact::set_lang("ita-it").await
}

pub async fn chinese() -> impl IntoResponse {
    CatFact::set_lang("zho-tw").await
}

pub async fn korean() -> impl IntoResponse {
    CatFact::set_lang("kor-ko").await
}
