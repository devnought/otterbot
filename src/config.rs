use message::Message;
use serde_json;
use std::{collections::BTreeMap, fs::File, io, path::Path};

#[derive(Deserialize, Debug)]
pub struct Config {
    host: String,
    port: u32,
    commands: BTreeMap<String, String>,
}

impl Config {
    pub fn load<P>(path: P) -> Result<Self, LoadConfigError>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn parse_command_message(&self, message: &str) -> Option<Message> {
        let mut iter = message.split(' ');

        if iter.next()? != "/otterbot" {
            return None;
        }

        let key = loop {
            match iter.next() {
                Some("") => continue,
                Some("help") | None => return Some(Message::Text(self.help_message())),
                Some(k) => break k.to_lowercase(),
            }
        };

        let value = match self.commands.get(&key) {
            Some(k) => k,
            None => return Some(Message::Error),
        };

        Some(Message::Image(value.clone()))
    }

    pub fn help_message(&self) -> String {
        self.commands
            .keys()
            .fold(String::from("/code "), |acc, key| acc + key + "\n")
    }
}

#[derive(Debug)]
pub enum LoadConfigError {
    Io(io::Error),
    Deserialize(serde_json::Error),
}

impl From<io::Error> for LoadConfigError {
    fn from(error: io::Error) -> Self {
        LoadConfigError::Io(error)
    }
}

impl From<serde_json::Error> for LoadConfigError {
    fn from(error: serde_json::Error) -> Self {
        LoadConfigError::Deserialize(error)
    }
}

#[cfg(test)]
mod tests {
    use config::Config;
    use message::Message;
    use std::collections::BTreeMap;

    fn build_config() -> Config {
        let mut commands = BTreeMap::new();
        commands.insert(String::from("dance"), String::from("dance.gif"));

        Config {
            host: String::from("hostman.com"),
            port: 8000,
            commands,
        }
    }

    #[test]
    fn parse_cmd_basic() {
        let config = build_config();
        let res = config.parse_command_message("/otterbot dance");
        assert_eq!(Some(Message::Image(String::from("dance.gif"))), res);
    }

    #[test]
    fn parse_cmd_basic_case() {
        let config = build_config();
        let res = config.parse_command_message("/otterbot DaNcE");
        assert_eq!(Some(Message::Image(String::from("dance.gif"))), res);
    }

    #[test]
    fn parse_cmd_complicated() {
        let config = build_config();
        let res = config.parse_command_message("I think we should go to the /otterbot dance party!");

        assert_eq!(None, res);
    }

    #[test]
    fn parse_cmd_multi() {
        let config = build_config();
        let res = config.parse_command_message("/otterbot dance sad fuckyeah");
        assert_eq!(Some(Message::Image(String::from("dance.gif"))), res);
    }

    #[test]
    fn parse_cmd_lots_whitespace() {
        let config = build_config();
        let res = config.parse_command_message("/otterbot      dance     ");
        assert_eq!(Some(Message::Image(String::from("dance.gif"))), res);
    }

    #[test]
    fn parse_cmd_implicit_help() {
        let config = build_config();
        let res = config.parse_command_message("/otterbot");
        assert_eq!(Some(Message::Text(String::from("/code dance\n"))), res);
    }

    #[test]
    fn parse_cmd_implicit_help_whitespace() {
        let config = build_config();
        let res = config.parse_command_message("/otterbot            ");
        assert_eq!(Some(Message::Text(String::from("/code dance\n"))), res);
    }

    #[test]
    fn parse_cmd_explicit_help() {
        let config = build_config();
        let res = config.parse_command_message("/otterbot help");
        assert_eq!(Some(Message::Text(String::from("/code dance\n"))), res);
    }
}
