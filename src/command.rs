use barium_core::Requirement;
use clap::{Args, Subcommand};

use strum::Display;

use crate::{bars::Bars, plates::Plates};

#[derive(Subcommand, Clone)]
pub enum Command {
    Available,
    Workout(WorkoutCommand),
    Plates(ActionCommand<Plates>),
    Bars(ActionCommand<Bars>),
}

#[derive(Display, Copy, Clone)]
pub enum ObjectType {
    Plates,
    Bars,
}
#[derive(Args, Clone)]
pub struct WorkoutCommand {
    #[arg(value_parser = clap::value_parser!(Requirement))]
    pub requirements: Vec<Requirement>,
}

#[derive(Args, Copy, Clone)]
pub struct ActionCommand<T: Args> {
    #[command(subcommand)]
    pub action: ConfigAction<T>,
}

#[derive(Subcommand, Copy, Clone)]
pub enum ConfigAction<T: Args> {
    /// prints the current config
    Show,
    /// resets the config to default values
    Reset,
    /// sets a config value
    Add(T),
    /// removes a config value
    /// if multiple plates of the same weight exist, only one will be removed
    /// if no plate of the given weight exists, nothing will be removed
    /// if count is given, that many plates will be removed
    /// if count is greater than the number of plates of the given weight, all plates of that weight will be removed
    Remove(T),
}
