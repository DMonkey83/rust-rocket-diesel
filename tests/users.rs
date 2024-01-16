use reqwest::blocking::Client;

#[test]
fn test_get_user() {
   let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/users/hello").send().unwrap();
    assert_eq!(response.status(), 200);
}
