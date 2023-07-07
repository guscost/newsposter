use serde::Deserialize;

// NewsAPI response data
#[derive(Deserialize, Debug)]
pub struct JsonNewsResponse {
    pub articles: Vec<JsonNewsItem>,
}
#[derive(Clone, Deserialize, Debug)]
pub struct JsonNewsItem {
    pub title: String,
    pub url: String,
}

// OpenAI response data
#[derive(Deserialize, Debug)]
pub struct JsonGeneratePostResponse {
    pub choices: Vec<JsonGeneratePostItem>,
}
#[derive(Deserialize, Debug)]
pub struct JsonGeneratePostItem {
    pub message: JsonGeneratePostMessage,
}
#[derive(Deserialize, Debug)]
pub struct JsonGeneratePostMessage {
    pub content: String,
}
