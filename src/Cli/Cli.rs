use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, subcommand_required=false)]
pub struct Args {
    #[command(subcommand)]
    sub_command: Option<SubCommands>,
    #[clap(
        short,
        long,
        required = false,
        value_name = "Location",
        help = "Specify the location"
    )]
    location: Option<String>,
}

impl Args {
    pub fn location(&self) -> Option<&str> {
        if !self.location.is_none() {
            Some(self.location.as_ref().unwrap())
        } else {
            None
        }
    }
    pub fn sub_command(&self) -> Option<&SubCommands> {
        if !self.sub_command.is_none() {
            Some(self.sub_command.as_ref().unwrap())
        } else {
            None
        }
    }
}

#[derive(Parser, Debug)]
pub struct CurrentArgs {
    #[clap(
        short,
        long,
        required = true,
        value_name = "Location",
        help = "Specify the location"
    )]
    location: Option<String>,
    #[clap(
        short,
        long,
        required = false,
        value_name = "Temperature Unit",
        help = "Specify the units of measurement, 0--°C, 1--°F, 2--K"
    )]
    units: Option<String>,
    #[clap(short, long, help = "Display additional information")]
    verbose: Option<String>,
}

impl CurrentArgs {
    pub fn location(&self) -> Option<&str> {
        if !self.location.is_none() {
            Some(self.location.as_ref().unwrap())
        } else {
            None
        }
    }
    pub fn units(&self) -> Option<&str> {
        if !self.units.is_none() {
            Some(self.units.as_ref().unwrap())
        } else {
            None
        }
    }
}

#[derive(Parser, Debug)]
pub struct ForecastArgs {
    #[clap(
        short,
        long,
        required = true,
        value_name = "Location",
        help = "Specify the location"
    )]
    location: Option<String>,
    #[clap(
        short,
        long,
        required = false,
        value_name = "Temperature Unit",
        help = "Specify the units of measurement"
    )]
    units: Option<String>,
    #[clap(
        short,
        long,
        required = true,
        value_name = "Days",
        help = "Specify the number of days for the forecast"
    )]
    days: Option<String>,
    #[clap(short, long, help = "Display additional information")]
    verbose: Option<String>,
}

impl ForecastArgs {
    pub fn location(&self) -> Option<&str> {
        if self.location.is_none() {
            None
        } else {
            Some(self.location.as_ref().unwrap())
        }
    }
    pub fn units(&self) -> Option<&str> {
        if self.units.is_none() {
            None
        } else {
            Some(self.units.as_ref().unwrap())
        }
    }
    pub fn days(&self) -> Option<u16> {
        if !self.days.is_none() {
            let days_parse = self.days.as_ref().unwrap().trim().parse::<u16>();
            if days_parse.is_ok() {
                return Some(days_parse.unwrap());
            }
        }
        None
    }
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    #[command(name = "current", about = "Get the current weather conditions")]
    Current(CurrentArgs),
    #[command(name = "forecast", about = "Get the weather forecast")]
    ForecastArgs(ForecastArgs),
}
