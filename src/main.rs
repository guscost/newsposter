use serde::Deserialize;
use tiny_tokio_actor::*;

#[derive(Clone, Debug)]
struct Event(String);
impl SystemEvent for Event {}

#[derive(Clone)]
struct HeadlineMessage;
struct HeadlineResponse {
    title: String,
    url: String,
}
impl Message for HeadlineMessage {
    type Response = Result<HeadlineResponse, ActorError>;
}

#[derive(Default)]
struct HeadlineActor {
    key: String
}

#[async_trait]
impl Actor<Event> for HeadlineActor {
    async fn pre_start(&mut self, _: &mut ActorContext<Event>) -> Result<(), ActorError> {
        match std::env::var("NEWSAPI_KEY") {
            Ok(key) => self.key = key,
            Err(_) => ()
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct JsonNewsResponse {
    articles: Vec<JsonNewsItem>,
}
#[derive(Clone, Deserialize, Debug)]
struct JsonNewsItem {
    title: String,
    url: String,
}

#[async_trait]
impl Handler<Event, HeadlineMessage> for HeadlineActor {

    async fn handle(
        &mut self,
        _: HeadlineMessage,
        _: &mut ActorContext<Event>,
    ) -> Result<HeadlineResponse, ActorError> {
        let url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", self.key);
        let req = surf::get(url)
            .header("User-Agent", "My News Headline Fetcher")
            .recv_json::<JsonNewsResponse>();
        match req.await {
            Ok(resp) => {
                let top_article = resp.articles[0].clone();
                Ok(HeadlineResponse {
                    title: top_article.title,
                    url: top_article.url,
                })
            }
            Err(e) => Err(ActorError::CreateError(format!("ERROR {}", e.status()))),
        }
    }
}

#[derive(Clone)]
struct GeneratePostMessage(String);
impl Message for GeneratePostMessage {
    type Response = Result<String, ActorError>;
}
#[derive(Default)]
struct GeneratePostActor {
    key: String,
}
#[async_trait]
impl Actor<Event> for GeneratePostActor {
    async fn pre_start(&mut self, _: &mut ActorContext<Event>) -> Result<(), ActorError> {
        match std::env::var("OPENAI_KEY") {
            Ok(key) => self.key = key,
            Err(_) => ()
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct JsonGeneratePostResponse {
    choices: Vec<JsonGeneratePostItem>,
}
#[derive(Deserialize, Debug)]
struct JsonGeneratePostItem {
    message: JsonGeneratePostMessage,
}
#[derive(Deserialize, Debug)]
struct JsonGeneratePostMessage {
    content: String,
}

#[async_trait]
impl Handler<Event, GeneratePostMessage> for GeneratePostActor {
    async fn handle(
        &mut self,
        msg: GeneratePostMessage,
        _: &mut ActorContext<Event>,
    ) -> Result<String, ActorError> {
        let prompt = format!("Please respond as if you are posting on Mark Zuckerberg's new social network Threads, about your reaction to this news headline: \\\"{}\\\". Use the tone of a friendly but somewhat distant citizen. Keep the post under 500 characters, and do not add emojis or hashtags.", msg.0);
        let body = format!(
            "{{
                \"model\": \"gpt-4\",
                \"messages\": [{{\"role\": \"user\", \"content\": \"{}\"}}],
                \"temperature\": 1
            }}",
            prompt
        );
        let req = surf::post("https://api.openai.com/v1/chat/completions")
            .header("User-Agent", "My News Post Generator")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.key),
            )
            .body(body)
            .recv_json::<JsonGeneratePostResponse>();
        match req.await {
            Ok(resp) => Ok(resp.choices[0].message.content.clone()),
            Err(e) => {
                println!("{}", e);
                Err(ActorError::CreateError(format!("ERROR {}", e.status())))
            }
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), ActorError> {
    let bus = EventBus::<Event>::new(1000);
    let system = ActorSystem::new("newsposter", bus);
    let actor_headline = system
        .create_actor("news-headline-actor", HeadlineActor { key: "".to_string() })
        .await?;
    let actor_post = system
        .create_actor("generate-post-actor", GeneratePostActor { key: "".to_string() })
        .await?;

    // Wait a little for the actor to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let headline = actor_headline.ask(HeadlineMessage).await?;
    match headline {
        Ok(headline) => {
            println!("{}", headline.url);
            let msg = GeneratePostMessage(headline.title);
            let post = actor_post.ask(msg).await?;
            match post {
                Ok(post) => println!("{}", post),
                Err(e) => println!("ERROR: {}", e),
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
    Ok(())
}
