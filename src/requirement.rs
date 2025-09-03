use std::str::FromStr;


use barium_core::Requirement as CoreRequirement;
use crate::{bar_kind::BarKind, units};

#[derive(Debug, Clone, Copy)]
pub struct Requirement(pub CoreRequirement);

impl FromStr for Requirement {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      let (weight, bar_kind) = s.split_at(s.len() - 1);
      let weight = weight
          .parse::<f64>()
          .map(units::kgs_to_grams)
          .map_err(|_| "Invalid weight".to_string())?;

      let bar_kind = BarKind::from_str(bar_kind.to_lowercase().as_str())?;

      Ok(Requirement(CoreRequirement::new(weight, bar_kind.into())))
  }
}
