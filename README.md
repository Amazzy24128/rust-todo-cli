# 🦀 Rust Todo CLI

一个优雅的命令行待办事项管理器，使用 Rust 编写。

## ✨ 特性

- ✅ 添加、完成、删除任务
- 🎯 三级优先级（高、中、低）
- 📅 支持截止日期
- 🎨 彩色终端输出
- 💾 JSON 文件持久化
- ⚡ 快速、安全、可靠

## 🚀 安装

```bash
# 克隆仓库
git clone https://github.com/Amazzy24128/rust-todo-cli.git
cd rust-todo-cli

# 构建项目
cargo build --release

# 可选：安装到系统
cargo install --path . 
```

## 📖 使用方法

### 添加任务

```bash
# 基本用法
todo add "学习 Rust"

# 指定优先级
todo add "重要任务" --priority high
todo add "普通任务" -p medium

# 添加截止日期
todo add "项目截止" -p high --due 2025-12-31

# 使用别名
todo a "快速添加任务"
```

### 列出任务

```bash
# 列出所有任务
todo list

# 列出待办任务
todo list pending

# 列出已完成任务
todo list completed

# 列出过期任务
todo list overdue

# 使用别名
todo ls
```

### 完成任务

```bash
# 标记任务为已完成
todo complete 1

# 使用别名
todo c 1
```

### 查看任务详情

```bash
todo show 1
todo s 1
```

### 删除任务

```bash
todo delete 1
todo d 1
```

### 清除已完成任务

```bash
# 需要确认
todo clear

# 跳过确认
todo clear --force
```

## 🎨 示例输出

```
📋 All Tasks
────────────────────────────────────────────────────────────
○ [  1] HIGH | 学习 Rust 所有权系统
○ [  2] MED  | 完成项目文档 📅 2025-12-31
✓ [  3] LOW  | 修复 bug

────────────────────────────────────────────────────────────
3 task(s)

📊 Statistics
  Total:     3
  Pending:   2
  Completed:  1
```

## 🏗️ 项目结构

```
src/
├── main.rs          # 主程序入口
├── lib.rs           # 库根模块
├── task.rs          # Task 结构体
├── todo_list.rs     # TodoList 管理器
├── storage.rs       # 文件持久化
├── cli.rs           # CLI 参数解析
├── display.rs       # 终端输出格式化
└── error.rs         # 错误类型定义
```

## 🧪 运行测试

```bash
cargo test
```

## 📝 技术栈

- **Rust 2021 Edition**
- **clap** - CLI 参数解析
- **serde** - 序列化/反序列化
- **chrono** - 日期时间处理
- **colored** - 终端彩色输出

## 🎓 学习要点

这个项目展示了 Rust 的多个核心特性：

- ✅ 所有权和借用
- ✅ 结构体和枚举
- ✅ 错误处理（Result 和 Option）
- ✅ Trait 实现
- ✅ 模式匹配
- ✅ 模块化设计
- ✅ 文件 IO
- ✅ 单元测试和集成测试

## 📄 许可证

MIT License

## 👤 作者

Amazzy24128
