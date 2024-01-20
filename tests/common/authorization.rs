use reqwest::blocking::{Client, ClientBuilder};
use reqwest::StatusCode;
use reqwest::header;
use serde_json::{json, Value};


#[test]
fn test_login() {
   let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/api/login").json(&json!({"username": "newTest1", "password": "password"})).send().unwrap();
    assert_eq!(response.status(), 200);
}


pub fn get_logged_in_client(username: &str, role: &str) -> Client {
    let client = Client::new();
    let _ = client.post("http://127.0.0.1:8000/api/users").json(&json!({"new_user": {"username": username, "password_hash": "password"}, "role_codes": [role]})).send().unwrap();
    let response = client.post("http://127.0.0.1:8000/api/login")
        .json(&json!({
            "username": username,
            "password": "password",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("session_id").is_some());
    let header_value = format!("Bearer {}", json["session_id"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap()
    );

    ClientBuilder::new().default_headers(headers).build().unwrap()
}


pub fn get_client_with_logged_in_viewer() -> Client {
    get_logged_in_client("test_viewer", "User")
}

pub fn get_client_with_logged_in_admin() -> Client {
    get_logged_in_client("test_admin", "Adming")
}

