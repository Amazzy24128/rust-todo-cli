use clap:: Parser;
use rust_todo_cli::{
    cli::{Cli, Commands, ListFilter},
    display::*,
    error::TodoError,
    storage::{load_from_file, save_to_file, DEFAULT_FILE_PATH},
    task::Priority,
    todo_list::TodoList,
};
use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    // 解析命令行参数
    let cli = Cli::parse();

    // 执行命令并处理结果
    if let Err(e) = run(cli) {
        print_error(&format!("{}", e));
        std::process:: exit(1);
    }
}

/// 主要运行逻辑
fn run(cli: Cli) -> Result<(), TodoError> {
    // 加载现有的待办列表
    let mut todo_list = load_from_file(DEFAULT_FILE_PATH)?;

    // 根据命令执行对应操作
    match cli.command {
        Commands::Add {
            title,
            priority,
            due,
        } => {
            handle_add(&mut todo_list, title, priority, due)?;
        }

        Commands::List { filter } => {
            handle_list(&todo_list, filter);
        }

        Commands::Complete { id } => {
            handle_complete(&mut todo_list, id)?;
        }

        Commands:: Delete { id } => {
            handle_delete(&mut todo_list, id)?;
        }

        Commands::Show { id } => {
            handle_show(&todo_list, id)?;
        }

        Commands::Clear { force } => {
            handle_clear(&mut todo_list, force)?;
        }
    }

    Ok(())
}

/// 处理添加任务
fn handle_add(
    todo_list: &mut TodoList,
    title: String,
    priority_str: String,
    due_str: Option<String>,
) -> Result<(), TodoError> {
    // 解析优先级
    let priority = Priority::from_str(&priority_str)
        .map_err(|_| TodoError::InvalidPriority(priority_str.clone()))?;

    // 解析截止日期
    let due_date = if let Some(date_str) = due_str {
        Some(parse_date(&date_str)?)
    } else {
        None
    };

    // 添加任务
    let id = todo_list.add_task(title. clone(), priority, due_date);

    // 保存到文件
    save_to_file(todo_list, DEFAULT_FILE_PATH)?;

    // 显示成功消息
    print_success(&format!(
        "Task added successfully!  (ID: {})",
        id
    ));

    // 显示任务详情
    if let Some(task) = todo_list.find_task(id) {
        println!();
        print_task_detail(task);
    }

    Ok(())
}

/// 处理列出任务
fn handle_list(todo_list: &TodoList, filter:  Option<ListFilter>) {
    let filter = filter.unwrap_or(ListFilter::All);

    match filter {
        ListFilter::All => {
            let tasks:  Vec<_> = todo_list.list_tasks().iter().collect();
            print_tasks(&tasks, "📋 All Tasks");
        }
        ListFilter::Pending => {
            let tasks = todo_list.list_pending_tasks();
            print_tasks(&tasks, "⏳ Pending Tasks");
        }
        ListFilter:: Completed => {
            let tasks = todo_list.list_completed_tasks();
            print_tasks(&tasks, "✅ Completed Tasks");
        }
        ListFilter::Overdue => {
            let tasks = todo_list. overdue_tasks();
            print_tasks(&tasks, "⚠️  Overdue Tasks");
        }
    }

    // 显示统计信息
    println!();
    print_statistics(todo_list);
}

/// 处理完成任务
fn handle_complete(todo_list: &mut TodoList, id: u32) -> Result<(), TodoError> {
    // 检查任务是否已经完成
    if let Some(task) = todo_list.find_task(id) {
        if task.completed {
            print_info(&format!("Task {} is already completed", id));
            return Ok(());
        }
    }

    // 完成任务
    todo_list.complete_task(id)?;

    // 保存到文件
    save_to_file(todo_list, DEFAULT_FILE_PATH)?;

    print_success(&format!("Task {} marked as completed!", id));

    // 显示更新后的任务
    if let Some(task) = todo_list.find_task(id) {
        println!();
        println!("{}", format_task(task));
    }

    Ok(())
}

