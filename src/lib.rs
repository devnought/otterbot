extern crate hipchat;
extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate url;

mod data;

pub use data::{DataEntry, DataStore};
use hipchat::{
    capabilities::{
        Avatar, Capabilities, CapabilitiesDescriptor, CapabilitiesEvent, HipchatApiConsumer, Links,
        Scope, WebHook,
    },
    notification::{Color, MessageFormat, Notification},
    request::HipchatRequest,
};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    sync::{Arc, RwLock},
};
use url::Url;

const CMD: &str = "/ob";

pub fn build_descriptor<'a>(host: &str) -> CapabilitiesDescriptor<'a> {
    let endpoint_url = format!("http://{}/otterbot", host);
    let avatar = Avatar::new("https://upload.wikimedia.org/wikipedia/commons/0/02/Sea_Otter_%28Enhydra_lutris%29_%2825169790524%29_crop.jpg");

    let scopes = vec![Scope::SendNotification];
    let api_consumer = HipchatApiConsumer::with_avatar(avatar, "Otter Bot", scopes);

    let webhooks = vec![WebHook::new(
        "OB",
        endpoint_url.clone(),
        CapabilitiesEvent::RoomMessage(format!(r"^{}.*", CMD)),
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

pub fn dispatcher(request: HipchatRequest, datastore: Arc<RwLock<DataStore>>) -> Notification {
    // TODO: Look into RegexSet
    lazy_static! {
        static ref FACT_PUSH: Regex =
            Regex::new(&format!("^{}\\s+fact\\s+push\\s+(.+)$", CMD)).unwrap();
        static ref FACT_POP: Regex = Regex::new(&format!("^{}\\s+fact\\s+pop\\s*$", CMD)).unwrap();
        static ref FACT: Regex = Regex::new(&format!("^{}\\s+fact\\s*$", CMD)).unwrap();
    }

    let raw_message = match request {
        HipchatRequest::RoomMessage { ref item, .. } => String::from(item.message()),
        _ => panic!("Unsuportted message type"),
    };

    if !raw_message.starts_with(CMD) {
        return Notification::basic(
            "I think this command is broken",
            Color::Red,
            MessageFormat::Text,
        );
    }

    if FACT.is_match(&raw_message) {
        fact(datastore)
    } else if FACT_PUSH.is_match(&raw_message) {
        let capture = FACT_PUSH.captures(&raw_message).unwrap();
        let message = capture.get(1).map_or("", |m| m.as_str().trim());
        fact_push(message, datastore)
    } else if FACT_POP.is_match(&raw_message) {
        fact_pop(datastore)
    } else {
        Notification::basic("(sadotter)", Color::Red, MessageFormat::Text)
    }
}

pub fn fact(datastore: Arc<RwLock<DataStore>>) -> Notification {
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

    match fact {
        DataEntry::Image(url) => Notification::basic(
            format!("<img width=\"300px\" src=\"{}\" />", url),
            Color::Gray,
            MessageFormat::Html,
        ),
        DataEntry::Text(msg) => Notification::basic(msg, Color::Gray, MessageFormat::Text),
    }
}

pub fn fact_push(message: &str, datastore: Arc<RwLock<DataStore>>) -> Notification {
    if message.is_empty() {
        return Notification::basic(
            "This is not a fact (stare)",
            Color::Red,
            MessageFormat::Text,
        );
    }

    let entry = match Url::parse(message) {
        Ok(url) => DataEntry::Image(url.as_str().into()),
        Err(_) => DataEntry::Text(message.into()),
    };

    {
        let mut db = datastore.write().expect("Could not acquire write lock");
        db.push(entry);
        write_db(&*db);
    }

    Notification::basic("Pushed! (happyotter)", Color::Green, MessageFormat::Text)
}

pub fn fact_pop(datastore: Arc<RwLock<DataStore>>) -> Notification {
    let popped = {
        let mut db = datastore.write().expect("Could not acquire write lock");
        let item = db.pop();

        write_db(&*db);

        item
    };

    if let Some(msg) = popped {
        Notification::basic(
            format!(
                "Popped {} (happyotter)",
                match msg {
                    DataEntry::Image(url) => format!("image {}", url),
                    DataEntry::Text(msg) => format!("message {}", msg),
                }
            ),
            Color::Green,
            MessageFormat::Text,
        )
    } else {
        Notification::basic("Nothing to pop (stare)", Color::Red, MessageFormat::Text)
    }
}

pub fn open_db<P>(path: P) -> Arc<RwLock<DataStore>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let datastore = if !path.exists() {
        DataStore::new(path)
    } else {
        let file = File::open(path).expect("Could not open datastore file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Could not deserialize datastore file")
    };

    Arc::new(RwLock::new(datastore))
}

fn write_db(datastore: &DataStore) {
    let file = File::create(datastore.path()).expect("Could not open datastore file");
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, datastore).expect("Could not write out datastore");
}
