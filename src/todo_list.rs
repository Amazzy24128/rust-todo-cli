use crate::task::{Priority, Task};
use crate::error::{TodoError, TodoResult};
use chrono::{DateTime, Utc};

/// 待办事项列表管理器
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    /// 创建一个新的空待办列表
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    /// 添加新任务
    pub fn add_task(&mut self, title: String, priority:  Priority, due_date: Option<DateTime<Utc>>) -> u32 {
        let id = self.next_id;
        let task = Task:: new(id, title, priority, due_date);
        self.tasks.push(task);
        self.next_id += 1;
        id
    }

    /// 获取所有任务的不可变引用
    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// 获取待办任务（未完成）
    pub fn list_pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| !task.completed).collect()
    }

    /// 获取已完成任务
    pub fn list_completed_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.completed).collect()
    }

    /// 根据 ID 查找任务的可变引用
    fn find_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    /// 根据 ID 查找任务的不可变引用
    pub fn find_task(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    /// 标记任务为已完成 - 使用新的错误类型
    pub fn complete_task(&mut self, id: u32) -> TodoResult<()> {
        match self.find_task_mut(id) {
            Some(task) => {
                task.complete();
                Ok(())
            }
            None => Err(TodoError::TaskNotFound(id)),
        }
    }

    /// 删除任务 - 使用新的错误类型
    pub fn delete_task(&mut self, id: u32) -> TodoResult<()> {
        let original_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        
        if self.tasks.len() < original_len {
            Ok(())
        } else {
            Err(TodoError::TaskNotFound(id))
        }
    }

    /// 获取任务总数
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// 检查列表是否为空
    pub fn is_empty(&self) -> bool {
        self.tasks. is_empty()
    }

    /// 按优先级排序任务
    pub fn tasks_by_priority(&self) -> Vec<&Task> {
        let mut tasks:  Vec<&Task> = self. tasks.iter().collect();
        tasks.sort_by(|a, b| {
            use Priority::*;
            let priority_order = |p: &Priority| match p {
                High => 0,
                Medium => 1,
                Low => 2,
            };
            priority_order(&a.priority).cmp(&priority_order(&b.priority))
        });
        tasks
    }

    /// 获取过期任务
    pub fn overdue_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.is_overdue()).collect()
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_task_with_error() {
        let mut list = TodoList::new();
        let id = list.add_task("测试". to_string(), Priority::Medium, None);
        
        // 成功情况
        assert!(list.complete_task(id).is_ok());
        
        // 失败情况 - 返回 TaskNotFound 错误
        let result = list.complete_task(999);
        assert!(result.is_err());
        
        if let Err(TodoError::TaskNotFound(missing_id)) = result {
            assert_eq!(missing_id, 999);
        } else {
            panic!("Expected TaskNotFound error");
        }
    }

    #[test]
    fn test_delete_task_with_error() {
        let mut list = TodoList::new();
        let id = list.add_task("测试".to_string(), Priority::Low, None);
        
        assert!(list.delete_task(id).is_ok());
        
        let result = list.delete_task(999);
        assert!(result. is_err());
        assert!(matches!(result, Err(TodoError::TaskNotFound(999))));
    }
}
