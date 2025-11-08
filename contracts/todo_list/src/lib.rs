#![no_std]

// Thêm `contracttype`
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, log, 
    Address, Env, String, Symbol, Vec,
};

#[derive(Clone)]
#[contract]
pub struct TodoContract;

// Thêm #[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

const TASKS: Symbol = symbol_short!("TASKS");

#[contractimpl]
impl TodoContract {
    /// 🟢 Thêm công việc mới
    pub fn add_task(env: Env, user: Address, description: String) {
        // SỬA LỖI 3: Sửa lại logic `unwrap_or`
        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&(TASKS, &user))
            .unwrap_or(Vec::new(&env)); // <-- Đã sửa

        let id = tasks.len() as u32 + 1;
        let new_task = Task {
            id,
            description: description.clone(),
            done: false,
        };

        tasks.push_back(new_task);
        env.storage().persistent().set(&(TASKS, &user), &tasks);

        log!(&env, "Task added for user: {:?}", user);
    }

    /// 🟡 Đánh dấu task là hoàn thành
    pub fn mark_done(env: Env, user: Address, task_id: u32) -> bool {
        // SỬA LỖI 3: Sửa lại logic `unwrap_or`
        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&(TASKS, &user))
            .unwrap_or(Vec::new(&env)); // <-- Đã sửa

        // SỬA LỖI 2: Viết lại logic vòng lặp
        for (i, task) in tasks.iter().enumerate() {
            if task.id == task_id {
                // Tạo task mới
                let updated_task = Task {
                    id: task.id,
                    description: task.description,
                    done: true,
                };
                
                // Thay thế task cũ
                tasks.set(i as u32, updated_task);

                // Lưu lại storage
                env.storage().persistent().set(&(TASKS, &user), &tasks);
                log!(&env, "Task {} marked done", task_id);
                return true;
            }
        }

        false // Không tìm thấy task
    }

    /// 🔵 Lấy danh sách task của 1 user
    pub fn get_tasks_by_user(env: Env, user: Address) -> Vec<Task> {
        // SỬA LỖI 3: Sửa lại logic `unwrap_or`
        env.storage()
            .persistent()
            .get(&(TASKS, &user))
            .unwrap_or(Vec::new(&env)) // <-- Đã sửa
    }
}