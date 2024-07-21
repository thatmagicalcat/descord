# descord
Descord is a minimal, easy to use discord api wrapper.

# NOTE: This repository is moved to https://github.com/DescordLib/descord

## Example
```rust
use descord::prelude::*;

#[tokio::main]
async fn main() {
    let mut client = Client::new(
        "TOKEN",
        GatewayIntent::MessageContent | GatewayIntent::GuildMessages,
        "!" // default prefix
    )
    .await;

    client.register_commands(vec![ping()]);
    client.register_events(vec![ready()]);

    client.login().await;
}

#[descord::command]
async fn ping(data: Message) {
    let clock = std::time::Instant::now();
    let msg = data.reply("Pong!").await;
    let latency = clock.elapsed().as_millis();

    msg.edit(format!("Pong! `{}ms`", latency)).await;
}

// Event type will be inferred from function name
#[descord::event]
async fn ready(data: ReadyData) {
    println!(
        "Logged in as: {}#{}",
        data.user.username, data.user.discriminator
    );
}
```
