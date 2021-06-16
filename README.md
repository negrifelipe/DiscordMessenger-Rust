# DiscordMessenger-Rust
A library that allows you to easily send discord webhooks

## Usage
Add to the **Cargo.toml** on dependencies
**DiscordMessenger = "1.0.0"**

## Example
```rs
use DiscordMessenger::{DiscordMessage, EmbedBuilder};

fn main() {
    DiscordMessage::new()
        .set_content("Hello World from the best lib".to_string())
        .add_embed(EmbedBuilder::new()
            .set_title("Feli".to_string())
            .set_description("An embed using the discord messenger library".to_string())
            .build())
        .set_username("Discord Messenger".to_string())
        .send("https://discord.com/api/webhooks/xxxx/xxxx".to_string());
}
```
