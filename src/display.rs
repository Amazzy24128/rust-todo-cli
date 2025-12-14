use crate::task::{Priority, Task};
use colored::*;

/// 格式化单个任务的显示
pub fn format_task(task: &Task) -> String {
    let status = if task.completed {
        "✓". green().bold()
    } else {
        "○".yellow()
    };

    let priority_str = match task.priority {
        Priority::High => "HIGH". red().bold(),
        Priority:: Medium => "MED".yellow(),
        Priority::Low => "LOW".blue(),
    };

    let title = if task.completed {
        task.title.strikethrough().dimmed()
    } else {
        task.title.normal()
    };

    let due_info = if let Some(due) = task.due_date {
        let due_str = due.format("%Y-%m-%d").to_string();
        if task.is_overdue() {
            format!(" 📅 {}", due_str. red())
        } else {
            format!(" 📅 {}", due_str. cyan())
        }
    } else {
        String::new()
    };

    format!(
        "{} [{}] {} | {}{}",
        status,
        format! ("{:3}", task.id).cyan(),
        priority_str,
        title,
        due_info
    )
}

/// 打印任务列表
pub fn print_tasks(tasks: &[&Task], title: &str) {
    if tasks.is_empty() {
        println!("{}", "📭 No tasks found. ".dimmed());
        return;
    }

    println!("\n{}", title. bold().underline());
    println!("{}", "─".repeat(60).dimmed());
    
    for task in tasks {
        println!("{}", format_task(task));
    }
    
    println!("{}", "─".repeat(60).dimmed());
    println!("{} task(s)", tasks.len().to_string().cyan().bold());
}

/// 打印单个任务的详细信息
pub fn print_task_detail(task: &Task) {
    println!("\n{}", "Task Details". bold().underline());
    println!("{}", "─".repeat(60).dimmed());
    
    println!("{}:  {}", "ID".bold(), task.id. to_string().cyan());
    println!("{}: {}", "Title".bold(), task.title);
    
    let status = if task.completed {
        "Completed ✓".green()
    } else {
        "Pending ○".yellow()
    };
    println!("{}: {}", "Status". bold(), status);
    
    let priority_str = match task.priority {
        Priority::High => "High".red(),
        Priority::Medium => "Medium".yellow(),
        Priority::Low => "Low".blue(),
    };
    println!("{}: {}", "Priority".bold(), priority_str);
    
    println!("{}: {}", "Created". bold(), 
        task.created_at. format("%Y-%m-%d %H:%M:%S").to_string().dimmed());
    
    if let Some(due) = task.due_date {
        let due_str = due.format("%Y-%m-%d %H:%M:%S").to_string();
        let display = if task.is_overdue() {
            format!("{} {}", due_str, "(OVERDUE!)".red().bold())
        } else {
            due_str.cyan().to_string()
        };
        println!("{}: {}", "Due Date".bold(), display);
    } else {
        println!("{}:  {}", "Due Date".bold(), "None".dimmed());
    }
    
    println!("{}", "─".repeat(60).dimmed());
}

/// 打印成功消息
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message.green());
}

/// 打印错误消息
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.red());
}

/// 打印信息消息
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ". cyan().bold(), message);
}
