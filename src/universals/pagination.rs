use serde_json::Value;
use super::{client::HubSpotClient, requests::HttpMethod, utils::to_array};

pub enum TurnPageMethod {
    After(usize),
    NextUrl(String),
}

pub struct CreateBody {
    pub static_body: Value,
    pub create_body: fn(Value, PaginationBodyParams) -> Value,
}

pub struct PaginationBodyParams {
    pub after: Option<usize>,
    pub limit: Option<usize>,
}

impl HubSpotClient {
    pub async fn request_with_pagination(
        &self,
        mut path: String,
        method: HttpMethod,
        body: Option<CreateBody>,
        max_amount: Option<usize>,
        get_next_page_method: fn(&Value) -> Option<TurnPageMethod>,
    ) -> Result<Vec<Value>, String> {
        let mut after = 0;
        let mut all_objects = Vec::new();
        let request_limit = match max_amount {
            Some(limit) if limit < 200 => limit,
            _ => 200
        };

        loop {
            let result = self.request(
                &path,
                &method,
                match &body {
                    Some(body) => Some((body.create_body)(body.static_body.clone(), PaginationBodyParams {
                        after: Some(after),
                        limit: Some(request_limit),
                    })),
                    None => None,
                }
            ).await?;

            all_objects.extend(to_array(&result["results"])?);

            match max_amount {
                Some(max_amount) if all_objects.len() >= max_amount => break,
                _ => (),
            };

            match get_next_page_method(&result) {
                Some(TurnPageMethod::After(new_after)) => after = new_after,
                Some(TurnPageMethod::NextUrl(next_url)) => path = next_url, // URL is handled same as path
                None => break,
            };

            tokio::time::sleep(std::time::Duration::from_millis(200)).await; // 5 requests per second max
        }
        return Ok(all_objects);
    }
}