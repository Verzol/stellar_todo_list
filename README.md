# Soroban To-Do List Contract

Một smart contract đơn giản được xây dựng bằng **Rust** và **Soroban SDK** để quản lý danh sách công việc (to-do list) cho người dùng trên blockchain.

## 🚀 Tính năng

* `add_task(user: Address, description: String)`: Thêm một công việc mới cho một user cụ thể.
* `mark_done(user: Address, task_id: u32)`: Đánh dấu một công việc đã hoàn thành dựa trên ID của nó.
* `get_tasks_by_user(user: Address) -> Vec<Task>`: Lấy về toàn bộ danh sách công việc (đã hoàn thành và chưa hoàn thành) của một user.

## Cấu trúc dữ liệu

Mỗi `Task` được lưu trữ với cấu trúc:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}
