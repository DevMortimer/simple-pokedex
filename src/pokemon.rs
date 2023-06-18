use iced::widget::image;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub name: String,
    // _url: String,
}

#[derive(Deserialize, Debug)]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: Entry,
    // _version: Entry,
}

#[derive(Deserialize, Debug, Default)]
pub struct Pokemon {
    pub name: String,
    pub id: i32,
    pub flavor_text_entries: Vec<FlavorText>,
}
