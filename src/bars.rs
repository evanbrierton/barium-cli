use barium_core::Bar;
use clap::Args;
use derivative::Derivative;
use proc_macros::Count;
use serde::{Deserialize, Serialize};

use crate::{bar_kind::BarKind, count::Count, units, writable::Writeable};

#[derive(Args, Copy, Clone, Serialize, Deserialize, Count, Derivative)]
#[derivative(PartialEq)]
pub struct Bars {
  weight: f64,
  gauge: u32,
  kind: BarKind,
  #[derivative(PartialEq="ignore")]
  count: Option<usize>,
}


impl From<Bars> for (Bar, Option<usize>) {
  fn from(bars: Bars) -> Self {
    let weight = units::kgs_to_grams(bars.weight);
    let bar = Bar::new(weight, bars.gauge, bars.kind.into());

    (bar, bars.count)
  }
}

impl Writeable for Bars {
  type WriteType = Bar;
}
