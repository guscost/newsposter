use tiny_tokio_actor::*;

// Only one kind of system event for now
#[derive(Clone, Debug)]
pub struct Event(String);
impl SystemEvent for Event {}

// Actor for fetching headlines
#[derive(Default)]
pub struct HeadlineActor {
    pub key: String,
}

#[async_trait]
impl Actor<Event> for HeadlineActor {
    async fn pre_start(&mut self, _: &mut ActorContext<Event>) -> Result<(), ActorError> {
        match std::env::var("NEWSAPI_KEY") {
            Ok(key) => Ok(self.key = key),
            Err(_) => Ok(()),
        }
    }
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

// Actor for generating posts based on the headlines
#[derive(Default)]
pub struct GeneratePostActor {
    pub key: String,
}

#[async_trait]
impl Actor<Event> for GeneratePostActor {
    async fn pre_start(&mut self, _: &mut ActorContext<Event>) -> Result<(), ActorError> {
        match std::env::var("OPENAI_KEY") {
            Ok(key) => Ok(self.key = key),
            Err(_) => Ok(()),
        }
    }
}

#[derive(Clone)]
pub struct GeneratePostMessage(pub String);
impl Message for GeneratePostMessage {
    type Response = Result<String, ActorError>;
}
