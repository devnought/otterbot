extern crate hipchat;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod config;
pub use config::Config;

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

    let webhooks = vec![
        WebHook::new(
            "OB Fact",
            format!("{}/fact", endpoint_url),
            CapabilitiesEvent::RoomMessage("^/otterbot fact$"),
        ),
        WebHook::new(
            "OB Fact Add",
            format!("{}/fact/add", endpoint_url),
            CapabilitiesEvent::RoomMessage("^/otterbot fact add .+"),
        ),
    ];

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

pub fn fact(request: HipchatRequest) -> Notification {
    Notification::basic("These are true facts about otters", Color::Gray, MessageFormat::Text)
}

pub fn fact_add(request: HipchatRequest) -> Notification {
    Notification::basic("(thumbsup)", Color::Gray, MessageFormat::Text)
}
