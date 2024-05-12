use crate::parser::config_parser;
use queues::*;
use rusty_pool::ThreadPool;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod task;

pub fn execute(
    task_properties_map: Box<HashMap<String, config_parser::config_structs::TaskParams>>,
    task_dependencies_map: Box<HashMap<String, Vec<String>>>,
) {
    println!("task_properties_map: {:?}", task_properties_map);
    println!("task_dependencies_map: {:?}", task_dependencies_map);

    let task_name_to_indegrees_map: HashMap<String, u16> = HashMap::new();
    let tasks: Queue<String> = Queue::new();
    let task_queue_arc = Arc::new(Mutex::new(tasks));
    let task_name_to_indegrees_map_arc = Arc::new(Mutex::new(task_name_to_indegrees_map));
    let task_properties_map_arc = Arc::new(task_properties_map);
    let task_dependencies_map_arc = Arc::new(task_dependencies_map);

    let max_iterations = 10;
    let mut iterations = 0;

    let mut num_tasks_to_execute = 0;
    // let mut num_tasks_executed = 0;
    let num_tasks_executed_arc = Arc::new(Mutex::new(0));

    let pool = ThreadPool::default();

    {
        let mut task_name_to_indegrees_map = task_name_to_indegrees_map_arc.lock().unwrap();

        for (task_name, _) in task_dependencies_map_arc.iter() {
            num_tasks_to_execute += 1;
            task_name_to_indegrees_map.insert(task_name.clone(), 0);
        }

        for (_, dependencies) in task_dependencies_map_arc.iter() {
            for dependency in dependencies {
                let indegree = task_name_to_indegrees_map.get(dependency).unwrap() + 1;
                task_name_to_indegrees_map.insert(dependency.clone(), indegree);
            }
        }

        println!(
            "task_name_to_indegrees_map: {:?}",
            task_name_to_indegrees_map
        );

        {
            let mut tasks = task_queue_arc.lock().unwrap();
            for (task_name, _) in task_name_to_indegrees_map.iter() {
                if task_name_to_indegrees_map.get(task_name).unwrap() == &0 {
                    let _ = tasks.add(task_name.clone());
                }
            }
        }
    }

    while iterations < max_iterations
    /*&& num_tasks_executed < num_tasks_to_execute*/
    {
        {
            let mut tasks = task_queue_arc.lock().unwrap();
            while tasks.size() > 0 {
                let task = tasks.remove().unwrap();
                // num_tasks_executed += 1;
                let task_queue_clone = Arc::clone(&task_queue_arc);
                let task_name_to_indegrees_map_arc_clone =
                    Arc::clone(&task_name_to_indegrees_map_arc);

                let task_properties_map_arc_clone = Arc::clone(&task_properties_map_arc);
                let task_dependencies_map_arc_clone = Arc::clone(&task_dependencies_map_arc);
                let num_tasks_executed_arc_clone = Arc::clone(&num_tasks_executed_arc);

                pool.execute(move || {
                    println!("Executing task: {}", task.clone());
                    let t = task::Task::new(
                        task.clone(),
                        task_properties_map_arc_clone.get(&task.clone()).unwrap(),
                    );

                    t.execute();
                    let mut queue_internal_thread = task_queue_clone.lock().unwrap();

                    let task_dependencies = task_dependencies_map_arc_clone.get(&task).unwrap();

                    let mut task_name_to_indegrees_map =
                        task_name_to_indegrees_map_arc_clone.lock().unwrap();

                    for dependency in task_dependencies {
                        let new_indegree = task_name_to_indegrees_map.get(dependency).unwrap() - 1;
                        task_name_to_indegrees_map.insert(dependency.clone(), new_indegree);
                        if new_indegree == 0 {
                            println!(
                                "Task {} now has no dependencies, putting in the queue",
                                dependency.clone()
                            );
                            let _ = queue_internal_thread.add(dependency.clone());
                        }
                    }

                    let mut num_tasks_executed_arc_clone_val =
                        num_tasks_executed_arc_clone.lock().unwrap();
                    *num_tasks_executed_arc_clone_val += 1;
                });
            }
            let num_tasks_executed = num_tasks_executed_arc.lock().unwrap();
            if *num_tasks_executed == num_tasks_to_execute {
                break;
            }
        }
        iterations += 1;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    let num_tasks_executed = num_tasks_executed_arc.lock().unwrap();
    if *num_tasks_executed == num_tasks_to_execute {
        println!("All {} tasks have been executed", num_tasks_to_execute);
    } else if iterations < max_iterations {
        println!(
            "There is a cyclic dependency between tasks. Executed {} tasks out of {}",
            *num_tasks_executed, num_tasks_to_execute
        )
    } else {
        println!(
            "Reached timeout. Executed {} tasks out of {}",
            *num_tasks_executed, num_tasks_to_execute
        )
    }
}
