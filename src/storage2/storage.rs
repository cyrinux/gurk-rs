use std::borrow::Cow;

use crate::data::{Channel, ChannelId, Message};

pub trait Storage {
    fn channels<'s>(&'s self) -> Box<dyn Iterator<Item = Cow<Channel>> + 's>;
    fn channel(&self, channel_id: ChannelId) -> Option<Cow<Channel>>;
    fn store_channel(&mut self, channel: Channel) -> Cow<Channel>;

    fn messages<'s>(&'s self, channel_id: ChannelId)
        -> Box<dyn Iterator<Item = Cow<Message>> + 's>;
    fn message(&self, message_id: MessageId) -> Option<Cow<Message>>;
    fn store_message(&mut self, channel_id: ChannelId, message: Message) -> Cow<Message>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageId {
    pub channel_id: ChannelId,
    pub arrived_at: u64,
}

impl MessageId {
    pub fn new(channel_id: ChannelId, arrived_at: u64) -> Self {
        Self {
            channel_id,
            arrived_at,
        }
    }
}