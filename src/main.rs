#[macro_use]
extern crate clap;

use clap::App;
use guessing_game::guess;
use hello::hello;
use functions::functions;
use temperature_converter::convert;

fn main() {
    let cli_config = load_yaml!("../config/cli_config.yml");
    let matches = App::from_yaml(cli_config).get_matches();

    match matches.subcommand() {
        ("hello", Some(_)) => hello(),
        ("guessing-game", Some(_)) => guess(),
        ("functions", Some(_)) => functions(),
        ("temperature-converter", Some(temperature_matches)) => {
            if temperature_matches.is_present("from-fahrenheit") {
                let temperature: f32 = temperature_matches.value_of("from-fahrenheit").unwrap().parse().unwrap();
                convert(temperature, "fahrenheit");
            } else if temperature_matches.is_present("from-celsius") {
                let temperature: f32 = temperature_matches.value_of("from-celsius").unwrap().parse().unwrap();
                convert(temperature, "celsius");
            }
        }
        _ => unreachable!()
    }
}

// TODO: FIBONACCI_GENERATOR        (Remember to add to CLI)
// TODO: TEMPERATURE_CONVERTER      (Remember to add to CLI)
// TODO: TWELVE_DAYS_OF_CHRISTMAS   (Remember to add to CLI)