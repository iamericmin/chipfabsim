use std::io::{self, stdout};
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use game_data::{GameData, game_data_init};

use crate::Action::{BuySilicon, MakeChip, BuyFab, QuitGame, Undefined};
use crate::Window::{Tech, Stats, Upgrades, Stocks};

pub enum Action {
  MakeChip,
  BuySilicon,
  BuyFab,
  QuitGame,
  Undefined
}

#[derive(PartialEq, Eq, Debug)]
pub enum Window {
    Tech,
    Stats,
    Upgrades,
    Stocks,
}

// process nodes
pub enum Nodes {
    
}

mod graphics;
mod game_data;

fn key_to_action(c: KeyCode) -> Action {
    match c {
        KeyCode::Char('m') => MakeChip,
        KeyCode::Char('b') => BuySilicon,
        KeyCode::Char('p') => BuyFab,
        KeyCode::Char('q') => QuitGame,
        _ => Undefined
    }
}

fn check_available_purchases(d: &mut GameData) {
    if d.tech.money >= 300.0 {
        // graphics::print_coord("Nano Fab available for purchase!\r\n",MESSAGES_ROW,0);
    }
}

fn take_action(a: &Action, d: &mut GameData) -> bool {
    match a {
        Action::MakeChip => {
            if d.tech.silicon >= 1 {
                d.tech.chips += (d.stats.wafer_die_count as f32 * d.stats.chip_yield).floor() as u64;
                d.tech.silicon -= 1;
            } else {
                // graphics::print_coord("Out of silicon!", MESSAGES_ROW, 0);
            }
        }
        Action::BuySilicon => {
            if d.tech.money >= d.stats.silicon_cost {
                d.tech.silicon += 5;
                d.tech.money -= d.stats.silicon_cost;
            } else {
                // graphics::print_coord("Out of money!", MESSAGES_ROW, 0);
            }
        }
        Action::BuyFab => {
            d.tech.fabs += 1;
        }
        Action::QuitGame => {
            return true;
        }
        Action::Undefined => {
                // graphics::print_coord("Error!", MESSAGES_ROW, 0);
        }
    }
    false
}

fn change_window(c: KeyCode, w: &Window) -> &Window {
    match c {
        KeyCode::Char('w') => {
            if *w == Window::Upgrades {
                &Tech
            } else if *w == Window::Stocks {
                &Stats
            } else {
                w
            }
        }
        KeyCode::Char('a') => {
            if *w == Window::Stats {
                &Tech
            } else if *w == Window::Stocks {
                &Upgrades
            } else {
                w
            }
        }
        KeyCode::Char('s') => {
            if *w == Window::Tech {
                &Upgrades
            } else if *w == Window::Stats {
                &Stocks
            } else {
                w
            }
        }
        KeyCode::Char('d') => {
            if *w == Window::Tech {
                &Stats
            } else if *w == Window::Upgrades {
                &Stocks
            } else {
                w
            }
        }
        _ => w
        
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    
    stdout().execute(EnterAlternateScreen)?; // Switches to a clean full-screen canvas
    
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    
    let mut data: GameData = game_data_init();

    // Track time for our automated background loop (1 tick per 500ms)
    let mut last_fab_tick = Instant::now();
    let mut last_sell_tick = Instant::now();

    let mut focused_window: &Window = &Tech;

    loop {
        _ = graphics::render_console(&mut terminal, &data, focused_window);

        // 1. Check for User Input (Quick non-blocking 20ms check)
        if event::poll(Duration::from_millis(20)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == KeyEventKind::Press {
                    let action = key_to_action(key_event.code);
                    focused_window = change_window(key_event.code, &mut focused_window);
                    
                    if let Action::QuitGame = action {
                        break; 
                    }
                    
                    // Take the action and mark that the state updated
                    take_action(&action, &mut data);
                }
            }
        }
        
        // sell clock
        if last_sell_tick.elapsed() >= Duration::from_millis((data.ticks.sell_tick / data.stats.chip_demand) as u64) {
            last_sell_tick = Instant::now(); // Reset the timer clock
            if data.tech.chips >= 1 {
                data.tech.chips -= 1;
                data.tech.money += data.stats.chip_performance / data.stats.chip_yield;
            } else {
                // graphics::print_coord("Out of stock!", MESSAGES_ROW, 0);
                // graphics::flush();
            }
        }

        // manufacturing clock
        if last_fab_tick.elapsed() >= Duration::from_millis(data.ticks.fab_tick as u64) {
            last_fab_tick = Instant::now(); // Reset the timer clock
            if data.tech.fabs >= 1 && data.tech.silicon >= data.tech.fabs as u64 {
                data.tech.chips += data.tech.fabs as u64 * (data.stats.wafer_die_count as f32 * data.stats.chip_yield).floor() as u64;
                data.tech.silicon -= data.tech.fabs as u64;
            } else {
                // graphics::print_coord("Fab ran out of silicon!", MESSAGES_ROW, 0);
                // graphics::flush();
            }
        }
    }

    disable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen)?; // Switches to a clean full-screen canvas
    println!("Thanks for playing!");

    Ok(())
}
