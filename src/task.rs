use std::fs;
use std::path::Path;
use std::fmt;
use chrono::{DateTime, Utc, NaiveDate};

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

// Implement Display for Priority
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        };
        write!(f, "{}", text)
    }
}

// Implement Display for Category
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Category::Work => "Work",
            Category::Personal => "Personal",
            Category::Health => "Health",
            Category::Education => "Education",
            Category::Leisure => "Leisure",
            Category::Household => "Household",
        };
        write!(f, "{}", text)
    }
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

    pub fn save_schedule(&self, file_path: &Path) -> Result<(), String> {
        let file = fs::File::create(file_path)
            .map_err(|e| format!("Failed to create file: {}", e))?;

        let mut writer = csv::Writer::from_writer(file);

        // Write header row
        writer.write_record(&["Task ID", "Task Name", "Start Time", "End Time", "Priority", "Category"])
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        // Write task rows
        for task in &self.tasks {
            writer.write_record(&[
                task.id.to_string(),
                task.name.clone(),
                task.start_time.format("%H:%M").to_string(),
                task.end_time.format("%H:%M").to_string(),
                format!("{:?}", task.priority),
                format!("{:?}", task.category),
            ])
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        }
        writer.flush().map_err(|e| format!("Failed to flush to file: {}", e))?;
        Ok(())
    }
}
