use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, prelude::Alignment, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Paragraph}, Terminal
};
use crossterm::{
    event::{self, read, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}
};
use std::io::{self, stdout};
use chrono::NaiveDate;

use crate::task::TaskManager;

pub fn startup_ui(quote: &str) -> io::Result<NaiveDate> {
    enable_raw_mode()?; // Enable raw mode for input handling
    let mut stdout = stdout(); // Explicitly define stdout
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut date_input = String::new();

    // Clear the screen before rendering the UI
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(6), // Quote panel height
                    Constraint::Min(1),    // Space for date input
                ])
                .split(f.size());

            // Display the quote at the top
            let quote_widget = Paragraph::new(vec![
                Line::from(Span::styled(
                    "Inspirational Quote of the Day",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::raw("")),
                Line::from(Span::styled(
                    quote,
                    Style::default().fg(Color::White),
                )),
            ])
            .block(Block::default().borders(Borders::ALL).title("Welcome!"))
            .alignment(Alignment::Center);
            f.render_widget(quote_widget, chunks[0]);

            // Display plain text for date input (no panel)
            let date_prompt = Paragraph::new(format!(
                "Enter Today's Date (YYYY-MM-DD): {}",
                date_input
            ))
            .style(Style::default().fg(Color::Cyan));
            f.render_widget(date_prompt, chunks[1]);
        })?;

        // Capture user input
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Enter => {
                    if let Ok(date) = NaiveDate::parse_from_str(&date_input, "%Y-%m-%d") {
                        disable_raw_mode()?;
                        execute!(stdout, Clear(ClearType::All))?; // Clear screen before exiting
                        return Ok(date); // Return the parsed date
                    } else {
                        date_input.clear(); // Invalid date, clear input
                    }
                }
                KeyCode::Char(c) => date_input.push(c),
                KeyCode::Backspace => {
                    date_input.pop();
                }
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    execute!(stdout, Clear(ClearType::All))?; // Clear screen before quitting
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}


/// Renders the main task interface.
pub fn task_ui(task_manager: &mut TaskManager, quote: &str) -> io::Result<()> {
    enable_raw_mode()?; // Enable raw mode
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut selected_task = 0;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Quote Panel
                    Constraint::Min(5),    // Task List
                    Constraint::Length(3), // Instructions
                ])
                .split(f.size());

            // Quote Panel
            let quote_widget = Paragraph::new(Line::from(Span::styled(
                quote,
                Style::default().fg(Color::Yellow),
            )))
            .block(Block::default().borders(Borders::ALL).title("Quote"));
            f.render_widget(quote_widget, chunks[0]);

            // Task List Panel
            let task_list: Vec<ListItem> = task_manager
                .tasks
                .iter()
                .enumerate()
                .map(|(i, task)| {
                    let style = if i == selected_task {
                        Style::default().fg(Color::Black).bg(Color::White)
                    } else {
                        Style::default()
                    };

                    ListItem::new(format!(
                        "[{}] {} ({} - {})",
                        task.id,
                        task.name,
                        task.start_time.format("%H:%M"),
                        task.end_time.format("%H:%M")
                    ))
                    .style(style)
                })
                .collect();

            let list = List::new(task_list)
                .block(Block::default().borders(Borders::ALL).title("Tasks"));
            f.render_widget(list, chunks[1]);

            // Instructions Panel
            let instructions = Paragraph::new("Up/Down: Navigate | a: Add | e: Edit | r: Remove | s: Save | q: Quit")
                .style(Style::default().fg(Color::LightCyan))
                .block(Block::default().borders(Borders::ALL).title("Instructions"));
            f.render_widget(instructions, chunks[2]);
        })?;

        // Event Handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    return Ok(());
                }
                KeyCode::Down => {
                    if selected_task < task_manager.tasks.len() - 1 {
                        selected_task += 1;
                    }
                }
                KeyCode::Up => {
                    if selected_task > 0 {
                        selected_task -= 1;
                    }
                }
                KeyCode::Char('a') => {
                    // TODO: Add task input logic
                }
                KeyCode::Char('e') => {
                    // TODO: Edit selected task
                }
                KeyCode::Char('r') => {
                    // TODO: Remove selected task
                }
                KeyCode::Char('s') => {
                    task_manager.save_schedule("schedule.txt").unwrap();
                }
                _ => {}
            }
        }
    }

}
