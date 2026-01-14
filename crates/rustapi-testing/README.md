# RustAPI Testing

**A fluid, ergonomic test harness for RustAPI applications.**

Don't just test your logic; test your endpoints.

## Features

- **`TestClient`**: Spawns your application directly (no port binding needed) and sends requests.
- **Fluid Assertions**: `res.assert_status(200).assert_json(&expected)`.
- **Mocking**: Utilities to swap out state/databases during tests.

## Example

```rust
#[cfg(test)]
mod tests {
    use rustapi_testing::TestClient;
    use rustapi_rs::prelude::*;

    #[tokio::test]
    async fn test_create_user() {
        // 1. Setup app
        let app = RustApi::new().mount_route(create_user_route());
        let client = TestClient::new(app);

        // 2. Execute
        let response = client.post("/users")
            .json(&json!({ "name": "Alice" }))
            .send()
            .await;

        // 3. Assert
        response
            .assert_status(StatusCode::OK)
            .assert_json_path("$.id", 1);
    }
}
```
