use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use clap::Parser;
use colored::*;
use weather_cli::Cli::Cli;
use weather_cli::TextMethod;
use weather_cli::Weather::{Weather::*, WeatherResponse::*};
use weather_cli::{Api, AsciiArt, WeatherIcons};

const API_KEY: &'static str = "e5c9e66275afa5767ef301b14caaca58";
const API_KEY1: &'static str = "4b4cefefd96ec04cd7e24cc254f85c9e";

async fn CliRunTest() {
    let args = Cli::Args::parse();
    println!("{:?}", args);

    if !args.location().is_none() {
        let location_city = args.location().unwrap();
        let location_city = TextMethod::convert_to_title_case(&location_city);
        RunAppLocation(&location_city, None).await;
        println!("{}", location_city);
    } else if !args.sub_command().is_none() {
        match args.sub_command().unwrap() {
            Cli::SubCommands::Current(current_args) => {
                let mut units: Option<u8> = None;
                if !current_args.units().is_none() {
                    let units_args = current_args.units().unwrap().trim().parse::<u8>();
                    if units_args.is_ok() {
                        match units_args.unwrap() {
                            0 => {
                                units = Some(0);
                            }
                            1 => {
                                units = Some(1);
                            }
                            2 => {
                                units = Some(2);
                            }
                            _ => {
                                println!("Weather Units out of range (0,1,2)");
                            }
                        }
                    }
                }
                if !current_args.location().is_none() {
                    let city = current_args.location().unwrap();
                    println!("current: {}", city);
                    RunAppLocation(city, units).await;
                }
            }
            Cli::SubCommands::ForecastArgs(forecast_args) => {
                let check_city = if !forecast_args.location().is_none() {
                    Some(forecast_args.location().unwrap())
                } else {
                    None
                };
                println!("{:?}", check_city);
                let check_days = forecast_args.days();
                println!("days: {:?}", check_days);
            }
        }
        println!("Sub Command");
    } else {
        println!(" [-] That's not a valid command - use the help command if you are stuck.");
    }
}

async fn RunAppLocation(city: &str, units: Option<u8>) {
    //let city = "Phnom Penh";
    let mut weather_data = Api::OpenWeatherApiUrl::new(city, API_KEY1);

    //let response = fetch_weather(Api::other_api, city, "en").await;
    let response = fetch_weather(&weather_data.get_url()).await;
    if response.is_ok() {
        let weather_data_response = response.unwrap();
        print_title(city /*&weather_data_response*/).await;
        print_weather_info(&weather_data_response, &weather_data);
        //println!("{:#?}", weather_data_response);
    } else {
        println!("Weather request error");
    }
}
async fn print_title(city: &str /*weather_response: &WeatherResponse*/) {
    let ascii_art_text = AsciiArt::ascii_art(city).await;
    if !ascii_art_text.is_none() {
        println!(" {}", ascii_art_text.unwrap().magenta().blink());
        //println!(" - {}", weather_response.sys().country());
    }
    println!();
}

fn print_weather_info(weather_response: &WeatherResponse, weather_data: &Api::OpenWeatherApiUrl) {
    let temp_mode = if weather_data.units().is_none() {
        "K"
    } else {
        weather_data.units().as_ref().unwrap().get_prefix()
    };
    let weather_icon = WeatherIcons::get_weather_icon(weather_response.weather()[0].icon());
    let weather_icon = if !weather_icon.is_none() {
        weather_icon.unwrap()
    } else {
        " "
    };
    let time = DateTime::from_timestamp(weather_response.timezone() as i64, 25200);
    let time = if !time.is_none() {
        time.unwrap().to_string()
    } else {
        " ".to_string()
    };
    let time = Local::now();

    println!(" |-| 🌎\t {}\t\t{}", weather_response.sys().country(), time);
    println!(
        " |-| 🐡\t Description: \t{}  {}",
        weather_response.weather()[0].description(),
        weather_icon
    );
    println!(
        " |-| ☀️\t Temperature: \t{} {}",
        weather_response.main().temp().to_string().yellow().bold(),
        temp_mode.yellow().bold()
    );
    /*
    println!(
        " |-| Feels Like: \t{} {}",
        weather_response.main().feels_like(),
        temp_mode
    );
    */
    /*
    println!(" |-| Pressure: \t{}", weather_response.main().pressure());
    */
    println!(
        " |-| 🌊\t Humidity: \t{} {}",
        weather_response.main().humidity().to_string().cyan().bold(),
        "%".bold().cyan()
    );
    println!(" |-| 📈\t High: \t");
    println!(" |-| 📉\t Low: \t");
}

#[tokio::main]
async fn main() {
    //RunApp("haha").await
    CliRunTest().await;
}
