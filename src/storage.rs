use crate::error::{TodoError, TodoResult};
use crate::todo_list::TodoList;
use std::fs;
use std::path::Path;

/// 默认数据文件路径
pub const DEFAULT_FILE_PATH: &str = "todos.json";

/// 将 TodoList 保存到文件
///
/// # 参数
/// - `todo_list`: 要保存的待办列表引用
/// - `path`: 文件路径
///
/// # 返回
/// 成功返回 `Ok(())`，失败返回 `TodoError`
///
/// # 示例
/// ```
/// use rust_todo_cli::todo_list::TodoList;
/// use rust_todo_cli::storage::save_to_file;
///
/// let todo_list = TodoList::new();
/// save_to_file(&todo_list, "test_todos.json").unwrap();
/// ```
pub fn save_to_file<P: AsRef<Path>>(todo_list: &TodoList, path: P) -> TodoResult<()> {
    // 序列化为 JSON（格式化输出，便于人类阅读）
    let json = serde_json::to_string_pretty(todo_list)?;
    
    // 写入文件
    fs::write(path, json)?;
    
    Ok(())
}

/// 从文件加载 TodoList
///
/// # 参数
/// - `path`: 文件路径
///
/// # 返回
/// 成功返回 `Ok(TodoList)`，失败返回 `TodoError`
/// 如果文件不存在，返回一个空的 TodoList
///
/// # 示例
/// ```
/// use rust_todo_cli::storage::load_from_file;
///
/// let todo_list = load_from_file("todos.json").unwrap();
/// println!("加载了 {} 个任务", todo_list.len());
/// ```
pub fn load_from_file<P: AsRef<Path>>(path: P) -> TodoResult<TodoList> {
    let path_ref = path.as_ref();
    
    // 如果文件不存在，返回空列表
    if !path_ref.exists() {
        return Ok(TodoList::new());
    }
    
    // 读取文件内容
    let content = fs:: read_to_string(path_ref)?;
    
    // 如果文件为空，返回空列表
    if content. trim().is_empty() {
        return Ok(TodoList::new());
    }
    
    // 反序列化 JSON
    let todo_list:  TodoList = serde_json:: from_str(&content)?;
    
    Ok(todo_list)
}

/// 检查文件是否存在
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path. as_ref().exists()
}

/// 删除数据文件
///
/// # 参数
/// - `path`: 文件路径
///
/// # 返回
/// 成功返回 `Ok(())`，失败返回 `TodoError`
pub fn delete_file<P: AsRef<Path>>(path: P) -> TodoResult<()> {
    let path_ref = path.as_ref();
    
    if path_ref.exists() {
        fs::remove_file(path_ref)?;
    }
    
    Ok(())
}

/// 备份数据文件
///
/// # 参数
/// - `source`: 源文件路径
/// - `backup`: 备份文件路径
///
/// # 返回
/// 成功返回 `Ok(())`，失败返回 `TodoError`
pub fn backup_file<P: AsRef<Path>, Q: AsRef<Path>>(source: P, backup: Q) -> TodoResult<()> {
    let source_ref = source. as_ref();
    
    if ! source_ref.exists() {
        return Err(TodoError:: Custom(
            format!("Source file '{}' does not exist", source_ref.display())
        ));
    }
    
    fs::copy(source_ref, backup)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Priority;
    use std::fs;

    // 测试辅助函数：创建临时测试文件路径
    fn test_file_path(name: &str) -> String {
        format!("test_{}.json", name)
    }

    // 测试后清理函数
    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_save_and_load() {
        let path = test_file_path("save_load");
        
        // 创建一个带任务的列表
        let mut original_list = TodoList::new();
        original_list.add_task("任务1".to_string(), Priority::High, None);
        original_list.add_task("任务2".to_string(), Priority::Medium, None);
        
        // 保存
        assert!(save_to_file(&original_list, &path).is_ok());
        
        // 加载
        let loaded_list = load_from_file(&path).unwrap();
        
        assert_eq!(loaded_list.len(), 2);
        assert_eq!(loaded_list.list_tasks()[0].title, "任务1");
        assert_eq!(loaded_list.list_tasks()[1].title, "任务2");
        
        cleanup(&path);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let path = test_file_path("nonexistent");
        
        // 加载不存在的文件应该返回空列表
        let result = load_from_file(&path);
        assert!(result. is_ok());
        
        let list = result.unwrap();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_load_empty_file() {
        let path = test_file_path("empty");
        
        // 创建空文件
        fs:: write(&path, "").unwrap();
        
        // 加载空文件应该返回空列表
        let result = load_from_file(&path);
        assert!(result.is_ok());
        
        let list = result.unwrap();
        assert_eq!(list.len(), 0);
        
        cleanup(&path);
    }

    #[test]
    fn test_file_exists() {
        let path = test_file_path("exists");
        
        assert! (!file_exists(&path));
        
        // 创建文件
        fs::write(&path, "test").unwrap();
        assert!(file_exists(&path));
        
        cleanup(&path);
    }

    #[test]
    fn test_delete_file() {
        let path = test_file_path("delete");
        
        // 创建文件
        fs::write(&path, "test").unwrap();
        assert!(file_exists(&path));
        
        // 删除
        assert!(delete_file(&path).is_ok());
        assert!(!file_exists(&path));
        
        // 删除不存在的文件不应该报错
        assert!(delete_file(&path).is_ok());
    }

    #[test]
    fn test_backup_file() {
        let source = test_file_path("backup_source");
        let backup = test_file_path("backup_dest");
        
        // 创建源文件
        let mut list = TodoList::new();
        list.add_task("备份测试".to_string(), Priority::Low, None);
        save_to_file(&list, &source).unwrap();
        
        // 备份
        assert!(backup_file(&source, &backup).is_ok());
        assert!(file_exists(&backup));
        
        // 验证备份内容
        let backup_list = load_from_file(&backup).unwrap();
        assert_eq!(backup_list.len(), 1);
        assert_eq!(backup_list.list_tasks()[0].title, "备份测试");
        
        cleanup(&source);
        cleanup(&backup);
    }

    #[test]
    fn test_backup_nonexistent_file() {
        let source = test_file_path("nonexistent_source");
        let backup = test_file_path("nonexistent_backup");
        
        // 备份不存在的文件应该失败
        let result = backup_file(&source, &backup);
        assert!(result.is_err());
        
        if let Err(TodoError::Custom(msg)) = result {
            assert!(msg.contains("does not exist"));
        } else {
            panic!("Expected Custom error");
        }
    }

    #[test]
    fn test_save_with_completed_tasks() {
        let path = test_file_path("completed");
        
        let mut list = TodoList::new();
        let id = list.add_task("已完成任务".to_string(), Priority::Medium, None);
        list.complete_task(id).unwrap();
        
        // 保存并加载
        save_to_file(&list, &path).unwrap();
        let loaded = load_from_file(&path).unwrap();
        
        assert_eq!(loaded.len(), 1);
        assert!(loaded. list_tasks()[0].completed);
        
        cleanup(&path);
    }

    #[test]
    fn test_json_format() {
        let path = test_file_path("format");
        
        let mut list = TodoList::new();
        list.add_task("格式测试".to_string(), Priority::High, None);
        
        save_to_file(&list, &path).unwrap();
        
        // 读取文件内容验证 JSON 格式
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("格式测试"));
        assert!(content.contains("High"));
        assert!(content.contains("tasks"));
        
        // 验证是格式化的 JSON（有换行）
        assert!(content.contains('\n'));
        
        cleanup(&path);
    }
}
