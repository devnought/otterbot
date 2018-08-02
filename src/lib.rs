extern crate hipchat;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod config;
pub use config::Config;

mod message;
pub use message::Message;

use hipchat::{
    capabilities::{
        Avatar, Capabilities, CapabilitiesDescriptor, CapabilitiesEvent, HipchatApiConsumer, Links,
        Scope, WebHook,
    },
    notification::{Color, MessageFormat, Notification},
    request::HipchatRequest,
};

pub fn build_descriptor<'a>(host: &str) -> CapabilitiesDescriptor<'a> {
    let endpoint_url = format!("http://{}/otterbot", host);
    let avatar = Avatar::new("https://upload.wikimedia.org/wikipedia/commons/0/02/Sea_Otter_%28Enhydra_lutris%29_%2825169790524%29_crop.jpg");

    let scopes = vec![Scope::SendNotification];
    let api_consumer = HipchatApiConsumer::with_avatar(avatar, "Otter Bot", scopes);

    let webhooks = vec![WebHook::new(
        "OB",
        endpoint_url.clone(),
        CapabilitiesEvent::RoomMessage("^/otterbot.*"),
    )];

    let capabilities = Capabilities::new(api_consumer, webhooks);
    let links = Links::new(endpoint_url);

    CapabilitiesDescriptor::new(
        capabilities,
        "Otter related shenanigans",
        "com.devnought.otterbot",
        links,
        "Otter Bot",
    )
}

pub fn post(request: HipchatRequest, config: &Config) -> Option<Notification> {
    let message_item = match request {
        HipchatRequest::RoomMessage { item, .. } => item,
        _ => panic!("unsupported message type"),
    };

    let message = message_item.message();
    let parsed_message = config.parse_command_message(message)?;

    Some(match parsed_message {
        Message::Error => Notification::basic("(sadotter)", Color::Red, MessageFormat::Text),
        Message::Image(url) => {
            let image = format!("<img width=\"300px\" src=\"{}\" />", url);
            Notification::basic(image, Color::Green, MessageFormat::Html)
        }
        Message::Text(msg) => Notification::basic(msg, Color::Gray, MessageFormat::Text),
    })
}
