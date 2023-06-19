mod pokedex;
mod pokemon;

use iced::{window, Application, Settings};
use pokedex::Pokedex;

fn main() -> iced::Result {
    Pokedex::run(Settings {
        window: window::Settings {
            size: (512, 768),
            position: window::Position::Centered,
            min_size: Some((256, 384)),
            // icon: todo!(),
            platform_specific: window::PlatformSpecific,
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}
