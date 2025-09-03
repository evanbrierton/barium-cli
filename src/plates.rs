use barium_core::Plate;
use clap::Args;
use derivative::Derivative;
use proc_macros::Count;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{count::Count, units, writable::Writeable};


#[derive(Args, Copy, Clone, Deserialize, Count, Derivative)]
#[derivative(PartialEq)]
pub struct Plates {
  weight: f64,
  gauge: u32,
  #[derivative(PartialEq="ignore")]
  count: Option<usize>,
}

impl Into<(Plate, Option<usize>)> for Plates {
  fn into(self) -> (Plate, Option<usize>) {
    let weight = units::kgs_to_grams(self.weight);
    let plate = Plate::new(weight, self.gauge);

    (plate, self.count)
  }
}

impl Writeable for Plates {
  type WriteType = Plate;
}
