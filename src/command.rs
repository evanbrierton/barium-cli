use barium_core::Requirement;
use clap::{Args, Subcommand};

use strum::Display;

use crate::{bars::Bars, plates::Plates};

#[derive(Subcommand, Clone)]
pub enum Command {
    /// List all possible weights for each type of bar
    Available,
    /// Create a workout plan based on the list of given requirements
    Workout(WorkoutCommand),
    /// Manage available plates
    Plates(ActionCommand<Plates>),
    /// Manage available bars
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
    /// Show all available items
    Show,
    /// Reset to an empty config
    Reset,
    /// Add an item to the config
    Add(T),
    /// Remove an item from the config, if no count is specified remove all matching items
    Remove(T),
}
