extern crate hipchat;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod config;
mod data;

pub use config::Config;
pub use data::DataStore;
use hipchat::{
    capabilities::{
        Avatar, Capabilities, CapabilitiesDescriptor, CapabilitiesEvent, HipchatApiConsumer, Links,
        Scope, WebHook,
    },
    notification::{Color, MessageFormat, Notification},
    request::HipchatRequest,
};
use rand::{thread_rng, Rng};
use std::sync::{Arc, RwLock};

pub fn build_descriptor<'a>(host: &str) -> CapabilitiesDescriptor<'a> {
    let endpoint_url = format!("http://{}/otterbot", host);
    let avatar = Avatar::new("https://upload.wikimedia.org/wikipedia/commons/0/02/Sea_Otter_%28Enhydra_lutris%29_%2825169790524%29_crop.jpg");

    let scopes = vec![Scope::SendNotification];
    let api_consumer = HipchatApiConsumer::with_avatar(avatar, "Otter Bot", scopes);

    let webhooks = vec![
        WebHook::new(
            "OB Fact",
            format!("{}/fact", endpoint_url),
            CapabilitiesEvent::RoomMessage(r"^/otterbot fact\s*$"),
        ),
        WebHook::new(
            "OB Fact Add",
            format!("{}/fact/add", endpoint_url),
            CapabilitiesEvent::RoomMessage(r"^/otterbot fact add.*"),
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

pub fn fact(_request: HipchatRequest, datastore: Arc<RwLock<DataStore>>) -> Notification {
    let db = datastore.read().expect("Could not acquire read lock");

    if db.is_empty() {
        return Notification::basic("No fact for you", Color::Red, MessageFormat::Text);
    }

    let fact = {
        let index = {
            let mut rng = thread_rng();
            rng.gen_range(0, db.len())
        };

        db.get(index)
    };

    Notification::basic(fact, Color::Gray, MessageFormat::Text)
}

pub fn fact_add(request: HipchatRequest, datastore: Arc<RwLock<DataStore>>) -> Notification {
    let raw_message = match request {
        HipchatRequest::RoomMessage { ref item, .. } => String::from(item.message()),
        _ => panic!("Unsuportted message type"),
    };

    let start_msg = "/otterbot fact add";

    if !raw_message.starts_with(start_msg) {
        return Notification::basic(
            "I think this command is broken",
            Color::Red,
            MessageFormat::Text,
        );
    }

    let (_, right) = raw_message.split_at(start_msg.len());
    let message = right.trim();

    if message.is_empty() {
        return Notification::basic(
            "This is not a fact (stare)",
            Color::Red,
            MessageFormat::Text,
        );
    }

    {
        let mut db = datastore.write().expect("Could not acquire write lock");
        db.add(message.into());
    }

    // TODO: Clone the datastore at this point,
    // then write it out to the filesystem.

    Notification::basic("(thumbsup)", Color::Green, MessageFormat::Text)
}
