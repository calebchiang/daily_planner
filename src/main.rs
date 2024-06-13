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

    fn validate_id(&mut self) -> Option<&mut Task> {
        if self.tasks.is_empty() {
            println!("No tasks available.");
            return None;
        }

        println!("Enter the ID of the task:");
        let mut id_input = String::new();
        std::io::stdin().read_line(&mut id_input).expect("Failed to read line");

        if let Ok(id) = id_input.trim().parse::<u16>() {
            for task in &mut self.tasks {
                if task.id == id {
                    return Some(task);
                }
            }
            println!("No task found with ID: {}", id);
        } else {
            println!("Invalid ID entered. Please enter a numeric value.");
        }

        None
    }


    fn add_task(&mut self) {
        println!("Task Name:");
        let mut task_name = String::new();
        io::stdin().read_line(&mut task_name).expect("Failed to read line");
        let task_name = task_name.trim().to_string();

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
            println!("{:<2} {:<8} {:<8} {:<20} {:<9} {:<15} ",
                     "ID", "START", "END", "NAME", "PRIORITY", "CATEGORY");
            println!("{}", "-".repeat(100));
            for task in &self.tasks {
                let priority_color = match task.priority {
                    Priority::High => "\x1b[31;1mHigh\x1b[0m",    // Red for High
                    Priority::Medium => "\x1b[33;1mMedium\x1b[0m", // Yellow for Medium
                    Priority::Low => "\x1b[32;1mLow\x1b[0m",       // Green for Low
                };

                println!(
                    "{:>2} {:<8} {:<8} {:<20} {:<9} {:<15}",
                    task.id,
                    task.start_time.format("%H:%M"),
                    task.end_time.format("%H:%M"),
                    task.name,
                    priority_color,
                    format!("{:?}", task.category),
                );
            }
            println!();
        }
    }

    fn edit_task(&mut self) {
        self.display_schedule();
        if let Some(task) = self.validate_id() {
            println!("Select the field you want to edit:");
            println!("1. Name");
            println!("2. Priority");
            println!("3. Category");
            println!("4. Start time");
            println!("5. End time");

            let mut field_input = String::new();
            loop {
                io::stdin().read_line(&mut field_input).expect("Failed to read line");
                let choice = field_input.trim().parse::<u8>();

                match choice {
                    Ok(num) if num >= 1 && num <= 5 => {
                        match num {
                            1 => {
                                println!("Enter new name:");
                                let mut new_name = String::new();
                                io::stdin().read_line(&mut new_name).expect("Failed to read line");
                                task.name = new_name.trim().to_string();
                            },
                            2 => {
                                println!("Enter new priority (Low, Medium, High):");
                                let mut new_priority = String::new();
                                io::stdin().read_line(&mut new_priority).expect("Failed to read line");
                                task.priority = match new_priority.trim().to_lowercase().as_str() {
                                    "low" => Priority::Low,
                                    "medium" => Priority::Medium,
                                    "high" => Priority::High,
                                    _ => {
                                        println!("Invalid priority entered. Keeping previous.");
                                        continue;
                                    }
                                };
                            },
                            3 => {
                                println!("Enter new category (Work/Personal/Health/Education/Leisure/Household):");
                                let mut new_category = String::new();
                                io::stdin().read_line(&mut new_category).expect("Failed to read line");
                                task.category = match new_category.trim().to_lowercase().as_str() {
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
                            4 => {
                                println!("Enter new start time (HH:MM):");
                                let mut new_start_time = String::new();
                                io::stdin().read_line(&mut new_start_time).expect("Failed to read line");
                                match Utc.datetime_from_str(
                                    &format!("{} {}", task.start_time.date().naive_utc(), new_start_time.trim()),
                                    "%Y-%m-%d %H:%M"
                                ) {
                                    Ok(parsed_time) => task.start_time = parsed_time,
                                    Err(_) => {
                                        println!("Invalid time format. Please use HH:MM format.");
                                        continue;  // If invalid, repeat the time input
                                    }
                                }
                            },
                            5 => {
                                println!("Enter new end time (HH:MM):");
                                let mut new_end_time = String::new();
                                io::stdin().read_line(&mut new_end_time).expect("Failed to read line");
                                match Utc.datetime_from_str(
                                    &format!("{} {}", task.end_time.date().naive_utc(), new_end_time.trim()),
                                    "%Y-%m-%d %H:%M"
                                ) {
                                    Ok(parsed_time) => task.end_time = parsed_time,
                                    Err(_) => {
                                        println!("Invalid time format. Please use HH:MM format.");
                                        continue;  // If invalid, repeat the time input
                                    }
                                }
                            },

                            _ => unreachable!(), // Since we already validate the range, this should not happen
                        }
                        println!("Task updated successfully.");
                        break;  // Exit the loop after successful update
                    },
                    _ => {
                        println!("Invalid choice. Please enter a number between 1 and 4:");
                        field_input.clear();  // Clear invalid input before next iteration
                    }
                }
            }
        }
    }

    pub fn remove_task(&mut self) {
        self.display_schedule();

        if let Some(task) = self.validate_id() {
            let task_id = task.id; // Store the ID of the task to remove.
            if let Some(index) = self.tasks.iter().position(|t| t.id == task_id) {
                self.tasks.remove(index); // Remove the task by index.
                println!("Task successfully removed.");
            } else {
                println!("Task not found.");
            }
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
                3 => task_manager.edit_task(),
                4 => task_manager.remove_task(),
                5 => println!("Save function to be implemented."),
                6 => {
                    println!("Exiting the program.");
                    break;
                },
                _ => println!("Invalid option. Please try again."),
            }
        }
    }

