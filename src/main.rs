mod quote;
mod task;
mod ui;

use quote::fetch_quote;
use ui::startup_ui;
use task::TaskManager;
use chrono::Local;

fn main() {
    // Step 1: Fetch the quote
    let quote = match fetch_quote() {
        Ok(quote) => quote,
        Err(err) => {
            eprintln!("Failed to fetch quote: {}", err);
            "Welcome to your Daily Planner!".to_string()
        }
    };

    // Step 2: Display startup UI and get the date
    let date = match startup_ui(&quote) {
        Ok(date) => date,
        Err(err) => {
            eprintln!("Error initializing UI: {}", err);
            return;
        }
    };

    // Step 3: Initialize the TaskManager with the date
    let mut task_manager = TaskManager::new(date);
    // TODO: Transition to the main task management interface
    ui::task_ui(&mut task_manager, &quote).unwrap();

}
