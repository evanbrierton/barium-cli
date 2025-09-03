

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use barium_core::BarKind as CoreBarKind;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum BarKind {
  Dumbbell,
  Barbell
}

impl FromStr for BarKind {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
          "d" | "db" | "dumbbell" => Ok(BarKind::Dumbbell),
          "b" | "bb" | "barbell" => Ok(BarKind::Barbell),
          _ => Err("Invalid bar kind".to_string()),
      }
  }
}

impl From<BarKind> for CoreBarKind {
  fn from(val: BarKind) -> Self {
      match val {
          BarKind::Dumbbell => CoreBarKind::Dumbbell,
          BarKind::Barbell => CoreBarKind::Barbell,
      }
  }
}
