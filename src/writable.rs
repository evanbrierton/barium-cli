use std::hash::Hash;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Writeable {
  type WriteType: Serialize + DeserializeOwned + Eq + Hash;
}
