use std::{collections::HashMap, fs::{self, File}, hash::Hash, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};

use crate::command::Command;

const BARIUM_DIR: &str = "barium";
const JSON_EXT: &str = "json";

fn get_config_file_path(command: Command) -> PathBuf {
  let directory = dirs::config_dir().expect("Could not find config directory");
  let path = command.to_string().to_lowercase();

  directory.join(BARIUM_DIR).join(path).with_extension(JSON_EXT)
}

pub fn create_config_file(command: Command) {
  let path = get_config_file_path(command);

  fs::create_dir_all(path.parent().unwrap()).expect("Could not create config directory");

  if !path.exists() {
      File::create(&path).expect("Could not create config file");
  }
}

fn read_config_file_str (command: Command) -> String {
    let path = get_config_file_path(command);
    fs::read_to_string(&path).expect("Could not read config file")
}

pub fn read_config_file<T: DeserializeOwned + Eq + Hash>(command: Command) -> HashMap<T, usize> {
    let s = read_config_file_str(command);
    let data: Vec<(T, usize)> = serde_json::from_str(&s).unwrap_or_default();
    data.into_iter().collect()
}

pub fn reset_config_file<V: Serialize>(command: Command) {
    let path = get_config_file_path(command);
    let empty: Vec<V> = vec![];
    let s = serde_json::to_string_pretty(&empty).expect("Could not serialize empty config");
    fs::write(&path, s).expect("Could not write to config file");
}

pub fn write_config_file<T: Serialize>(command: Command, data: &HashMap<T, usize>) {
    let path = get_config_file_path(command);

    println!("Writing to config file: {:?}", path);

    let vec_data = data.iter().map(|(k, v)| (k, *v)).collect::<Vec<_>>();

    let s = serde_json::to_string_pretty(&vec_data).expect("Could not serialize config");
    fs::write(&path, s).expect("Could not write to config file");
}

pub fn add_to_config_file<T: Serialize + DeserializeOwned + Eq + Hash>(command: Command, (item, count): (T, Option<usize>)) {
    let mut data = read_config_file::<T>(command);

    let count = data.get(&item).copied().unwrap_or_default() + count.unwrap_or(1);
    data.insert(item, count);

    write_config_file(command, &data);
}

pub fn remove_from_config_file<T: Serialize + DeserializeOwned + Eq + Hash>(command: Command, (item, count): (T, Option<usize>)) {
    let mut data = read_config_file::<T>(command);

    match count {
        Some(count) => {
          let count = data.get(&item).copied().unwrap_or_default() - count;
          data.insert(item, count);
        }
        None => { data.remove(&item); },
    }

    write_config_file(command, &data);
}
