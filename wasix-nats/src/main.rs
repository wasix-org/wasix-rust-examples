use futures::StreamExt;

const PUBLISH_SUBJECT: &str = "wasix.messages.rust";
const SUBSCRIBE_SUBJECT: &str = "wasix.messages.cli";

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    // Connect to the NATS server
    let client = async_nats::connect("127.0.0.1:4222").await?;
    println!("Connected to NATS server");

    // Subscribe to the "messages" subject
    let mut subscriber = client.subscribe(SUBSCRIBE_SUBJECT.into()).await?;

    // Publish messages to the "messages" subject
    for i in 0..10 {
        client
            .publish(PUBLISH_SUBJECT.into(), format!("Hello - {}", i).into())
            .await?;
    }

    // Receive and process messages
    while let Some(message) = subscriber.next().await {
        println!("Received message {:?}", message);
    }

    Ok(())
}
