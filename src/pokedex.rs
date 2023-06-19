use iced::widget::{button, column, image, row, text, text_input, Container, Scrollable};
use iced::{Alignment, Element, Length, Padding, Sandbox};
use ureq::{self, serde_json};

use crate::pokemon::Pokemon;

#[derive(Clone, Default)]
pub struct Pokedex {
    query: String,
    pokemon_name: String,
    description: String,
    img_bytes: Vec<u8>,
    stats: Vec<i64>,
}
impl Sandbox for Pokedex {
    type Message = Message;

    fn new() -> Self {
        Self {
            stats: vec![0; 6],
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        "Simple Pokedex".to_string()
    }

    fn view(&self) -> Element<Message> {
        column![
            column![
                text("Pokedex")
                    .size(64)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center),
                Container::new(
                    text_input("Enter query...", &self.query)
                        .on_input(Message::QUERY)
                        .on_submit(Message::SEARCH)
                )
                .width(Length::Fill)
                .padding(Padding::from([0, 16])),
                button("Search").on_press(Message::SEARCH),
                image::viewer(image::Handle::from_memory(self.img_bytes.clone())),
                text(&self.pokemon_name).size(42),
                Scrollable::new(text(&self.description).size(32)),
            ]
            .align_items(Alignment::Center)
            .height(Length::FillPortion(2)),
            row![
                if !self.stats.iter().all(|x| *x == 0) {
                    column![
                        text(&format!("HP: {}", self.stats[0])),
                        text(&format!("ATK: {}", self.stats[1])),
                        text(&format!("DEF: {}", self.stats[2])),
                        text(&format!("SP. ATK: {}", self.stats[3])),
                        text(&format!("SP. DEF: {}", self.stats[4])),
                        text(&format!("SPEED: {}", self.stats[5])),
                    ]
                } else {
                    column![]
                }
                .align_items(Alignment::Center)
                .spacing(4),
                text("some sutf here later")
            ]
            .height(Length::FillPortion(1)),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::QUERY(input) => self.query = input,
            Message::SEARCH => {
                self.img_bytes = Vec::new();

                // For Pokemon Name, Desciption, and Sprite
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

                // For Pokemon Stats
                self.stats = vec![0; 6];
                let search_result2: serde_json::Value = ureq::get(&format!(
                    "https://pokeapi.co/api/v2/pokemon/{}",
                    self.query.to_lowercase()
                ))
                .call()
                .unwrap()
                .into_json()
                .unwrap();

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

                let stats = search_result2["stats"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|val| val.as_object().unwrap())
                    .collect::<Vec<_>>();
                // hp
                self.stats[0] = stats[0]
                    .get_key_value("base_stat")
                    .unwrap()
                    .1
                    .as_i64()
                    .unwrap();
                // attack
                self.stats[1] = stats[1]
                    .get_key_value("base_stat")
                    .unwrap()
                    .1
                    .as_i64()
                    .unwrap();
                // defense
                self.stats[2] = stats[2]
                    .get_key_value("base_stat")
                    .unwrap()
                    .1
                    .as_i64()
                    .unwrap();
                // special-attack
                self.stats[3] = stats[3]
                    .get_key_value("base_stat")
                    .unwrap()
                    .1
                    .as_i64()
                    .unwrap();
                // special-defense
                self.stats[4] = stats[4]
                    .get_key_value("base_stat")
                    .unwrap()
                    .1
                    .as_i64()
                    .unwrap();
                // speed
                self.stats[5] = stats[5]
                    .get_key_value("base_stat")
                    .unwrap()
                    .1
                    .as_i64()
                    .unwrap();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    QUERY(String),
    SEARCH,
}
