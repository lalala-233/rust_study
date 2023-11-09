use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct Text(Vec<String>);
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Content {
    paragraph: Vec<Text>,
}
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    content: Content,
}
fn main() {
    let text = Text(vec!["114".to_string(), "514".to_string()]);
    let content = Content {
        paragraph: vec![text; 2],
    };
    let config = Config { content };
    let x = toml::to_string(&config).unwrap();
    println!("{}", x);
}
