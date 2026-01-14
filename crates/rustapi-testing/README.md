# RustAPI Testing

Testing utilities and test harness for the RustAPI framework.

## Features

- **TestClient**: A wrapper around `reqwest` and `hyper` for testing your API endpoints integration.
- **Fluid Assertions**: Custom matchers for status codes, headers, and body content.
- **Mocking**: Helpers for mocking dependencies (if applicable).
- **Proptest Integration**: Strategies for property-based testing of handlers.

## Usage

Add to `dev-dependencies`:

```toml
[dev-dependencies]
rustapi-testing = "0.1"
```

### Example

```rust
use rustapi_testing::TestClient;
use rustapi::status::StatusCode;

#[tokio::test]
async fn test_create_user() {
    let client = TestClient::new(app());
    
    let res = client.post("/users")
        .json(&json!({ "name": "Alice" }))
        .send()
        .await;
        
    res.assert_status(StatusCode::OK);
    res.assert_json_snapshot!("create_user_response");
}
```
