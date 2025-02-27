use serde_json::Value;


pub fn to_array(json: &Value) -> Result<Vec<Value>, String> {
    match json.as_array() {
        Some(results) => Ok(results.to_vec()),
        None => Err(format!("JSON value is not an array: {:#?}", json))
    }
}