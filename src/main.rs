mod pokedex;
mod pokemon;

use iced::{window, Application, Settings};
use pokedex::Pokedex;

fn main() -> iced::Result {
    Pokedex::run(Settings {
        window: window::Settings {
            size: (512, 768),
            position: window::Position::Centered,
            min_size: Some((512, 768)),
            // icon: todo!(),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}