/// 处理删除任务
fn handle_delete(todo_list: &mut TodoList, id: u32) -> Result<(), TodoError> {
    // 先获取任务信息用于显示
    let task_title = todo_list
        .find_task(id)
        .map(|t| t.title.clone())
        .ok_or(TodoError::TaskNotFound(id))?;

    // 删除任务
    todo_list.delete_task(id)?;

    // 保存到文件
    save_to_file(todo_list, DEFAULT_FILE_PATH)?;

    print_success(&format!("Task {} '{}' deleted!", id, task_title));

    Ok(())
}

/// 处理显示任务详情
fn handle_show(todo_list: &TodoList, id: u32) -> Result<(), TodoError> {
    let task = todo_list.find_task(id).ok_or(TodoError::TaskNotFound(id))?;

    print_task_detail(task);

    Ok(())
}

/// 处理清除已完成任务
fn handle_clear(todo_list: &mut TodoList, force: bool) -> Result<(), TodoError> {
    let completed_tasks = todo_list.list_completed_tasks();

    if completed_tasks.is_empty() {
        print_info("No completed tasks to clear");
        return Ok(());
    }

    // 如果没有 force 标志，请求确认
    if !force {
        println!(
            "⚠️  About to delete {} completed task(s). Are you sure? (y/N): ",
            completed_tasks.len()
        );
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if ! input.trim().eq_ignore_ascii_case("y") {
            print_info("Operation cancelled");
            return Ok(());
        }
    }

    // 收集要删除的任务 ID
    let ids_to_delete: Vec<u32> = completed_tasks. iter().map(|t| t.id).collect();

    // 删除所有已完成的任务
    for id in &ids_to_delete {
        todo_list.delete_task(*id)?;
    }

    // 保存到文件
    save_to_file(todo_list, DEFAULT_FILE_PATH)?;

    print_success(&format!(
        "Cleared {} completed task(s)!",
        ids_to_delete. len()
    ));

    Ok(())
}

/// 解析日期字符串（格式：YYYY-MM-DD）
fn parse_date(date_str: &str) -> Result<chrono::DateTime<chrono:: Utc>, TodoError> {
    use chrono::{NaiveDate, TimeZone, Utc};

    let naive_date = NaiveDate:: parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| TodoError::InvalidDateFormat(date_str.to_string()))?;

    // 转换为 UTC DateTime（时间设置为当天结束）
    let datetime = naive_date
        .and_hms_opt(23, 59, 59)
        .ok_or_else(|| TodoError::InvalidDateFormat(date_str.to_string()))?;

    Ok(Utc.from_utc_datetime(&datetime))
}

/// 打印统计信息
fn print_statistics(todo_list: &TodoList) {
    use colored::*;

    let total = todo_list.len();
    let pending = todo_list.list_pending_tasks().len();
    let completed = todo_list.list_completed_tasks().len();
    let overdue = todo_list.overdue_tasks().len();

    println!("{}", "📊 Statistics".bold());
    println!("  Total:      {}", total. to_string().cyan());
    println!("  Pending:   {}", pending.to_string().yellow());
    println!("  Completed: {}", completed.to_string().green());
    
    if overdue > 0 {
        println!("  Overdue:   {}", overdue.to_string().red().bold());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_parse_date() {
        let result = parse_date("2025-12-31");
        assert!(result.is_ok());

        let date = result.unwrap();
        assert_eq!(date.format("%Y-%m-%d").to_string(), "2025-12-31");
    }

    #[test]
    fn test_parse_invalid_date() {
        let result = parse_date("invalid-date");
        assert!(result. is_err());

        if let Err(TodoError::InvalidDateFormat(msg)) = result {
            assert_eq!(msg, "invalid-date");
        } else {
            panic!("Expected InvalidDateFormat error");
        }
    }

    #[test]
    fn test_parse_date_wrong_format() {
        let result = parse_date("12/31/2025");
        assert!(result.is_err());
    }
}
