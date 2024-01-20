use reqwest::blocking::Client;
pub mod common;

#[test]
fn test_get_user_unauthorized() {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/api/users/newTest1")
        .send()
        .unwrap();
    assert_eq!(response.status(), 401);
}

#[test]
fn test_get_user_authorized() {
    let client = common::authorization::get_client_with_logged_in_viewer();
    let response = client
        .get("http://127.0.0.1:8000/api/users/newTest1")
        .send()
        .unwrap();
    assert_eq!(response.status(), 200);
}
