use clap::{Parser, Subcommand};

/// 命令行待办事项管理器
#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "A simple and elegant CLI todo list manager", long_about = None)]
pub struct Cli {
    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

/// 所有可用的命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 添加新任务
    #[command(alias = "a")]
    Add {
        /// 任务标题
        title: String,
        
        /// 优先级:  high (h), medium (m), low (l)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        
        /// 截止日期 (格式: YYYY-MM-DD)
        #[arg(short, long)]
        due: Option<String>,
    },

    /// 列出任务
    #[command(alias = "ls")]
    List {
        /// 筛选选项
        #[arg(value_enum)]
        filter: Option<ListFilter>,
    },

    /// 标记任务为已完成
    #[command(alias = "c")]
    Complete {
        /// 任务 ID
        id: u32,
    },

    /// 删除任务
    #[command(alias = "d")]
    Delete {
        /// 任务 ID
        id: u32,
    },

    /// 显示任务详情
    #[command(alias = "s")]
    Show {
        /// 任务 ID
        id: u32,
    },

    /// 清除所有已完成的任务
    Clear {
        /// 跳过确认提示
        #[arg(short, long)]
        force: bool,
    },
}

/// 列表筛选选项
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ListFilter {
    /// 所有任务
    All,
    /// 待办任务（未完成）
    Pending,
    /// 已完成任务
    Completed,
    /// 过期任务
    Overdue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_add_command() {
        let cli = Cli::parse_from(vec! ["todo", "add", "测试任务", "--priority", "high"]);
        
        match cli.command {
            Commands:: Add { title, priority, ..  } => {
                assert_eq!(title, "测试任务");
                assert_eq!(priority, "high");
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_cli_list_command() {
        let cli = Cli::parse_from(vec! ["todo", "list", "pending"]);
        
        match cli.command {
            Commands::List { filter } => {
                assert!(filter.is_some());
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_complete_command() {
        let cli = Cli::parse_from(vec!["todo", "complete", "42"]);
        
        match cli.command {
            Commands::Complete { id } => {
                assert_eq!(id, 42);
            }
            _ => panic!("Expected Complete command"),
        }
    }

    #[test]
    fn test_cli_alias() {
        // 测试 'a' 别名
        let cli = Cli::parse_from(vec!["todo", "a", "任务"]);
        assert!(matches!(cli.command, Commands::Add { .. }));
        
        // 测试 'ls' 别名
        let cli = Cli::parse_from(vec!["todo", "ls"]);
        assert!(matches!(cli.command, Commands::List { ..  }));
    }
}
