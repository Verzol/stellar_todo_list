# Soroban To-Do List Contract

A simple smart contract built with **Rust** and the **Soroban SDK** to manage a per-user to-do list on the blockchain.

## 🚀 Features

* **`add_task(user: Address, description: String)`**: Adds a new task for a specific user.
* **`mark_done(user: Address, task_id: u32)`**: Marks an existing task as completed by its ID.
* **`get_tasks_by_user(user: Address) -> Vec<Task>`**: Retrieves the entire list of tasks (both pending and completed) for a user.

## Data Structure

Each `Task` is stored using the following struct:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}
