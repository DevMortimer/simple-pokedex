use iced::widget::{
    button, column, image, row, text, text_input, Container, ProgressBar, Scrollable, Space,
};
use iced::{theme::Palette, Alignment, Color, Element, Length, Padding, Sandbox, Theme};
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
                // The title
                text("POKEDEX")
                    .size(64)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .style(Color::from_rgb8(205, 65, 82)),
                // The text input
                Container::new(
                    text_input("Enter query...", &self.query)
                        .on_input(Message::QUERY)
                        .on_submit(Message::SEARCH)
                )
                .width(Length::Fill)
                .padding(Padding::from([0, 16])),
                Space::new(0, 8),
                // the search button
                button("Search").on_press(Message::SEARCH),
                // the sprite (it's draggable and zoomable)
                image::viewer(image::Handle::from_memory(self.img_bytes.clone())),
                // the pokemon name
                text(&self.pokemon_name)
                    .size(42)
                    .style(Color::from_rgb8(255, 189, 131)),
                // the description (it's scrollable)
                Scrollable::new(text(&self.description).size(32)),
            ]
            .align_items(Alignment::Center)
            .height(Length::FillPortion(2)),
            row![
                // the stats
                Container::new(Scrollable::new(
                    if !self.stats.iter().all(|x| *x == 0) {
                        column![
                            Container::new(text("STATS").style(Color::from_rgb8(238, 230, 164)))
                                .padding(Padding::from([0, 42])),
                            row![
                                text(&format!("HP: {}", self.stats[0]))
                                    .style(Color::from_rgb8(65, 180, 131)),
                                Space::new(2, 0),
                                ProgressBar::new(0.0..=255.0, self.stats[0] as f32).width(96),
                            ],
                            row![
                                text(&format!("ATK: {}", self.stats[1]))
                                    .style(Color::from_rgb8(222, 106, 98)),
                                Space::new(2, 0),
                                ProgressBar::new(0.0..=255.0, self.stats[1] as f32).width(96),
                            ],
                            row![
                                text(&format!("DEF: {}", self.stats[2]))
                                    .style(Color::from_rgb8(74, 115, 172)),
                                Space::new(2, 0),
                                ProgressBar::new(0.0..=255.0, self.stats[2] as f32).width(96),
                            ],
                            row![
                                text(&format!("SP. ATK: {}", self.stats[3]))
                                    .style(Color::from_rgb8(255, 197, 90)),
                                Space::new(2, 0),
                                ProgressBar::new(0.0..=255.0, self.stats[3] as f32).width(96),
                            ],
                            row![
                                text(&format!("SP. DEF: {}", self.stats[4]))
                                    .style(Color::from_rgb8(98, 98, 123)),
                                Space::new(2, 0),
                                ProgressBar::new(0.0..=255.0, self.stats[4] as f32).width(96),
                            ],
                            row![
                                text(&format!("SPEED: {}", self.stats[5]))
                                    .style(Color::from_rgb8(238, 148, 115)),
                                Space::new(2, 0),
                                ProgressBar::new(0.0..=255.0, self.stats[5] as f32).width(96),
                            ]
                        ]
                    } else {
                        column![]
                    }
                    .align_items(Alignment::End)
                    .spacing(4)
                ))
                .center_x()
                .center_y()
                .width(Length::FillPortion(1)),
                // the moves
                Container::new(text("some sutf here later"))
                    .center_x()
                    .center_y()
                    .width(Length::FillPortion(1))
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
                if !self.query.is_empty() {
                    self.img_bytes = Vec::new();
                    self.stats = vec![0; 6];

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

                        // For Pokemon Stats
                        let search_result2: serde_json::Value = ureq::get(&format!(
                            "https://pokeapi.co/api/v2/pokemon/{}",
                            self.query.to_lowercase()
                        ))
                        .call()
                        .unwrap()
                        .into_json()
                        .unwrap();

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
    }

    fn theme(&self) -> Theme {
        Theme::custom(Palette {
            background: Color::from_rgb8(16, 32, 57),
            text: Color::from_rgb8(222, 230, 238),
            primary: Color::from_rgb8(57, 139, 49),
            success: Color::from_rgb8(180, 255, 131),
            danger: Color::from_rgb8(197, 65, 65),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    QUERY(String),
    SEARCH,
}
