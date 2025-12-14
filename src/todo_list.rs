use crate::task::{Priority, Task};
use chrono::{DateTime, Utc};

/// 待办事项列表管理器
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    /// 创建一个新的空待办列表
    ///
    /// # 示例
    /// ```
    /// use rust_todo_cli::todo_list::TodoList;
    ///
    /// let todo_list = TodoList:: new();
    /// assert_eq!(todo_list.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    /// 添加新任务
    ///
    /// # 参数
    /// - `title`: 任务标题
    /// - `priority`: 优先级
    /// - `due_date`: 可选的截止日期
    ///
    /// # 返回
    /// 返回新创建任务的 ID
    ///
    /// # 示例
    /// ```
    /// use rust_todo_cli::todo_list::TodoList;
    /// use rust_todo_cli::task:: Priority;
    ///
    /// let mut todo_list = TodoList::new();
    /// let id = todo_list.add_task("学习 Rust". to_string(), Priority::High, None);
    /// assert_eq!(id, 1);
    /// ```
    pub fn add_task(&mut self, title: String, priority: Priority, due_date: Option<DateTime<Utc>>) -> u32 {
        let id = self.next_id;
        let task = Task::new(id, title, priority, due_date);
        self.tasks.push(task);
        self.next_id += 1;
        id
    }

    /// 获取所有任务的不可变引用
    ///
    /// # 返回
    /// 返回任务切片的引用
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
    ///
    /// # 参数
    /// - `id`: 任务 ID
    ///
    /// # 返回
    /// 如果找到返回 `Some(&mut Task)`，否则返回 `None`
    fn find_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    /// 根据 ID 查找任务的不可变引用
    pub fn find_task(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    /// 标记任务为已完成
    ///
    /// # 参数
    /// - `id`: 任务 ID
    ///
    /// # 返回
    /// 成功返回 `Ok(())`，任务不存在返回 `Err`
    ///
    /// # 示例
    /// ```
    /// use rust_todo_cli::todo_list::TodoList;
    /// use rust_todo_cli::task::Priority;
    ///
    /// let mut todo_list = TodoList::new();
    /// let id = todo_list.add_task("完成作业".to_string(), Priority::Medium, None);
    /// assert!(todo_list.complete_task(id).is_ok());
    /// ```
    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        match self.find_task_mut(id) {
            Some(task) => {
                task.complete();
                Ok(())
            }
            None => Err(format!("Task with id {} not found", id)),
        }
    }

    /// 删除任务
    ///
    /// # 参数
    /// - `id`: 任务 ID
    ///
    /// # 返回
    /// 成功返回 `Ok(())`，任务不存在返回 `Err`
    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        let original_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        
        if self.tasks.len() < original_len {
            Ok(())
        } else {
            Err(format!("Task with id {} not found", id))
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

    /// 按优先级排序任务（返回新的排序后的引用列表）
    pub fn tasks_by_priority(&self) -> Vec<&Task> {
        let mut tasks:  Vec<&Task> = self. tasks.iter().collect();
        tasks.sort_by(|a, b| {
            use Priority::*;
            let priority_order = |p: &Priority| match p {
                High => 0,
                Medium => 1,
                Low => 2,
            };
            priority_order(&a. priority).cmp(&priority_order(&b.priority))
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
        Self:: new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo_list() {
        let list = TodoList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_add_task() {
        let mut list = TodoList::new();
        let id1 = list.add_task("任务1".to_string(), Priority::High, None);
        let id2 = list.add_task("任务2".to_string(), Priority::Low, None);
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_complete_task() {
        let mut list = TodoList::new();
        let id = list.add_task("测试完成".to_string(), Priority::Medium, None);
        
        assert!(list.complete_task(id).is_ok());
        let task = list.find_task(id).unwrap();
        assert!(task.completed);
        
        // 测试完成不存在的任务
        assert!(list.complete_task(999).is_err());
    }

    #[test]
    fn test_delete_task() {
        let mut list = TodoList::new();
        let id = list.add_task("待删除". to_string(), Priority::Low, None);
        
        assert_eq!(list.len(), 1);
        assert!(list.delete_task(id).is_ok());
        assert_eq!(list.len(), 0);
        
        // 测试删除不存在的任务
        assert!(list.delete_task(999).is_err());
    }

    #[test]
    fn test_list_pending_and_completed() {
        let mut list = TodoList::new();
        let _id1 = list. add_task("任务1".to_string(), Priority::High, None);
        let id2 = list.add_task("任务2".to_string(), Priority::Medium, None);
        let _id3 = list.add_task("任务3".to_string(), Priority::Low, None);
        
        list.complete_task(id2).unwrap();
        
        let pending = list.list_pending_tasks();
        let completed = list.list_completed_tasks();
        
        assert_eq!(pending.len(), 2);
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].id, id2);
    }

    #[test]
    fn test_tasks_by_priority() {
        let mut list = TodoList::new();
        list.add_task("低优先级". to_string(), Priority::Low, None);
        list.add_task("高优先级".to_string(), Priority::High, None);
        list.add_task("中优先级".to_string(), Priority::Medium, None);
        
        let sorted = list.tasks_by_priority();
        assert_eq!(sorted[0].priority, Priority::High);
        assert_eq!(sorted[1].priority, Priority::Medium);
        assert_eq!(sorted[2].priority, Priority::Low);
    }

    #[test]
    fn test_overdue_tasks() {
        use chrono::Duration;
        
        let mut list = TodoList:: new();
        let past = Utc::now() - Duration::days(1);
        let future = Utc::now() + Duration::days(1);
        
        list.add_task("过期任务".to_string(), Priority::High, Some(past));
        list.add_task("未来任务".to_string(), Priority::Medium, Some(future));
        list.add_task("无期限". to_string(), Priority::Low, None);
        
        let overdue = list.overdue_tasks();
        assert_eq!(overdue.len(), 1);
        assert_eq!(overdue[0].title, "过期任务");
    }
}
