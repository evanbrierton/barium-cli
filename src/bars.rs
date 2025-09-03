use barium_core::{Bar, BarKind};
use clap::Args;
use uom::si::rational64::{Length, Mass};

#[derive(Args, Copy,Clone)]
pub struct Bars {
    weight: Mass,
    gauge: Length,
    kind: BarKind,
    count: Option<usize>,
}

impl From<Bars> for (Bar, Option<usize>) {
    fn from(bars: Bars) -> Self {
        let bar = Bar::new(bars.weight, bars.gauge, bars.kind);
        (bar, bars.count)
    }
}

