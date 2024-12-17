use std::fs::File;
use std::io::Write;
use chrono::{DateTime, Utc, NaiveDate, TimeZone};

#[derive(Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub enum Category {
    Work,
    Personal,
    Health,
    Education,
    Leisure,
    Household,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u16,
    pub name: String,
    pub priority: Priority,
    pub category: Category,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub struct TaskManager {
    pub date: NaiveDate,
    pub tasks: Vec<Task>,
    pub next_id: u16,
}

impl TaskManager {
    pub fn new(date: NaiveDate) -> Self {
        TaskManager {
            date,
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        self.date = date;
    }

    pub fn add_task(
        &mut self,
        name: String,
        priority: Priority,
        category: Category,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> u16 {
        let task = Task {
            id: self.next_id,
            name,
            priority,
            category,
            start_time,
            end_time,
        };
        self.tasks.push(task);
        self.next_id += 1;
        self.next_id - 1 // Return the ID of the new task
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn validate_id(&mut self, id: u16) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    pub fn edit_task(
        &mut self,
        id: u16,
        new_name: Option<String>,
        new_priority: Option<Priority>,
        new_category: Option<Category>,
        new_start_time: Option<DateTime<Utc>>,
        new_end_time: Option<DateTime<Utc>>,
    ) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if let Some(name) = new_name {
                task.name = name;
            }
            if let Some(priority) = new_priority {
                task.priority = priority;
            }
            if let Some(category) = new_category {
                task.category = category;
            }
            if let Some(start_time) = new_start_time {
                task.start_time = start_time;
            }
            if let Some(end_time) = new_end_time {
                task.end_time = end_time;
            }
            Ok(())
        } else {
            Err("Task ID not found".to_string())
        }
    }

    pub fn remove_task(&mut self, id: u16) -> Result<(), String> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err("Task ID not found".to_string())
        }
    }

    pub fn save_schedule(&self, file_name: &str) -> Result<(), String> {
        let mut file = File::create(file_name)
            .map_err(|e| format!("Failed to create file: {}", e))?;

        let mut content = format!("Schedule for {}\n\n", self.date);
        content.push_str("ID  START   END     NAME                PRIORITY   CATEGORY\n");
        content.push_str(&"-".repeat(80));
        content.push('\n');

        for task in &self.tasks {
            content.push_str(&format!(
                "{:<3} {:<7} {:<7} {:<20} {:<9} {:<10}\n",
                task.id,
                task.start_time.format("%H:%M"),
                task.end_time.format("%H:%M"),
                task.name,
                format!("{:?}", task.priority),
                format!("{:?}", task.category),
            ));
        }

        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }
}
