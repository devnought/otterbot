#[derive(Debug, PartialEq)]
pub enum Message {
    Error,
    Image(String),
    Text(String),
}
