use reqwest::Client;
use serde_json::Value;
use super::client::HubSpotClient;

const BASE_URL: &str = "https://api.hubspot.com";

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HubSpotClient {
    pub async fn request(
        &self,
        path: &str,
        method: &HttpMethod,
        payload: Option<Value>,
    ) -> Result<Value, String> {
        let path = path.trim_start_matches(BASE_URL); // sometimes we can receive a "next url" rather than a path if we are using HubSpot's pagination
        let mut reqwest_builder = match method {
            HttpMethod::Get => Client::new().get(BASE_URL.to_string() + path),
            HttpMethod::Post => Client::new().post(BASE_URL.to_string() + path),
            HttpMethod::Put => Client::new().put(BASE_URL.to_string() + path),
            HttpMethod::Patch => Client::new().patch(BASE_URL.to_string() + path),
            HttpMethod::Delete => Client::new().delete(BASE_URL.to_string() + path),
        };

        reqwest_builder = reqwest_builder.bearer_auth(&self.api_key);

        let response = reqwest_builder
            .json(&payload)
            .send()
            .await
            .map_err(|err| format!("Error sending request: {:#?}", err))?;

        let status = response.status();
        if !status.is_success() {
            let response_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read response body".to_string());

            return Err(format!(
                "Error: received status code {}\npath: {}\npayload: {:#?}\nresponse body: {}",
                status, path, payload, response_text
            ))
        }

        let result_str = response
            .text()
            .await
            .map_err(|err| format!("Error reading response body: {:#?}", err))?;

        if result_str.is_empty() {
            return Ok(Value::Null);
        }

        return match result_str.parse::<Value>() {
            Ok(value) => Ok(value),
            Err(err) => Err(format!("Error converting response to JSON: {:#?}", err)),
        }
    }
}