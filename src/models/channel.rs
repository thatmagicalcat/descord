use crate::utils;
use nanoserde::{DeJson, SerJson};

use super::message_response::{CreateMessageData, Message};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Channel {
    pub last_message_id: Option<String>,
    pub flags: Option<usize>,
    pub guild_id: Option<String>,
    pub parent_id: Option<String>,
    pub topic: Option<String>,
    pub rate_limit_per_user: Option<usize>,
    pub position: Option<usize>,
    pub nsfw: Option<bool>,
    pub name: Option<String>,
    pub id: String,

    #[nserde(rename = "type")]
    pub channel_type: u32,

    #[nserde(default)]
    pub mention: String,
}

impl Channel {
    pub async fn send_message(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::send(&self.id, data).await
    }
}
