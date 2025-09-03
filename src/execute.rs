use std::{collections::HashMap, fmt::Display, hash::Hash, iter::repeat_n};

use barium_core::{Gym, Requirement};
use clap::Args;
use itertools::Itertools;
use serde::{Serialize, de::DeserializeOwned};
use uom::si::{mass::kilogram, quantities::Mass};

use crate::{
    command::{ActionCommand, Command, ConfigAction, ObjectType},
    files,
};

pub fn execute(command: Command) {
    match command {
        Command::Workout(workout_command) => {
            execute_workout_command(&workout_command.requirements).unwrap();
        }
        Command::Available => execute_available_command(),
        Command::Bars(action) => execute_action_command(ObjectType::Bars, &action),
        Command::Plates(action) => execute_action_command(ObjectType::Plates, &action),
    }
}

fn build_gym() -> Gym {
    let plates = expand(files::read_config_file(ObjectType::Plates));
    let bars = expand(files::read_config_file(ObjectType::Bars));

    Gym::new(&plates, &bars)
}

fn expand<T: Clone>(item: HashMap<T, usize>) -> Vec<T> {
    item.into_iter()
        .flat_map(|(obj, count)| repeat_n(obj, count))
        .collect()
}

fn execute_workout_command(requirements: &[Requirement]) -> Result<(), anyhow::Error> {
    let gym = build_gym();
    let workout = gym.workout(requirements)?;

    println!("Workout:");
    for (bar, dumbbells) in workout {
        println!("{bar}");
        for dumbbell in dumbbells {
            println!("  - {dumbbell}");
        }
    }

    Ok(())
}

fn execute_available_command() {
    let gym = build_gym();
    let weights = gym.weights();

    for (bar, weights) in weights.into_iter().sorted() {
        println!(
            "{}: {:?}",
            bar,
            weights
                .iter()
                .map(Mass::get::<kilogram>)
                .collect::<Vec<_>>()
        );
    }
}

fn execute_action_command<
    V: Serialize + DeserializeOwned + Eq + Hash + Display + Ord,
    T: Clone + Args + Into<(V, Option<usize>)>,
>(
    object_type: ObjectType,
    action: &ActionCommand<T>,
) {
    files::create_config_file(object_type);

    match &action.action {
        ConfigAction::Show => {
            let data = files::read_config_file::<V>(object_type);
            let sorted = data.into_iter().sorted_by(|(a, _), (b, _)| a.cmp(b));

            println!("{object_type}");
            for (item, count) in sorted {
                println!("- {item} (x{count})");
            }
        }
        ConfigAction::Reset => {
            files::reset_config_file::<V>(object_type);
        }
        ConfigAction::Add(item) => {
            files::add_to_config_file::<V>(object_type, item.clone().into());
        }
        ConfigAction::Remove(item) => {
            files::remove_from_config_file(object_type, item.clone().into());
        }
    }
}
