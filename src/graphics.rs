// src/graphics.rs

use std::io::{Stdout};

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::BorderType::{Double, Rounded};
use ratatui::widgets::{Paragraph, BorderType, Borders, Block};
use ratatui::style::{Color, Style};

use ratatui::layout::{Constraint, Direction, Layout};

use crate::Window;
use crate::game_data::{GameData, Tech, Stats, Upgrades};

pub fn render_tech<'a>(t: &Tech, w: &Window) -> Paragraph<'a>  {
    let tech_text = format!("
    Unsold Chips:       {}\n
    Silicon:            {}\n
    Money:              {:.2}\n
    Engineers:          {}\n
    Fabs:               {}
    ", t.chips, t.silicon, t.money, t.engineers, t.fabs);

    let (border_color, title, border_type) = if w == &Window::Tech {
        (Color::Green, "RESOURCES & TECHNOLOGY", Double)
    } else {
        (Color::Gray, "Resources & Technology", Rounded)
    };

    Paragraph::new(tech_text)
    .style(Style::default().fg(border_color))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_type(border_type)
    )
}

pub fn render_stats<'a>(s: &Stats, w: &Window) -> Paragraph<'a>  {
    let tech_text = format!("
    Chip Price (1U):        {:.2}\n
    Dies per Wafer:         {:.2}\n
    Wafer Yield:            {:.2}\n
    Chip Performance:       {:.2}\n
    Silicon Cost:           {:.2}
    ", s.chip_price, s.wafer_die_count, s.chip_yield, s.chip_performance, s.silicon_cost);

    let (border_color, title, border_type) = if w == &Window::Stats {
        (Color::Green, "OPERATIONAL STATISTICS", Double)
    } else {
        (Color::Gray, "Operational Statistics", Rounded)
    };

    Paragraph::new(tech_text)
    .style(Style::default().fg(border_color))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_type(border_type)
    )
}

pub fn render_upgrades<'a>(u: &Upgrades, w: &Window) -> Paragraph<'a>  {
    let tech_text = format!("
    Fab ............................ $ 1000
    MegaFab ........................ $ 10000
    GigaFab ........................ $ 100000
    Senior Engineer ................ $ 250K/yr
    Consultant ..................... $ 400k
    Upgrade Process Node ........... $ 2M
    M&A ............................ (View available firms)
    ");

    let (border_color, title, border_type) = if w == &Window::Upgrades {
        (Color::Green, "AVAILABLE UPGRADES", Double)
    } else {
        (Color::Gray, "Available Upgrades", Rounded)
    };

    Paragraph::new(tech_text)
    .style(Style::default().fg(border_color))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_type(border_type)
    )
}

pub fn render_console(t: &mut Terminal<CrosstermBackend<Stdout>>, d: &GameData, w: &Window) -> Result<(), std::io::Error> {
    t.draw(|frame| {
        let columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(frame.area());

        let top_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(columns[0]);

        let bottom_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(columns[1]);
        frame.render_widget(
            render_tech(&d.tech, w),
            top_row[0]
        );
        frame.render_widget(
            render_stats(&d.stats, w),
            top_row[1]
        );
        frame.render_widget(
            render_upgrades(&d.upgrades, w),
            bottom_row[0]
        );
    })?;
    Ok(())
}