use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 任务优先级枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

/// 实现 Priority 的字符串转换，方便 CLI 使用
impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s:  &str) -> Result<Self, Self::Err> {
        match s. to_lowercase().as_str() {
            "high" | "h" => Ok(Priority:: High),
            "medium" | "m" => Ok(Priority:: Medium),
            "low" | "l" => Ok(Priority:: Low),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

/// 实现 Display trait 用于打印
impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt:: Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "High"),
            Priority::Medium => write!(f, "Medium"),
            Priority:: Low => write!(f, "Low"),
        }
    }
}

/// 待办任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// 任务唯一标识符
    pub id: u32,
    /// 任务标题
    pub title: String,
    /// 是否已完成
    pub completed: bool,
    /// 优先级
    pub priority: Priority,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 截止日期（可选）
    pub due_date: Option<DateTime<Utc>>,
}

impl Task {
    /// 创建新任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    /// - `title`: 任务标题
    /// - `priority`: 优先级
    /// - `due_date`: 可选的截止日期
    ///
    /// # 示例
    /// ```
    /// use rust_todo_cli::task::{Task, Priority};
    ///
    /// let task = Task:: new(1, "学习 Rust 所有权". to_string(), Priority::High, None);
    /// assert_eq!(task.id, 1);
    /// assert!(!task.completed);
    /// ```
    pub fn new(id:  u32, title: String, priority: Priority, due_date: Option<DateTime<Utc>>) -> Self {
        Self {
            id,
            title,
            completed: false,
            priority,
            created_at:  Utc::now(),
            due_date,
        }
    }

    /// 标记任务为已完成
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// 检查任务是否过期
    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            ! self.completed && Utc::now() > due
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let task = Task::new(1, "测试任务".to_string(), Priority::Medium, None);
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "测试任务");
        assert!(!task.completed);
        assert_eq!(task.priority, Priority::Medium);
    }

    #[test]
    fn test_complete_task() {
        let mut task = Task::new(1, "完成测试".to_string(), Priority::Low, None);
        assert! (!task.completed);
        task.complete();
        assert!(task.completed);
    }

    #[test]
    fn test_priority_from_string() {
        use std::str::FromStr;
        
        assert_eq!(Priority::from_str("high").unwrap(), Priority::High);
        assert_eq!(Priority:: from_str("H").unwrap(), Priority::High);
        assert_eq!(Priority:: from_str("medium").unwrap(), Priority::Medium);
        assert!(Priority::from_str("invalid").is_err());
    }

    #[test]
    fn test_is_overdue() {
        use chrono::Duration;

        // 未过期的任务
        let future_date = Utc::now() + Duration::days(1);
        let task = Task::new(1, "未来任务".to_string(), Priority::High, Some(future_date));
        assert!(!task.is_overdue());

        // 已过期的任务
        let past_date = Utc:: now() - Duration::days(1);
        let task = Task:: new(2, "过期任务".to_string(), Priority::High, Some(past_date));
        assert!(task.is_overdue());

        // 无截止日期
        let task = Task::new(3, "无期限". to_string(), Priority::Low, None);
        assert!(!task.is_overdue());
    }
}
