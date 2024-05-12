mod parser;

mod executor;

fn main() {
    let (task_properties_map, task_dependencies_map) = parser::parse_config();

    executor::execute(task_properties_map.clone(), task_dependencies_map.clone());
}
