use serenity::model::prelude::Ready;

pub enum ChannelMessage {
    BotConnected(Box<Ready>),
    UserJoinedChannel(String, String),
    UserAlreadyInChannel(String, String),
    UserLeftChannel(String),
}

#[derive(Debug, Clone, Copy)]
pub enum NotificationSound {
    Default,
    IM,
    Mail,
    Reminder,
    SMS,
}

impl NotificationSound {
    pub fn to_str(self) -> String {
        match self {
            NotificationSound::Default => "Default",
            NotificationSound::IM => "IM",
            NotificationSound::Mail => "Mail",
            NotificationSound::Reminder => "Reminder",
            NotificationSound::SMS => "SMS",
        }
        .to_owned()
    }
}
