use barium_core::Plate;
use clap::Args;
use uom::si::rational64::{Length, Mass};

#[derive(Args, Copy, Clone, Debug)]
pub struct Plates {
    weight: Mass,
    gauge: Length,
    count: Option<usize>,
}

impl From<Plates> for (Plate, Option<usize>) {
    fn from(plates: Plates) -> Self {
        let plate = Plate::new(plates.weight, plates.gauge);

        (plate, plates.count)
    }
}
