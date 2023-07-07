use tiny_tokio_actor::*;

#[derive(Default)]
pub struct HeadlineActor {
    pub key: String,
}

#[derive(Clone)]
pub struct HeadlineMessage;
pub struct HeadlineResponse {
    pub title: String,
    pub url: String,
}
impl Message for HeadlineMessage {
    type Response = Result<HeadlineResponse, ActorError>;
}

#[derive(Default)]
pub struct GeneratePostActor {
    pub key: String,
}

#[derive(Clone)]
pub struct GeneratePostMessage(pub String);
impl Message for GeneratePostMessage {
    type Response = Result<String, ActorError>;
}
