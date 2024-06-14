extern crate chrono;
extern crate env_logger;
use chrono::{NaiveDate};
mod lib;
mod quote;


    fn main() {

        match quote::fetch_quote() {
            Ok(quote) => println!("Quote of the day: {}", quote),
            Err(err) => println!("Failed to fetch quote: {}", err),
        }

        let mut task_manager = lib::TaskManager::new(NaiveDate::from_ymd(1970, 1, 1));
        task_manager.get_date();

        loop {
            let user_input = lib::TaskManager::get_user_input();
            match user_input {
                1 => task_manager.add_task(),
                2 => task_manager.display_schedule(),
                3 => task_manager.edit_task(),
                4 => task_manager.remove_task(),
                5 => task_manager.save_schedule(),
                6 => {
                    println!("Exiting the program.");
                    break;
                },
                _ => println!("Invalid option. Please try again."),
            }
        }
    }

