use iced::widget::{
    button, column, image, row, text, text_input, Container, ProgressBar, Scrollable, Space,
};
use iced::{theme::Palette, Alignment, Color, Element, Length, Padding, Sandbox, Theme};
use textdistance::str::damerau_levenshtein;
use ureq::{self, serde_json};

use crate::pokemon::Pokemon;

macro_rules! stat {
    ($name:expr => $color:expr => $value:expr) => {
        row![
            text(&format!("{}: {}", $name, $value)).style($color),
            Space::new(2, 0),
            ProgressBar::new(0.0..=255.0, $value as f32).width(96),
        ]
    };
}
macro_rules! stat_value {
    ($stat_vec:expr, $index:expr) => {
        $stat_vec[$index]
            .get_key_value("base_stat")
            .unwrap()
            .1
            .as_i64()
            .unwrap()
    };
}

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
        let title = text("POKEDEX")
            .size(64)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .style(Color::from_rgb8(205, 65, 82));
        let query_input = Container::new(
            text_input("Enter query...", &self.query)
                .on_input(Message::QUERY)
                .on_submit(Message::SEARCH),
        )
        .width(Length::Fill)
        .padding(Padding::from([0, 16]));
        let search_button = button("Search").on_press(Message::SEARCH);
        let sprite_img = image::viewer(image::Handle::from_memory(self.img_bytes.clone()));
        let pokemon_name = text(&self.pokemon_name)
            .size(42)
            .style(Color::from_rgb8(255, 189, 131));
        let pokemon_desc = Scrollable::new(text(&self.description).size(32));

        column![
            column![
                title,
                query_input,
                Space::new(0, 8),
                search_button,
                sprite_img,
                pokemon_name,
                pokemon_desc,
            ]
            .align_items(Alignment::Center)
            .height(Length::FillPortion(2)),
            row![
                // the stats
                Container::new(Scrollable::new(
                    if !self.stats.iter().all(|x| *x == 0) {
                        let stats_title =
                            Container::new(text("STATS").style(Color::from_rgb8(238, 230, 164)))
                                .padding(Padding::from([0, 48]));

                        let stats = column![
                            stats_title,
                            stat!("HP" => Color::from_rgb8(65, 180, 131) => self.stats[0]),
                            stat!("ATK" => Color::from_rgb8(222, 106, 98) => self.stats[1]),
                            stat!("DEF" => Color::from_rgb8(74, 115, 172) => self.stats[2]),
                            stat!("SP. ATK" => Color::from_rgb8(255, 197, 90) => self.stats[3]),
                            stat!("SP. DEF" => Color::from_rgb8(98, 98, 123) => self.stats[4]),
                            stat!("SPEED" => Color::from_rgb8(238, 148, 115) => self.stats[5]),
                        ];

                        stats
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
            Message::QUERY(input) => self.query = input.to_lowercase(),
            Message::SEARCH => {
                if !self.query.is_empty() {
                    self.img_bytes = Vec::new();
                    self.stats = vec![0; 6];

                    // For Pokemon Name, Desciption, and Sprite
                    let mut search_result: Pokemon = Pokemon::default();
                    while search_result.name.is_empty() {
                        let search_query =
                            &format!("https://pokeapi.co/api/v2/pokemon-species/{}", self.query);
                        let req = ureq::get(search_query).call();
                        self.description = String::new();
                        self.pokemon_name = String::new();
                        if req.is_ok() {
                            search_result = req.unwrap().into_json().unwrap();
                        } else {
                            // if there is no match, then change the query
                            // then find the closest one and use that instead
                            let list_of_species: serde_json::Value = ureq::get(
                                "https://pokeapi.co/api/v2/pokemon-species?offset=0&limit=1010",
                            )
                            .call()
                            .unwrap()
                            .into_json()
                            .unwrap();

                            let list_of_names = list_of_species["results"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|val| val.as_object().unwrap()["name"].as_str().unwrap())
                                .collect::<Vec<&str>>();
                            let list_of_comparisons = (0..1010)
                                .map(|i| damerau_levenshtein(&self.query, list_of_names[i]))
                                .collect::<Vec<usize>>();
                            let (min_index, _) = list_of_comparisons
                                .iter()
                                .enumerate()
                                .min_by(|(_, a), (_, b)| a.cmp(b))
                                .unwrap();

                            self.query = list_of_names[min_index].to_string();
                        }
                    }

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

                    self.stats[0] = stat_value!(stats, 0); // hp
                    self.stats[1] = stat_value!(stats, 1); // atk
                    self.stats[2] = stat_value!(stats, 2); // def
                    self.stats[3] = stat_value!(stats, 3); // sp. atk
                    self.stats[4] = stat_value!(stats, 4); // sp. def
                    self.stats[5] = stat_value!(stats, 5); // speed
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
