use iced::widget::{button, column, image, row, text, text_input};
use iced::{Alignment, Element, Sandbox};
use ureq::{self, serde_json};

use crate::pokemon::Pokemon;

#[derive(Clone, Default)]
pub struct Pokedex {
    query: String,
    pokemon_name: String,
    description: String,
    img_bytes: Vec<u8>,
}
impl Sandbox for Pokedex {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Simple Pokedex".to_string()
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Pokedex")
                .size(64)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
            text_input("Enter query...", &self.query).on_input(Message::QUERY),
            button("Search").on_press(Message::SEARCH),
            image::viewer(image::Handle::from_memory(self.img_bytes.clone())),
            text(&self.pokemon_name).size(42),
            text(&self.description).size(32),
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::QUERY(input) => self.query = input,
            Message::SEARCH => {
                self.img_bytes = Vec::new();
                let search_result: Pokemon = match ureq::get(&format!(
                    "https://pokeapi.co/api/v2/pokemon-species/{}",
                    self.query.to_lowercase()
                ))
                .call()
                {
                    Ok(res) => {
                        self.description = String::new();
                        self.pokemon_name = String::new();
                        res.into_json().unwrap()
                    }
                    Err(_) => {
                        self.pokemon_name = String::new();
                        self.description = String::from("ERROR: Unable to find pokemon.");
                        Pokemon::default()
                    }
                };

                if !search_result.name.is_empty() {
                    let img_res = ureq::get(&format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", search_result.id)).call().unwrap();
                    img_res
                        .into_reader()
                        .read_to_end(&mut self.img_bytes)
                        .unwrap();

                    self.pokemon_name = search_result
                        .name
                        .split_whitespace()
                        .map(|s| {
                            let mut c = s.chars();
                            match c.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ");
                    self.description = search_result
                        .flavor_text_entries
                        .iter()
                        .find(|e| e.language.name == "en")
                        .unwrap()
                        .flavor_text
                        .chars()
                        .map(|c| if c == '\n' { ' ' } else { c })
                        .collect::<String>();
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    QUERY(String),
    SEARCH,
}
