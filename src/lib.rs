use serde::Serialize;
use std::clone::Clone;

#[derive(Serialize)]
pub struct DiscordMessage {
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub content: Option<String>,
    pub tts: Option<bool>,
    pub embeds: Vec<Embed>
}

impl DiscordMessage {
    pub fn new() -> Self {
        Self {
            username: None,
            avatar_url: None,
            content: None,
            tts: None,
            embeds: vec![]
        }
    }

    pub fn set_username(&mut self, username: String) -> &mut DiscordMessage {
        self.username = Some(username);
        self
    }

    pub fn set_avatar(&mut self, avatar: String) -> &mut DiscordMessage {
        self.avatar_url = Some(avatar);
        self
    }

    pub fn set_content(&mut self, content: String) -> &mut DiscordMessage {
        self.content = Some(content);
        self
    }

    pub fn set_tts(&mut self, tts: bool) -> &mut DiscordMessage {
        self.tts = Some(tts);
        self
    }

    pub fn add_embed(&mut self, embed: Embed) -> &mut DiscordMessage {
        self.embeds.push(embed);
        self
    }

    pub fn send(&mut self, webhookUrl: String) {
        let client = reqwest::Client::new();
        client.post(&webhookUrl).json(self).send();
    }
}

#[derive(Serialize, Clone)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<i32>,
    pub footer: Option<Footer>,
    pub image: Option<Image>,
    pub thumbnail: Option<Thumbnail>,
    pub video: Option<Video>,
    pub provider: Option<Provider>,
    pub author: Option<Author>,
    pub fields: Vec<Field>
}

