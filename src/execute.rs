use std::{fmt::Display, hash::Hash};

use barium_core::{Bar, Plate};
use clap::Args;
use serde::{de::DeserializeOwned, Serialize};

use crate::{bars::Bars, command::{ActionCommand, Command, ConfigAction}, files, plates::Plates};

pub fn execute(command: Command) {
    match command {
        Command::Bars(action) => execute_action_command::<Bar, Bars>(command, &action),
        Command::Plates(action) => execute_action_command::<Plate, Plates>(command, &action),
    }
}

fn execute_action_command<V: Serialize + DeserializeOwned + Eq + Hash + Display, T: Clone + Args + Into<(V, Option<usize>)>>(command: Command, action: &ActionCommand<T>) {
    files::create_config_file(command);

    match &action.action {
        ConfigAction::Show => {
            let data = files::read_config_file::<V>(command);

            println!("{command}");
            for (item, count) in data {
                 println!("- {item} (x{count})");
            }
        }
        ConfigAction::Reset => {
            files::reset_config_file::<V>(command);
        }
        ConfigAction::Add(item) => {
            files::add_to_config_file::<V>(command, item.clone().into());
        }
        ConfigAction::Remove(item) => {
            files::remove_from_config_file(command, item.clone().into());
        }
    }
}
