// src/graphics.rs

use std::io::{Stdout};

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Paragraph, BorderType, Borders, Block};
use ratatui::style::{Color, Style};

use ratatui::layout::{Constraint, Direction, Layout};

use crate::game_data::{GameData, Tech, Stats, Upgrades};

pub fn render_tech<'a>(t: &Tech) -> Paragraph<'a>  {
    let tech_text = format!("
    Chips:      {}\n
    Silicon:    {}\n
    Money:      {}\n
    Engineers:  {}\n
    Fabs:       {}
    ", t.chips, t.silicon, t.money, t.engineers, t.fabs);

    Paragraph::new(tech_text)
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Technology & Resources")
            .border_type(BorderType::Rounded)
    )
}

pub fn render_stats<'a>(s: &Stats) -> Paragraph<'a>  {
    let tech_text = format!("
    Chip Price (1U):        {}\n
    Dies per Wafer:         {}\n
    Wafer Yield:            {}\n
    Chip Performance:       {}\n
    Silicon Cost:           {}
    ", s.chip_price, s.wafer_die_count, s.chip_yield, s.chip_performance, s.silicon_cost);

    Paragraph::new(tech_text)
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Operational Statistics")
            .border_type(BorderType::Rounded)
    )
}

pub fn render_upgrades<'a>(u: &Upgrades) -> Paragraph<'a>  {
    let tech_text = format!("
    Fab ............................ $ 1000
    MegaFab ........................ $ 10000
    GigaFab ........................ $ 100000
    Senior Engineer ................ $ 250K/yr
    Consultant ..................... $ 400k
    Upgrade Process Node ........... $ 2M
    M&A ............................ (View available firms)
    ");

    Paragraph::new(tech_text)
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Available Upgrades")
            .border_type(BorderType::Rounded)
    )
}

pub fn render_console(t: &mut Terminal<CrosstermBackend<Stdout>>, d: &GameData) -> Result<(), std::io::Error> {
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
            render_tech(&d.tech),
            top_row[0]
        );
        frame.render_widget(
            render_stats(&d.stats),
            top_row[1]
        );
        frame.render_widget(
            render_upgrades(&d.upgrades),
            bottom_row[0]
        );
    })?;
    Ok(())
}