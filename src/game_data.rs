use std::io::{self, Write};

pub struct Tech {
  pub chips: u64,
  pub silicon: u64,
  pub money: f32,
  pub engineers: u64, // determines performance
  pub fabs: u32, //  determines yield and die count
}

impl Tech {
  pub fn new() -> Self {
    Self {
      chips: 1000,
      silicon: 500,
      money: 1000.0,
      engineers: 10,
      fabs: 0,
    }
  }
}

pub struct TickRates {
  pub fab_tick: i16,
  pub sell_tick: i8,
}

impl TickRates {
  pub fn new() -> Self {
    Self {
      fab_tick: 1000,
      sell_tick: 10,
    }
  }
}

pub struct Stats {
  pub node: u8, // process node: higher is better
  pub chip_price: f32, // chip_price = chip_performance / chip_yield;
  pub chip_yield: f32,
  pub chip_performance: f32,
  pub chip_price_multiplier: f32,
  pub silicon_cost: f32,
  pub silicon_cost_multiplier: f32,
  pub wafer_die_count: u16,
}

impl Stats {
  pub fn new() -> Self {
    Self {
      node: 0,
      chip_price: 1.0,
      chip_yield: 0.5,
      chip_performance: 1.0,
      chip_price_multiplier: 1.5,
      silicon_cost: 10.0,
      silicon_cost_multiplier: 1.0,
      wafer_die_count: 100,
    }
  }
}

pub struct Upgrades {
  pub fuck: u8,
}

impl Upgrades {
  pub fn new() -> Self {
    Self {
      fuck: 0,
    }
  }
}

pub struct GameData {
  pub tech: Tech,
  pub ticks: TickRates,
  pub stats: Stats,
  pub upgrades: Upgrades,
}

pub fn game_data_init() -> GameData {
  GameData {
    tech: Tech::new(),
    ticks: TickRates::new(),
    stats: Stats::new(),
    upgrades: Upgrades::new(),
  }
}