#[derive(Clone)]
pub struct EmbedBuilder {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<i32>,
    pub footer: Option<Footer>,
    pub image: Option<Image>,
    pub thumbnail: Option<Thumbnail>,
    pub video: Option<Video>,
    pub provider: Option<Provider>,
    pub author: Option<Author>,
    pub fields: Vec<Field>
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: vec![]
        }
    }

    pub fn set_title(&mut self, title: String) -> &mut EmbedBuilder {
        self.title = Some(title);
        self
    }

    pub fn set_description(&mut self, description: String) -> &mut EmbedBuilder {
        self.description = Some(description);
        self
    }

    pub fn set_url(&mut self, url: String) -> &mut EmbedBuilder {
        self.url = Some(url);
        self
    }

    pub fn set_timestamp(&mut self, time: String) -> &mut EmbedBuilder {
        self.timestamp = Some(time);
        self
    }

    pub fn set_color(&mut self, color: i32) -> &mut EmbedBuilder {
        self.color = Some(color);
        self
    }

    pub fn set_footer(&mut self, footer: Footer) -> &mut EmbedBuilder {
        self.footer = Some(footer);
        self
    }

    pub fn set_image(&mut self, image: Image) -> &mut EmbedBuilder {
        self.image = Some(image);
        self
    }

    pub fn set_thumbnail(&mut self, thumbnail: Thumbnail) -> &mut EmbedBuilder {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn set_video(&mut self, video: Video) -> &mut EmbedBuilder {
        self.video = Some(video);
        self
    }

    pub fn set_provider(&mut self, provider: Provider) -> &mut EmbedBuilder {
        self.provider = Some(provider);
        self
    }

    pub fn set_author(&mut self, author: Author) -> &mut EmbedBuilder {
        self.author = Some(author);
        self
    }

    pub fn add_field(&mut self, field: Field) -> &mut EmbedBuilder {
        self.fields.push(field);
        self
    }

    pub fn build(&mut self) -> Embed {
        Embed {
            title: self.title.clone(),
            description: self.description.clone(),
            url: self.url.clone(),
            timestamp: self.timestamp.clone(),
            color: self.color.clone(),
            footer: self.footer.clone(),
            image: self.image.clone(),
            thumbnail: self.thumbnail.clone(),
            video: self.video.clone(),
            provider: self.provider.clone(),
            author: self.author.clone(),
            fields: self.fields.clone()
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool
}

impl Field {
    pub fn new(name: String, value: String, inline: bool) -> Self {
        Self {
            name,
            value,
            inline
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Author {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>
}

impl Author {
    pub fn new() -> Self {
        Self {
            name: None,
            url: None,
            icon_url: None,
            proxy_icon_url: None
        }
    }

    pub fn set_name(&mut self, name: String) -> &mut Author {
        self.name = Some(name);
        self
    }

    pub fn set_url(&mut self, url: String) -> &mut Author {
        self.url = Some(url);
        self
    }

    pub fn set_icon_url(&mut self, icon_url: String) -> &mut Author {
        self.icon_url = Some(icon_url);
        self
    }

    pub fn set_proxy_icon_url(&mut self, proxy_icon_url: String) -> &mut Author {
        self.proxy_icon_url = Some(proxy_icon_url);
        self
    }
}

#[derive(Serialize, Clone)]
pub struct Provider {
    pub name: Option<String>,
    pub url: Option<String>
}

impl Provider {
    pub fn new() -> Self {
        Self {
            name: None,
            url: None,
        }
    }

    pub fn set_name(&mut self, name: String) -> &mut Provider {
        self.name = Some(name);
        self
    }

    pub fn set_url(&mut self, url: String) -> &mut Provider {
        self.url = Some(url);
        self
    }
}

#[derive(Serialize, Clone)]
pub struct Video {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>
}

impl Video {
    pub fn new() -> Self {
        Self {
            url: None,
            proxy_url: None,
            height: None,
            width: None
        }
    }

    pub fn set_url(&mut self, url: String) -> &mut Video {
        self.url = Some(url);
        self
    }

    pub fn set_proxy_url(&mut self, proxy_url: String) -> &mut Video {
        self.proxy_url = Some(proxy_url);
        self
    }

    pub fn set_height(&mut self, height: i32) -> &mut Video {
        self.height = Some(height);
        self
    }

    pub fn set_width(&mut self, width: i32) -> &mut Video {
        self.width = Some(width);
        self
    }
}

#[derive(Serialize, Clone)]
pub struct Thumbnail {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>
}

impl Thumbnail {
    pub fn new() -> Self {
        Self {
            url: None,
            proxy_url: None,
            height: None,
            width: None
        }
    }

    pub fn set_url(&mut self, url: String) -> &mut Thumbnail {
        self.url = Some(url);
        self
    }

    pub fn set_proxy_url(&mut self, proxy_url: String) -> &mut Thumbnail {
        self.proxy_url = Some(proxy_url);
        self
    }

    pub fn set_height(&mut self, height: i32) -> &mut Thumbnail {
        self.height = Some(height);
        self
    }

    pub fn set_width(&mut self, width: i32) -> &mut Thumbnail {
        self.width = Some(width);
        self
    }
}

#[derive(Serialize, Clone)]
pub struct Image {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>
}

impl Image {
    pub fn new() -> Self {
        Self {
            url: None,
            proxy_url: None,
            height: None,
            width: None
        }
    }

    pub fn set_url(&mut self, url: String) -> &mut Image {
        self.url = Some(url);
        self
    }

    pub fn set_proxy_url(&mut self, proxy_url: String) -> &mut Image {
        self.proxy_url = Some(proxy_url);
        self
    }

    pub fn set_height(&mut self, height: i32) -> &mut Image {
        self.height = Some(height);
        self
    }

    pub fn set_width(&mut self, width: i32) -> &mut Image {
        self.width = Some(width);
        self
    }
}

#[derive(Serialize, Clone)]
pub struct Footer {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>
}

impl Footer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            icon_url: None,
            proxy_icon_url: None
        }
    }

    pub fn set_icon_url(&mut self, icon_url: String) -> &mut Footer {
        self.icon_url = Some(icon_url);
        self
    }

    pub fn set_proxy_icon_url(&mut self, proxy_icon_url: String) -> &mut Footer {
        self.proxy_icon_url = Some(proxy_icon_url);
        self
    }
}
