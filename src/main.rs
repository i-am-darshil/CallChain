// use std::collections::HashMap;

use rayon::ThreadPoolBuilder;
use rusty_pool::ThreadPool;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use queues::*;

mod parser;

mod executor;

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    dependencies: Vec<usize>,
}
impl Task {
    fn new(id: usize, dependencies: Vec<usize>) -> Task {
        Task { id, dependencies }
    }
    fn execute(&self) {
        println!("Task {:?} executed", self);
    }
}

fn main() {
    let (task_properties_map, task_dependencies_map) = parser::parse_config();

    executor::execute(task_properties_map.clone(), task_dependencies_map.clone());

    // println!("task_dependencies_map: {:?}", task_dependencies_map);
    // println!("task_properties_map: {:?}", task_properties_map);

    // task_dependencies_map.as_mut().insert("k", vec!["a"]);

    // Sample tasks with dependencies
    // let tasks = vec![
    //     Task::new(1, vec![]),
    //     Task::new(2, vec![1]),
    //     Task::new(3, vec![2]),
    //     Task::new(4, vec![3]),
    // ];

    // let mut tasks: Queue<Task> = Queue::new();
    // tasks.add(Task::new(1, vec![]));
    // tasks.add(Task::new(2, vec![1]));
    // tasks.add(Task::new(3, vec![2]));
    // tasks.add(Task::new(4, vec![3]));

    // // Arc and Mutex to share the task queue among threads safely
    // let task_queue = Arc::new(Mutex::new(tasks));

    // // Create default `ThreadPool` configuration with the number of CPUs as core pool size
    // let pool = ThreadPool::default();

    // // Main loop to periodically spawn tasks
    // loop {
    //     {
    //         let mut tasks = task_queue.lock().unwrap();

    //         // Check if there are any tasks in the queue
    //         while tasks.size() > 0 {
    //             let task = tasks.remove().unwrap();
    //             let task_queue_internal_thread_clone = Arc::clone(&task_queue);

    //             // Spawn threads for each task using the thread pool
    //             // for task in tasks.iter() {
    //             pool.execute(move || {
    //                 thread::sleep(Duration::from_secs(3));
    //                 task.execute();

    //                 let mut queue_internal_thread =
    //                     task_queue_internal_thread_clone.lock().unwrap();

    //                 for &dependency in &task.dependencies {
    //                     queue_internal_thread.add(Task::new(dependency, vec![]));
    //                 }
    //             });
    //         }
    //     }

    //     std::thread::sleep(std::time::Duration::from_secs(2));
    // }
}
