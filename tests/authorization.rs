use reqwest::blocking::Client;

#[test]
fn test_login() {
   let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/login").body({}).send().unwrap();
    assert_eq!(response.status(), 200);
}
