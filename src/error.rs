use std::fmt;

/// 自定义错误类型
#[derive(Debug)]
pub enum TodoError {
    /// 任务未找到
    TaskNotFound(u32),
    /// IO 错误（文件读写）
    IoError(std::io::Error),
    /// JSON 序列化/反序列化错误
    SerdeError(serde_json:: Error),
    /// 无效的优先级
    InvalidPriority(String),
    /// 无效的日期格式
    InvalidDateFormat(String),
    /// 其他自定义错误
    Custom(String),
}

/// 实现 Display trait，用于友好的错误信息显示
impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt:: Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::TaskNotFound(id) => {
                write!(f, "❌ Task with ID {} not found", id)
            }
            TodoError::IoError(err) => {
                write!(f, "❌ File operation failed: {}", err)
            }
            TodoError::SerdeError(err) => {
                write!(f, "❌ JSON parsing failed: {}", err)
            }
            TodoError::InvalidPriority(priority) => {
                write!(f, "❌ Invalid priority '{}'.  Use:  high, medium, or low", priority)
            }
            TodoError::InvalidDateFormat(date) => {
                write!(f, "❌ Invalid date format '{}'. Expected:  YYYY-MM-DD", date)
            }
            TodoError::Custom(msg) => {
                write!(f, "❌ Error: {}", msg)
            }
        }
    }
}

/// 实现标准库的 Error trait
impl std::error::Error for TodoError {
    fn source(&self) -> Option<&(dyn std:: error::Error + 'static)> {
        match self {
            TodoError::IoError(err) => Some(err),
            TodoError::SerdeError(err) => Some(err),
            _ => None,
        }
    }
}

/// 自动从 std::io::Error 转换为 TodoError
impl From<std::io::Error> for TodoError {
    fn from(err: std::io::Error) -> Self {
        TodoError:: IoError(err)
    }
}

/// 自动从 serde_json::Error 转换为 TodoError
impl From<serde_json::Error> for TodoError {
    fn from(err: serde_json::Error) -> Self {
        TodoError::SerdeError(err)
    }
}

/// 自定义 Result 类型别名，简化函数签名
pub type TodoResult<T> = Result<T, TodoError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_not_found_error() {
        let error = TodoError::TaskNotFound(42);
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("42"));
        assert!(error_msg.contains("not found"));
    }

    #[test]
    fn test_invalid_priority_error() {
        let error = TodoError::InvalidPriority("urgent".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("urgent"));
        assert!(error_msg.contains("high, medium, or low"));
    }

    #[test]
    fn test_custom_error() {
        let error = TodoError::Custom("Something went wrong".to_string());
        let error_msg = format! ("{}", error);
        assert!(error_msg.contains("Something went wrong"));
    }

    #[test]
    fn test_error_conversion_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let todo_error:  TodoError = io_error.into();
        
        match todo_error {
            TodoError::IoError(_) => (),
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_error() -> TodoResult<()> {
            Err(TodoError:: TaskNotFound(1))
        }
        
        assert!(returns_error().is_err());
    }
}
