#![warn(clippy::pedantic)]

use std::hash::BuildHasher;
use std::{collections::HashMap, iter::repeat_n};

use anyhow::Ok;
use clap::{Args, FromArgMatches, Parser, Subcommand};
use itertools::Itertools;
use barium_core::{
    Bar, Gym, Plate, BarKind
};

mod bar_kind;
mod requirement;
mod plates;
mod bars;
mod command;
mod execute;
mod files;
mod count;
mod units;
mod writable;

use crate::requirement::Requirement;
use crate::command::Command;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(Requirement))]
    requirements: Vec<Requirement>,

    #[command(subcommand)]
    command: Command,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    execute::execute(args.command);

    println!("{}", args.command.to_string().to_lowercase());

    // process_bars(&[], &[], &args.requirements)?;

    Ok(())
}

#[must_use] pub fn from_weights_map<S: BuildHasher>(weights_map: HashMap<Plate, usize, S>) -> Vec<Plate> {
    weights_map
        .into_iter()
        .flat_map(|(plate, count)| repeat_n(plate, count))
        .collect()
}

fn process_bars(
    plates: &[Plate],
    bars: &[Bar],
    requirements: &[Requirement],
) -> anyhow::Result<()> {
    let gym = Gym::new(plates, bars);

    if requirements.is_empty() {
        let weights = gym.weights();

        println!("Available weights:");
        for (bar, weights) in weights.into_iter().sorted() {
            println!(
                "{}: {:?}",
                bar,
                weights
                    .iter()
                    .map(|w| f64::from(*w) / 1000.0)
                    .collect::<Vec<_>>()
            );
        }
    } else {
        let workout = gym.workout(&requirements.iter().map(|Requirement(r)| *r).collect::<Vec<_>>())?;
        for (bar, dumbbells) in workout {
            println!("{bar}");
            for dumbbell in dumbbells {
                println!("  - {dumbbell}");
            }
        }
    }

    Ok(())
}
