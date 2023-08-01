use mail_parser::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Eml {
    pub body: Arc<str>,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone)]
pub enum Attachment {
    Eml(Eml),
    Other {
        name: Option<Arc<str>>,
        content_type: Option<Arc<str>>,
        contents: Arc<[u8]>,
    },
}

impl From<Message<'_>> for Eml {
    fn from(message: Message<'_>) -> Self {
        let text_bodies_iter = message
            .text_bodies()
            .filter_map(|message_part| message_part.text_contents());

        let bodies: Vec<&str> = message
            .html_bodies()
            .filter_map(|message_part| message_part.text_contents())
            .chain(text_bodies_iter)
            .collect();

        let attachments = message.attachments().map(Into::into).collect();

        let body = bodies.join("\n\n").into();

        Eml { body, attachments }
    }
}

impl From<&Message<'_>> for Eml {
    fn from(value: &Message<'_>) -> Self {
        value.into()
    }
}

impl From<MessagePart<'_>> for Attachment {
    fn from(message_part: MessagePart<'_>) -> Self {
        if let Some(message) = message_part.message() {
            return Attachment::Eml(message.into());
        };

        if message_part
            .content_type()
            .map(|c| c.ctype())
            .unwrap_or_default()
            == "application"
            && message_part
                .attachment_name()
                .unwrap_or_default()
                .trim()
                .ends_with(".eml")
        {
            return Attachment::Eml(
                Message::parse(message_part.contents())
                    .expect("Failed to parse embedded message")
                    .into(),
            );
        }

        let attachment = Attachment::Other {
            name: message_part.attachment_name().map(Into::into),
            content_type: message_part
                .content_type()
                .map(|c| format!("{}/{}", c.ctype(), c.subtype().unwrap_or_default()).into()),
            contents: message_part.contents().into(),
        };

        attachment
    }
}

impl From<&MessagePart<'_>> for Attachment {
    fn from(value: &MessagePart<'_>) -> Self {
        value.into()
    }
}
