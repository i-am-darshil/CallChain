use pest::Parser;
use pest_derive::Parser;

use std::collections::HashMap;
use std::fs;

use config_structs::*;

pub mod config_structs {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct TaskParams {
        pub chain_name: String,
        pub method_name: String,
        pub properties_map: HashMap<String, String>,
        pub method_params: MethodParams,
    }

    #[derive(Debug, Clone)]
    pub struct MethodParams {
        pub string_params_map: HashMap<String, String>,
        pub num_params_map: HashMap<String, u128>,
        pub string_vec_params_map: HashMap<String, Vec<String>>,
        pub num_vec_params_map: HashMap<String, Vec<u128>>,
    }

    impl MethodParams {
        pub fn empty() -> MethodParams {
            MethodParams {
                string_params_map: HashMap::new(),
                num_params_map: HashMap::new(),
                string_vec_params_map: HashMap::new(),
                num_vec_params_map: HashMap::new(),
            }
        }
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

    for task_declaration in file.into_inner() {
        let mut current_task_name = String::from("");
        let mut chain_name = String::from("");
        let mut method_name = String::from("");
        let mut properties_map: HashMap<String, String> = HashMap::new();
        let mut method_params = MethodParams::empty();
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
                                let mut string_params_map: HashMap<String, String> = HashMap::new();
                                let mut num_params_map: HashMap<String, u128> = HashMap::new();
                                let mut string_vec_params_map: HashMap<String, Vec<String>> =
                                    HashMap::new();
                                let mut num_vec_params_map: HashMap<String, Vec<u128>> =
                                    HashMap::new();

                                let method_param_declarations = task_body_inner.into_inner();
                                for method_param_declaration in method_param_declarations {
                                    let mut method_param_declaration_inner =
                                        method_param_declaration.into_inner();
                                    let method_param_name = String::from(
                                        method_param_declaration_inner.next().unwrap().as_str(),
                                    )
                                    .clone();
                                    println!("method_param_name: {}", method_param_name);

                                    let method_param_type =
                                        method_param_declaration_inner.next().unwrap().as_str();
                                    println!("method_param_type: {}", method_param_type);

                                    if method_param_type == "list_of_string" {
                                        let mut param_value_vec: Vec<String> = Vec::new();
                                        for method_param_value_rule in
                                            method_param_declaration_inner
                                        {
                                            let method_param_value =
                                                String::from(method_param_value_rule.as_str());

                                            param_value_vec.push(method_param_value.clone());
                                        }
                                        println!(
                                            "method_param_value in list of str: {:?}",
                                            param_value_vec
                                        );
                                        string_vec_params_map
                                            .insert(method_param_name, param_value_vec);
                                    } else if method_param_type == "list_of_number" {
                                        let mut param_value_vec: Vec<u128> = Vec::new();
                                        for method_param_value_rule in
                                            method_param_declaration_inner
                                        {
                                            let method_param_value = method_param_value_rule
                                                .as_str()
                                                .parse::<u128>()
                                                .unwrap();
                                            param_value_vec.push(method_param_value);
                                        }
                                        println!(
                                            "method_param_value in list of u128: {:?}",
                                            param_value_vec
                                        );
                                        num_vec_params_map
                                            .insert(method_param_name, param_value_vec);
                                    } else if method_param_type == "string" {
                                        let method_param_value = String::from(
                                            method_param_declaration_inner.next().unwrap().as_str(),
                                        );
                                        println!(
                                            "method_param_value in str: {}",
                                            method_param_value
                                        );
                                        string_params_map
                                            .insert(method_param_name, method_param_value.clone());
                                    } else if method_param_type == "number" {
                                        let method_param_value =
                                            method_param_declaration_inner.next().unwrap().as_str();
                                        println!(
                                            "method_param_value in u128: {}",
                                            method_param_value
                                        );
                                        num_params_map.insert(
                                            method_param_name,
                                            method_param_value.parse::<u128>().unwrap(),
                                        );
                                    }
                                }
                                method_params = MethodParams {
                                    string_params_map: string_params_map,
                                    num_params_map: num_params_map,
                                    string_vec_params_map: string_vec_params_map,
                                    num_vec_params_map: num_vec_params_map,
                                }
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
                method_params: method_params,
            },
        );
    }

    // println!("task_dependencies_map: {:?}", task_dependencies_map);
    // println!("task_properties_map: {:?}", task_properties_map);

    return (
        Box::new(task_properties_map),
        Box::new(task_dependencies_map),
    );
}
