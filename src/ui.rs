use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, prelude::Alignment, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Paragraph}, Terminal
};
use crossterm::{
    event::{self, read, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}
};
use std::{env::home_dir, fs, io::{self, stdout}, path::PathBuf};
use chrono::{NaiveDate, NaiveTime, Utc};

use crate::task::{Category, Priority, TaskManager};

pub fn startup_ui(quote: &str) -> io::Result<NaiveDate> {
    enable_raw_mode()?; 
    let mut stdout = stdout(); 
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut date_input = String::new();

    // Clear the screen before rendering the UI
    execute!(stdout, Clear(ClearType::All))?;
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(6), 
                    Constraint::Min(1),   
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
                        execute!(stdout, Clear(ClearType::All))?; 
                        return Ok(date); 
                    } else {
                        date_input.clear(); 
                    }
                }
                KeyCode::Char(c) => date_input.push(c),
                KeyCode::Backspace => {
                    date_input.pop();
                }
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    execute!(stdout, Clear(ClearType::All))?; 
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
    let mut needs_redraw = true; // Flag to track if the UI needs to be redrawn

    // Ensure the "schedules" directory exists in the home directory
    let schedules_dir: PathBuf = home_dir().unwrap_or_else(|| PathBuf::from(".")).join("schedules");
    if let Err(e) = fs::create_dir_all(&schedules_dir) {
        eprintln!("Error creating schedules directory: {}", e);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to create schedules directory"));
    }

    loop {
        // Redraw the UI only if needed
        if needs_redraw {
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
                            "{} | {} - {} | {} | Priority: {}",
                            task.name,
                            task.start_time.format("%H:%M"),
                            task.end_time.format("%H:%M"),
                            task.category,
                            task.priority
                        ))
                        .style(style)
                    })
                    .collect();

                let list = List::new(task_list)
                    .block(Block::default().borders(Borders::ALL).title("Tasks"));
                f.render_widget(list, chunks[1]);

                // Instructions Panel
                let instructions = Paragraph::new("Up/Down: Navigate | a: Add | r: Remove | s: Save | q: Quit")
                    .style(Style::default().fg(Color::LightCyan))
                    .block(Block::default().borders(Borders::ALL).title("Instructions"));
                f.render_widget(instructions, chunks[2]);
            })?;

            needs_redraw = false; // Reset the redraw flag
        }

        // Event Handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(stdout(), Clear(ClearType::All))?; // Clear the terminal
                    return Ok(());
                }
                KeyCode::Down => {
                    if !task_manager.tasks.is_empty() && selected_task < task_manager.tasks.len() - 1 {
                        selected_task += 1;
                        needs_redraw = true; // Update the UI
                    }
                }
                KeyCode::Up => {
                    if !task_manager.tasks.is_empty() && selected_task > 0 {
                        selected_task -= 1;
                        needs_redraw = true; // Update the UI
                    }
                }
                KeyCode::Char('a') => {
                    prompt_new_task(task_manager, &mut terminal)?; // Use shared terminal
                    needs_redraw = true; // Force a full redraw
                }
                KeyCode::Char('r') => {
                    if !task_manager.tasks.is_empty() {
                        task_manager.tasks.remove(selected_task);
                        if selected_task > 0 {
                            selected_task -= 1;
                        }
                        needs_redraw = true; // Refresh the UI
                    }
                }
                KeyCode::Char('s') => {
                    let save_path = schedules_dir.join(format!("{}.csv", task_manager.date));
                    match task_manager.save_schedule(&save_path) {
                        Ok(_) => {
                            disable_raw_mode()?;
                            execute!(stdout(), Clear(ClearType::All))?; // Clear the terminal
                            println!("\nSchedule saved successfully to {:?}!", save_path);
                            return Ok(());
                        }
                        Err(err) => {
                            disable_raw_mode()?;
                            execute!(stdout(), Clear(ClearType::All))?; // Clear the terminal
                            eprintln!("Error saving schedule: {}", err);
                            return Err(io::Error::new(io::ErrorKind::Other, "Save failed"));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Interactive task input using ratatui
pub fn prompt_new_task(
    task_manager: &mut TaskManager,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> io::Result<()> {
    let mut needs_redraw = true;
    let mut selected_field = 0; // Index of the currently selected field
    let mut task_name = String::new();
    let mut priority = Some(Priority::High); // Default value
    let mut category = Some(Category::Work); // Default value
    let mut start_time = None;
    let mut end_time = None;
    let mut input_buffer = String::new(); // Buffer for user input

    // Priority and Category options for scrolling
    let priority_options = [Priority::High, Priority::Medium, Priority::Low];
    let category_options = [
        Category::Work,
        Category::Personal,
        Category::Health,
        Category::Education,
        Category::Leisure,
        Category::Household,
    ];
    let mut priority_index = 0;
    let mut category_index = 0;

    loop {
        if needs_redraw {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Task Name
                        Constraint::Length(3), // Priority
                        Constraint::Length(3), // Category
                        Constraint::Length(3), // Start Time
                        Constraint::Length(3), // End Time
                        Constraint::Length(3), // Save Button
                    ])
                    .split(f.size());

                // Task Name Field
                let task_name_widget = Paragraph::new(format!(
                    "Task Name: {}",
                    if selected_field == 0 {
                        input_buffer.clone()
                    } else {
                        task_name.clone()
                    }
                ))
                .style(if selected_field == 0 {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default()
                })
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(task_name_widget, chunks[0]);

                // Priority Field with Scrolling
                let priority_widget = Paragraph::new(format!(
                    "Priority: < {} >",
                    priority_options[priority_index]
                ))
                .style(if selected_field == 1 {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default()
                })
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(priority_widget, chunks[1]);

                // Category Field with Scrolling
                let category_widget = Paragraph::new(format!(
                    "Category: < {} >",
                    category_options[category_index]
                ))
                .style(if selected_field == 2 {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default()
                })
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(category_widget, chunks[2]);

                // Start Time Field
                let start_time_widget = Paragraph::new(format!(
                    "Start Time: {}",
                    if selected_field == 3 {
                        input_buffer.clone()
                    } else {
                        start_time
                            .map(|t: NaiveTime| t.format("%H:%M").to_string())
                            .unwrap_or("[HH:MM]".to_string())
                    }
                ))
                .style(if selected_field == 3 {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default()
                })
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(start_time_widget, chunks[3]);

                // End Time Field
                let end_time_widget = Paragraph::new(format!(
                    "End Time: {}",
                    if selected_field == 4 {
                        input_buffer.clone()
                    } else {
                        end_time
                            .map(|t: NaiveTime| t.format("%H:%M").to_string())
                            .unwrap_or("[HH:MM]".to_string())
                    }
                ))
                .style(if selected_field == 4 {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default()
                })
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(end_time_widget, chunks[4]);

                // Save Button
                let save_button = Paragraph::new("Save Task")
                    .style(if selected_field == 5 {
                        Style::default().fg(Color::Black).bg(Color::Green)
                    } else {
                        Style::default()
                    })
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(save_button, chunks[5]);
            })?;

            needs_redraw = false;
        }

        // Handle User Input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if selected_field > 0 {
                        selected_field -= 1;
                        needs_redraw = true;
                        input_buffer.clear();
                    }
                }
                KeyCode::Down => {
                    if selected_field < 5 {
                        // Finalize input for the current field
                        match selected_field {
                            0 => task_name = input_buffer.clone(),
                            3 => start_time = NaiveTime::parse_from_str(&input_buffer, "%H:%M").ok(),
                            4 => end_time = NaiveTime::parse_from_str(&input_buffer, "%H:%M").ok(),
                            _ => {}
                        }
                        selected_field += 1;
                        needs_redraw = true;
                        input_buffer.clear();
                    }
                }
                KeyCode::Left => {
                    if selected_field == 1 {
                        priority_index = (priority_index + priority_options.len() - 1) % priority_options.len();
                        priority = Some(priority_options[priority_index].clone());
                        needs_redraw = true;
                    } else if selected_field == 2 {
                        category_index = (category_index + category_options.len() - 1) % category_options.len();
                        category = Some(category_options[category_index].clone());
                        needs_redraw = true;
                    }
                }
                KeyCode::Right => {
                    if selected_field == 1 {
                        priority_index = (priority_index + 1) % priority_options.len();
                        priority = Some(priority_options[priority_index].clone());
                        needs_redraw = true;
                    } else if selected_field == 2 {
                        category_index = (category_index + 1) % category_options.len();
                        category = Some(category_options[category_index].clone());
                        needs_redraw = true;
                    }
                }
                KeyCode::Char(c) => {
                    if selected_field == 0 || selected_field == 3 || selected_field == 4 {
                        input_buffer.push(c);
                        needs_redraw = true;
                    }
                }
                KeyCode::Backspace => {
                    if !input_buffer.is_empty() {
                        input_buffer.pop();
                        needs_redraw = true;
                    }
                }
                KeyCode::Enter => {
                    if selected_field == 5 {
                        // Save the task
                        if let (Some(priority), Some(category), Some(start_time), Some(end_time)) =
                            (priority.clone(), category.clone(), start_time, end_time)
                        {
                            if end_time > start_time {
                                task_manager.add_task(
                                    task_name.clone(),
                                    priority,
                                    category,
                                    Utc::now().date().and_time(start_time).unwrap(),
                                    Utc::now().date().and_time(end_time).unwrap(),
                                );
                                return Ok(());
                            }
                        }
                    }
                }
                KeyCode::Esc => return Ok(()),
                _ => {}
            }
        }
    }
}
