use std::{
    collections::HashMap,
    fs::{self, File},
    hash::Hash,
    path::PathBuf,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::command::ObjectType;

const BARIUM_DIR: &str = "barium";
const JSON_EXT: &str = "json";

fn get_config_file_path(object_type: ObjectType) -> PathBuf {
    let directory = dirs::config_dir().expect("Could not find config directory");
    let path = object_type.to_string().to_lowercase();

    directory
        .join(BARIUM_DIR)
        .join(path)
        .with_extension(JSON_EXT)
}

pub fn create_config_file(object_type: ObjectType) {
    let path = get_config_file_path(object_type);

    fs::create_dir_all(path.parent().unwrap()).expect("Could not create config directory");

    if !path.exists() {
        File::create(&path).expect("Could not create config file");
    }
}

fn read_config_file_str(object_type: ObjectType) -> String {
    let path = get_config_file_path(object_type);
    fs::read_to_string(&path).expect("Could not read config file")
}

pub fn read_config_file<T: DeserializeOwned + Eq + Hash>(
    object_type: ObjectType,
) -> HashMap<T, usize> {
    let s = read_config_file_str(object_type);
    let data: Vec<(T, usize)> = serde_json::from_str(&s).unwrap_or_default();
    data.into_iter().collect()
}

pub fn reset_config_file<V: Serialize>(object_type: ObjectType) {
    let path = get_config_file_path(object_type);
    let empty: Vec<V> = vec![];
    let s = serde_json::to_string_pretty(&empty).expect("Could not serialize empty config");
    fs::write(&path, s).expect("Could not write to config file");
}

pub fn write_config_file<T: Serialize>(object_type: ObjectType, data: &HashMap<T, usize>) {
    let path = get_config_file_path(object_type);

    println!("Writing to config file: {}", path.display());

    let vec_data = data.iter().map(|(k, v)| (k, *v)).collect::<Vec<_>>();

    let s = serde_json::to_string_pretty(&vec_data).expect("Could not serialize config");
    fs::write(&path, s).expect("Could not write to config file");
}

pub fn add_to_config_file<T: Serialize + DeserializeOwned + Eq + Hash>(
    object_type: ObjectType,
    (item, count): (T, Option<usize>),
) {
    let mut data = read_config_file::<T>(object_type);

    let count = data.get(&item).copied().unwrap_or_default() + count.unwrap_or(1);
    data.insert(item, count);

    write_config_file(object_type, &data);
}

pub fn remove_from_config_file<T: Serialize + DeserializeOwned + Eq + Hash>(
    object_type: ObjectType,
    (item, count): (T, Option<usize>),
) {
    let mut data = read_config_file::<T>(object_type);

    match count {
        Some(count) => {
            let count = data.get(&item).copied().unwrap_or_default() - count;
            data.insert(item, count);
        }
        None => {
            data.remove(&item);
        }
    }

    write_config_file(object_type, &data);
}
