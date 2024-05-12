use crate::parser::config_parser;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct Task<'a> {
    name: String,
    task_params: &'a config_parser::config_structs::TaskParams,
}

impl<'a> Task<'a> {
    pub fn new(name: String, task_params: &config_parser::config_structs::TaskParams) -> Task {
        Task { name, task_params }
    }

    pub fn execute(&self) {
        let result: Result<(), ()> = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let api_response_string = self
                .make_http_request()
                .await
                .expect(format!("Failed to make HTTP request for task: {}", self.name).as_str());

            self.store_response(api_response_string)
                .await
                .expect(format!("Failed to store api response for task: {}", self.name).as_str());
            Ok(())
        });
    }

    async fn make_http_request(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        // headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        if self.task_params.chain_name == "eth-mainnet" {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_static("Bearer eth-mainnet-api-token"),
            );
        } else if self.task_params.chain_name == "solana-mainnet" {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_static("Bearer solana-mainnet-api-token"),
            );
        }

        // Create a reqwest Client
        let client = Client::new();

        let url = format!("https://{}.g.alchemy.com/v2/", self.task_params.chain_name);
        let resp = client
            .post(url)
            .headers(headers)
            .body(self.task_params.method_params_json_string.clone())
            .send()
            .await?;

        let resp_string = resp.text().await.unwrap();
        let deserialized_map: HashMap<String, Value> = from_str(resp_string.clone().as_str())?;

        println!(
            "Response for task {} is {:?}. deserialized_map: {:?}",
            self.name, resp_string, deserialized_map
        );

        Ok(resp_string)
    }

    async fn store_response(&self, text: String) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("./outputs/response_{}.json", self.name);
        // Store the response locally
        let mut file = File::create(path.clone())?;

        file.write_all(text.as_bytes())?;
        println!("Response stored locally into {}", path);

        Ok(())
    }
}
