# Daily Planner CLI Application

A command-line daily planner built with **Rust** for organizing tasks and creating a structured schedule for the day.

The application allows users to add, edit, view, and remove tasks directly from the terminal, then save the completed daily schedule to a text file for later reference.

## Features

* **Add Tasks** — Create tasks with a start time, end time, priority, category, and description.
* **View Schedule** — Display the current day's tasks in an organized table directly in the terminal.
* **Edit Tasks** — Update the details of an existing task without recreating it.
* **Remove Tasks** — Delete tasks that are no longer needed.
* **Task Priorities** — Assign priorities to help distinguish important tasks from lower-priority work.
* **Task Categories** — Organize tasks into different categories for easier planning.
* **Save Schedule** — Export the completed daily schedule to a text file for future reference.
* **Interactive CLI** — Navigate planner functionality through a simple terminal-based interface.

## Tech Stack

* **Rust**
* Command-line interface
* File I/O for schedule persistence
* Rust structs and collections for task management

## How It Works

The planner maintains a collection of tasks for the current day. Each task contains information such as:

```text
Task
├── Start Time
├── End Time
├── Priority
├── Category
└── Description
```

Users interact with the planner through the terminal and can continuously modify their schedule until they are ready to save it.

A typical workflow looks like:

```text
1. Start the planner
2. Add tasks
3. View the daily schedule
4. Edit or remove tasks as needed
5. Confirm the final schedule
6. Save the schedule to a text file
```

## Example

A daily schedule might look similar to:

```text
---------------------------------------------------------
Start      End        Priority     Category       Task
---------------------------------------------------------
08:00      09:00      High         Work           Team meeting
10:00      11:30      High         Study          LeetCode practice
12:00      13:00      Medium       Personal       Lunch
15:00      16:00      Medium       Fitness        Workout
---------------------------------------------------------
```

## Getting Started

Clone the repository:

```bash
git clone https://github.com/calebchiang/daily_planner.git
cd daily_planner
```

Make sure Rust is installed on your machine.

You can verify your installation with:

```bash
rustc --version
cargo --version
```

Build the project:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

## Saving Schedules

When the user finishes planning their day, the application can write the current schedule to a text file.

This provides a simple way to preserve previous schedules without requiring an external database or cloud service.

## Project Goals

This project was built to practice core Rust concepts while developing a useful command-line application, including:

* Structs and data modeling
* Collections
* User input handling
* Control flow
* Error handling
* File reading and writing
* Building interactive CLI applications

## Future Improvements

Potential improvements include:

* Persistent task storage between sessions
* Support for multiple dates
* Recurring tasks
* Task completion tracking
* Schedule conflict detection
* Filtering tasks by category or priority
* Colored terminal output
* JSON or database-based persistence

## Repository

Source code is available on GitHub:

`github.com/calebchiang/daily_planner`
