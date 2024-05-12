use config_structs::*;
use pest::Parser;
use pest_derive::Parser;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

pub mod config_structs {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct TaskParams {
        pub chain_name: String,
        pub method_name: String,
        pub properties_map: HashMap<String, String>,
        pub method_params_json_string: String,
    }
}

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIParser;

pub fn parse() -> (
    Box<HashMap<String, TaskParams>>,
    Box<HashMap<String, Vec<String>>>,
) {
    let mut task_properties_map: HashMap<String, TaskParams> = HashMap::new();
    let mut task_dependencies_map: HashMap<String, Vec<String>> = HashMap::new();

    let unparsed_file = fs::read_to_string("./src/config/config.ini").expect("cannot read file");

    let file = INIParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    let mut method_params_all_map: HashMap<String, Value> = HashMap::new();
    method_params_all_map.insert(String::from("id"), json!(1));
    method_params_all_map.insert(String::from("jsonrpc"), json!(String::from("2.0")));

    for task_declaration in file.into_inner() {
        let mut current_task_name = String::from("");
        let mut chain_name = String::from("");
        let mut method_name = String::from("");
        let mut properties_map: HashMap<String, String> = HashMap::new();
        let mut method_params_json_string = String::new();
        let task_declaration_inners = task_declaration.into_inner();

        for task_declaration_inner in task_declaration_inners {
            match task_declaration_inner.as_rule() {
                Rule::task_name => {
                    current_task_name =
                        String::from(task_declaration_inner.into_inner().next().unwrap().as_str());
                    if !task_dependencies_map.contains_key(&current_task_name) {
                        task_dependencies_map.insert(current_task_name.clone(), Vec::new());
                    }
                    println!("Task: {}", current_task_name);
                }
                Rule::task_body => {
                    let task_body_inners = task_declaration_inner.into_inner();
                    for task_body_inner in task_body_inners {
                        match task_body_inner.as_rule() {
                            Rule::depends_on_command => {
                                let upstream_task_names = task_body_inner.into_inner();
                                for upstream_task_name in upstream_task_names {
                                    let upstream_task_name_string = String::from(
                                        upstream_task_name.into_inner().next().unwrap().as_str(),
                                    );
                                    let upstream_task_name_depedency_vector = task_dependencies_map
                                        .entry(upstream_task_name_string.clone())
                                        .or_default();
                                    upstream_task_name_depedency_vector
                                        .push(current_task_name.clone());

                                    println!(
                                        "Upstream task of {} is {}: {:?}",
                                        current_task_name,
                                        upstream_task_name_string,
                                        task_dependencies_map
                                    );
                                }
                            }
                            Rule::select_command => {
                                chain_name = String::from(
                                    task_body_inner.into_inner().next().unwrap().as_str(),
                                );
                                println!("chain_name: {}", chain_name);
                            }
                            Rule::use_command => {
                                method_name = String::from(
                                    task_body_inner.into_inner().next().unwrap().as_str(),
                                );
                                method_params_all_map
                                    .insert(String::from("method"), json!(method_name));
                                println!("method_name: {}", method_name);
                            }
                            Rule::properties_command => {
                                let property_declarations = task_body_inner.into_inner();

                                for property_declaration in property_declarations {
                                    let mut property_declaration_inner =
                                        property_declaration.into_inner();
                                    let property_name = String::from(
                                        property_declaration_inner.next().unwrap().as_str(),
                                    );
                                    let property_value = String::from(
                                        property_declaration_inner.next().unwrap().as_str(),
                                    );
                                    properties_map
                                        .insert(property_name.clone(), property_value.clone());
                                    println!(
                                        "property_name: {}, property_value: {}",
                                        property_name, property_value
                                    );
                                }
                            }
                            Rule::method_params_command => {
                                let mut method_params_list: Vec<Value> = Vec::new();

                                let mut method_param_declarations = task_body_inner.into_inner();
                                let entire_method_params_type = String::from(
                                    method_param_declarations.next().unwrap().as_str(),
                                )
                                .to_lowercase();

                                for method_param_declaration in method_param_declarations {
                                    let method_param_declaration_inner =
                                        method_param_declaration.into_inner();

                                    for method_param_type_declaration in
                                        method_param_declaration_inner
                                    {
                                        match method_param_type_declaration.as_rule() {
                                            Rule::method_param_is_object_declaration => {
                                                let mut param_obj_map: HashMap<String, Value> =
                                                    HashMap::new();
                                                let method_param_is_object_declarations =
                                                    method_param_type_declaration.into_inner();
                                                for method_param_is_object_declaration in
                                                    method_param_is_object_declarations
                                                {
                                                    let mut method_param_inside_object_declaration =
                                                        method_param_is_object_declaration
                                                            .into_inner();
                                                    let param_name = String::from(
                                                        method_param_inside_object_declaration
                                                            .next()
                                                            .unwrap()
                                                            .as_str(),
                                                    );

                                                    let param_type = String::from(
                                                        method_param_inside_object_declaration
                                                            .next()
                                                            .unwrap()
                                                            .as_str(),
                                                    )
                                                    .to_lowercase();

                                                    if param_type == "string" {
                                                        let param_value = String::from(
                                                            method_param_inside_object_declaration
                                                                .next()
                                                                .unwrap()
                                                                .as_str(),
                                                        );

                                                        param_obj_map.insert(
                                                            param_name.clone(),
                                                            json!(param_value),
                                                        );
                                                    } else if param_type == "number" {
                                                        let param_value =
                                                            method_param_inside_object_declaration
                                                                .next()
                                                                .unwrap()
                                                                .as_str()
                                                                .parse::<i64>()
                                                                .unwrap();

                                                        param_obj_map.insert(
                                                            param_name.clone(),
                                                            json!(param_value),
                                                        );
                                                    } else if param_type == "list_of_string" {
                                                        let mut method_object_params_list: Vec<
                                                            Value,
                                                        > = Vec::new();
                                                        for method_param_value in
                                                            method_param_inside_object_declaration
                                                        {
                                                            let param_value = String::from(
                                                                method_param_value.as_str(),
                                                            );
                                                            method_object_params_list
                                                                .push(json!(param_value));
                                                        }

                                                        param_obj_map.insert(
                                                            param_name.clone(),
                                                            json!(method_object_params_list),
                                                        );
                                                    } else if param_type == "list_of_number" {
                                                        let mut method_object_params_list: Vec<
                                                            Value,
                                                        > = Vec::new();
                                                        for method_param_value in
                                                            method_param_inside_object_declaration
                                                        {
                                                            let param_value = method_param_value
                                                                .as_str()
                                                                .parse::<i64>()
                                                                .unwrap();
                                                            method_object_params_list
                                                                .push(json!(param_value));
                                                        }
                                                        param_obj_map.insert(
                                                            param_name.clone(),
                                                            json!(method_object_params_list),
                                                        );
                                                    }
                                                }
                                                method_params_list.push(json!(param_obj_map));
                                            }
                                            Rule::method_param_is_non_object_declaration => {
                                                let mut method_param_is_non_object_declaration =
                                                    method_param_type_declaration.into_inner();

                                                let param_type = String::from(
                                                    method_param_is_non_object_declaration
                                                        .next()
                                                        .unwrap()
                                                        .as_str(),
                                                )
                                                .to_lowercase();

                                                if param_type == "string" {
                                                    let param_value = String::from(
                                                        method_param_is_non_object_declaration
                                                            .next()
                                                            .unwrap()
                                                            .as_str(),
                                                    );
                                                    method_params_list.push(json!(param_value));
                                                } else if param_type == "number" {
                                                    let param_value =
                                                        method_param_is_non_object_declaration
                                                            .next()
                                                            .unwrap()
                                                            .as_str()
                                                            .parse::<i64>()
                                                            .unwrap();
                                                    method_params_list.push(json!(param_value));
                                                } else if param_type == "boolean" {
                                                    let param_value =
                                                        method_param_is_non_object_declaration
                                                            .next()
                                                            .unwrap()
                                                            .as_str()
                                                            .parse::<bool>()
                                                            .unwrap();
                                                    method_params_list.push(json!(param_value));
                                                }
                                            }
                                            Rule::EOI => (),
                                            _ => unreachable!(),
                                        }
                                    }
                                }

                                if entire_method_params_type == "list" {
                                    method_params_all_map
                                        .insert(String::from("params"), json!(method_params_list));
                                } else {
                                    if method_params_list.len() > 0 {
                                        method_params_all_map.insert(
                                            String::from("params"),
                                            json!(method_params_list[0]),
                                        );
                                    }
                                }

                                method_params_json_string =
                                    serde_json::to_string(&method_params_all_map).unwrap();

                                println!(
                                    "method_params_list: {:?}, method_params_json_string: {:?}",
                                    method_params_list, method_params_json_string
                                );
                            }
                            Rule::EOI => (),
                            _ => unreachable!(),
                        }
                    }
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        task_properties_map.insert(
            current_task_name.clone(),
            TaskParams {
                chain_name: chain_name.clone(),
                method_name: method_name.clone(),
                properties_map: properties_map,
                method_params_json_string: method_params_json_string,
            },
        );
    }

    return (
        Box::new(task_properties_map),
        Box::new(task_dependencies_map),
    );
}
