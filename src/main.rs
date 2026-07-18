use std::mem::take;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

mod graphics;

use crate::Action::{BuySilicon, MakeChip, SellChip, QuitGame, Undefined};

const STATS_ROW: u8 = 1;
const PROMPT_ROW: u8 = 20;
const MESSAGES_ROW: u8 = 25;
const HELP_ROW: u8 = 30;

struct GameData {
    chips: u64,
    silicon: u64,
    chip_cost: f32,
    silicon_cost: f32,
    chip_price_multiplier: f32,
    silicon_price_multiplier: f32,
    money: f32,
    engineers: u64,
    fabs: i32,
}

enum Action {
    SellChip,
    MakeChip,
    BuySilicon,
    QuitGame,
    Undefined
}

fn key_to_action(c: KeyCode) -> Action {
    match c {
        KeyCode::Char('s') => SellChip,
        KeyCode::Char('m') => MakeChip,
        KeyCode::Char('b') => BuySilicon,
        KeyCode::Char('q') => QuitGame,
        _ => Undefined
    }
}

fn display_status(d: &GameData) {
    graphics::print_coord(&format!("Chips:     {}", d.chips), STATS_ROW, 0);
    graphics::print_coord(&format!("Silicon:   {}", d.silicon), STATS_ROW + 1, 0);
    graphics::print_coord(&format!("Money:     {}", d.money), STATS_ROW + 2, 0);
    graphics::print_coord(&format!("Engineers: {}", d.engineers), STATS_ROW + 3, 0);
    graphics::print_coord(&format!("Fabs:      {}", d.fabs), STATS_ROW + 4, 0);
    graphics::print_coord("Press: [s] Sell, [m] Make, [b] Buy Silicon, [q] Quit\r\n>_ ", PROMPT_ROW, 0);
}

fn check_available_purchases(d: &GameData) {
    if d.money >= 300.0 {
        graphics::print_coord("Nano Fab available for purchase!\r\n",MESSAGES_ROW,0);
    }
}

fn take_action(a: &Action, d: &mut GameData) -> bool {
    match a {
        Action::SellChip => {
            if d.chips >= 1 {
                d.chips -= 1;
                d.money += d.chip_cost;
            } else {
                graphics::print_coord("Out of stock!", MESSAGES_ROW, 0);
            }
        }
        Action::MakeChip => {
            if d.silicon >= 1 {
                d.chips += 5;
                d.silicon -= 1;
            } else {
                graphics::print_coord("Out of silicon!", MESSAGES_ROW, 0);
            }
        }
        Action::BuySilicon => {
            if d.money >= d.silicon_cost {
                d.silicon += 5;
                d.money -= d.silicon_cost;
            } else {
                graphics::print_coord("Out of money!", MESSAGES_ROW, 0);
            }
        }
        Action::QuitGame => {
            return true;
        }
        Action::Undefined => {
                graphics::print_coord("Error!", MESSAGES_ROW, 0);
        }
    }
    false
}

fn game_loop(a: &Action, d: &mut GameData) -> bool {
    graphics::clear_screen();
    if take_action(a, d) {
        return true;
    }
    display_status(d);
    check_available_purchases(d);
    graphics::flush();
    false
}
fn main() {

    graphics::clear_screen();

    let mut data = GameData {
        chips: 50,
        silicon: 50,
        money: 100.0,
        silicon_cost: 1.0,
        silicon_price_multiplier: 1.0,
        chip_cost: 1.0,
        chip_price_multiplier: 1.5,
        engineers: 10,
        fabs: 0,
    };

    enable_raw_mode().unwrap();
    display_status(&data);

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            // Read the hardware event safely
            if let Event::Key(key_event) = event::read().unwrap() {
                
                // Crossterm captures both down-presses and releases (mostly on Windows).
                // Filter this so our Action logic only fires once per tap.
                if key_event.kind == KeyEventKind::Press {
                    let action = key_to_action(key_event.code);
                    
                    // Run the loop and break if 'q' was handled
                    if game_loop(&action, &mut data) {
                        break;
                    }
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    graphics::clear_screen();
    println!("Thanks for playing!");
}
