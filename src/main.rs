extern crate chrono;
use chrono::{NaiveDate, DateTime, Utc, NaiveTime, TimeZone};
use std::io;

#[derive(Debug)]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
enum Category {
    Work,
    Personal,
    Health,
    Education,
    Leisure,
    Household,
}

struct Task {
    id: u16,
    name: String,
    description: String,
    priority: Priority,
    category: Category,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

struct TaskManager {
    date: NaiveDate,
    tasks: Vec<Task>,
    next_id: u16,
}

impl TaskManager {
    fn new(date: NaiveDate) -> Self {
        TaskManager {
            date,
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn get_date(&mut self) {
        loop {
            println!("-----------------------");
            println!("Daily Planner");
            println!("Enter today's date (YYYY-MM-DD):");

            let mut date_input = String::new();
            io::stdin().read_line(&mut date_input).expect("Failed to read line");
            match NaiveDate::parse_from_str(date_input.trim(), "%Y-%m-%d") {
                Ok(date) => {
                    self.date = date;
                    break;  // Exit loop if date is valid
                },
                Err(_) => {
                    println!("Invalid date format. Ensure it's YYYY-MM-DD. Please try again.");
                }
            }
        }
    }

    fn get_user_input() -> u8 {
        loop {
            println!("-----------------------");
            println!("Choose an option:");
            println!("1. Add Task");
            println!("2. View Schedule");
            println!("3. Edit Task");
            println!("4. Remove Task");
            println!("5. Finish and Save Schedule");
            println!("6. Exit");

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");

            match user_input.trim().parse::<u8>() {
                Ok(num) if num >= 1 && num <= 6 => return num,
                _ => {
                    println!("Invalid input. Please enter a number between 1 and 6:")
                }
            }
        }
    }

    fn add_task(&mut self) {
        println!("Task Name:");
        let mut task_name = String::new();
        io::stdin().read_line(&mut task_name).expect("Failed to read line");
        let task_name = task_name.trim().to_string();

        println!("Description:");
        let mut description = String::new();
        io::stdin().read_line(&mut description).expect("Failed to read line");
        let description = description.trim().to_string();

        // Loop for priority input
        let mut priority_input = String::new();
        let mut priority;
        loop {
            println!("Priority (High/Medium/Low):");
            priority_input.clear();  // Clear previous input
            io::stdin().read_line(&mut priority_input).expect("Failed to read line");
            priority = match priority_input.trim().to_lowercase().as_ref() {
                "high" => Priority::High,
                "medium" => Priority::Medium,
                "low" => Priority::Low,
                _ => {
                    println!("Invalid priority. Please enter 'high', 'medium', or 'low'.");
                    continue;
                },
            };
            break;  // Exit loop if valid priority is entered
        }

        // Loop for category input
        let mut category_input = String::new();
        let mut category;
        loop {
            println!("Category (Work/Personal/Health/Education/Leisure/Household):");
            category_input.clear();  // Clear previous input
            io::stdin().read_line(&mut category_input).expect("Failed to read line");
            category = match category_input.trim().to_lowercase().as_ref() {
                "work" => Category::Work,
                "personal" => Category::Personal,
                "health" => Category::Health,
                "education" => Category::Education,
                "leisure" => Category::Leisure,
                "household" => Category::Household,
                _ => {
                    println!("Invalid Category.");
                    continue;
                },
            };
            break;
        }

        // Logic to get start_time and end_time
        let mut start_time_input = String::new();
        let mut end_time_input = String::new();
        let start_time: DateTime<Utc>;
        let end_time: DateTime<Utc>;
        let date_format = "%Y-%m-%d %H:%M";  // Combined date and time format

        loop {
            println!("Enter start time (HH:MM):");
            start_time_input.clear(); // Clear previous input
            io::stdin().read_line(&mut start_time_input).expect("Failed to read line");
            let combined_start_time = format!("{} {}", self.date, start_time_input.trim());
            match Utc.datetime_from_str(&combined_start_time, date_format) {
                Ok(dt) => {
                    start_time = dt;
                    break;
                },
                Err(_) => {
                    println!("Invalid time format. Please use HH:MM format.");
                    continue;
                }
            }
        }

        loop {
            println!("Enter end time (HH:MM):");
            end_time_input.clear(); // Clear previous input
            io::stdin().read_line(&mut end_time_input).expect("Failed to read line");
            let combined_end_time = format!("{} {}", self.date, end_time_input.trim());
            match Utc.datetime_from_str(&combined_end_time, date_format) {
                Ok(dt) => {
                    end_time = dt;
                    break;
                },
                Err(_) => {
                    println!("Invalid time format. Please use HH:MM format.");
                    continue;
                }
            }
        }

        let task = Task {
            id: self.next_id,
            name: task_name,
            description: description,
            priority: priority,
            category: category,
            start_time: start_time,
            end_time: end_time,
        };

        self.tasks.push(task);
        self.next_id += 1;

        println!("Task added successfully. Task ID: {}", self.next_id - 1);
    }

    fn display_schedule(&self) {
        if self.tasks.is_empty() {
            println!("No tasks to display.");
        } else {
            println!("\nSchedule for Today:\n");
            println!("{:<5} {:<8} {:<8} {:<20} {:<30} {:<10} {:<10} ",
                     "ID", "START", "END", "NAME", "DESCRIPTION", "PRIORITY", "CATEGORY");
            println!("{}", "-".repeat(100));
            for task in &self.tasks {
                let priority_color = match task.priority {
                    Priority::High => "\x1b[31;1mHigh\x1b[0m",    // Red for High
                    Priority::Medium => "\x1b[33;1mMedium\x1b[0m", // Yellow for Medium
                    Priority::Low => "\x1b[32;1mLow\x1b[0m",       // Green for Low
                };

                println!(
                    "{:>5} {:<8} {:<8} {:<20} {:<30} {:<10} {:<10}",
                    task.id,
                    task.start_time.format("%H:%M"),
                    task.end_time.format("%H:%M"),
                    task.name,
                    task.description,
                    priority_color,
                    format!("{:?}", task.category),
                );
            }
            println!();
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new(NaiveDate::from_ymd(1970, 1, 1));
    task_manager.get_date();

    loop {
        let user_input = TaskManager::get_user_input();
        match user_input {
            1 => task_manager.add_task(),
            2 => task_manager.display_schedule(),
            3 => println!("Edit function to be implemented. "),
            4 => println!("Remove function to be implemented."),
            5 => println!("Save function to be implemented."),
            6 => {
                println!("Exiting the program.");
                break;
            },
            _ => println!("Invalid option. Please try again."),
        }
    }
}
