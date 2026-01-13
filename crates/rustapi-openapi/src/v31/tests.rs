//! Property tests for OpenAPI 3.1 compliance
//!
//! These tests verify that the generated OpenAPI 3.1 specifications
//! comply with the OpenAPI 3.1.0 standard.

#[cfg(test)]
mod tests {
    use crate::v31::*;

    /// Property 18: OpenAPI 3.1 Compliance
    ///
    /// Validates: Requirements 12.1, 12.2
    /// - Generated spec has "openapi": "3.1.0"
    /// - JSON Schema 2020-12 dialect is set
    /// - Webhook definitions are properly serialized
    /// - Schema transformations work correctly

    #[test]
    fn test_openapi_31_version() {
        let spec = OpenApi31Spec::new("Test API", "1.0.0");
        assert_eq!(spec.openapi, "3.1.0");
    }

    #[test]
    fn test_json_schema_dialect() {
        let spec = OpenApi31Spec::new("Test API", "1.0.0");
        assert_eq!(
            spec.json_schema_dialect,
            Some("https://json-schema.org/draft/2020-12/schema".to_string())
        );
    }

    #[test]
    fn test_webhook_serialization() {
        let spec = OpenApi31Spec::new("Test API", "1.0.0").webhook(
            "orderPlaced",
            Webhook::with_summary("Order notification")
                .description("Triggered when an order is placed")
                .post(
                    WebhookOperation::new()
                        .operation_id("notifyOrderPlaced")
                        .summary("Notify about order placement")
                        .request_body(WebhookRequestBody::json(
                            JsonSchema2020::object()
                                .with_property("orderId", JsonSchema2020::string())
                                .with_property("amount", JsonSchema2020::number())
                                .with_required("orderId"),
                        ))
                        .response("200", WebhookResponse::new("Webhook processed")),
                ),
        );

        let json = spec.to_json();

        // Verify webhooks are present
        assert!(json.get("webhooks").is_some());
        assert!(json["webhooks"].get("orderPlaced").is_some());
        assert_eq!(
            json["webhooks"]["orderPlaced"]["summary"],
            "Order notification"
        );
    }

    #[test]
    fn test_schema_transformation_nullable() {
        // Test OpenAPI 3.0 -> 3.1 transformation for nullable types
        let schema_30 = serde_json::json!({
            "type": "string",
            "nullable": true
        });

        let schema_31 = SchemaTransformer::transform_30_to_31(schema_30);

        // In 3.1, nullable should become type array
        assert_eq!(schema_31["type"], serde_json::json!(["string", "null"]));
        assert!(schema_31.get("nullable").is_none());
    }

    #[test]
    fn test_schema_transformation_exclusive_bounds() {
        // Test exclusive minimum/maximum transformation
        let schema_30 = serde_json::json!({
            "type": "integer",
            "minimum": 0,
            "exclusiveMinimum": true,
            "maximum": 100,
            "exclusiveMaximum": true
        });

        let schema_31 = SchemaTransformer::transform_30_to_31(schema_30);

        // In 3.1, exclusiveMinimum/Maximum are numbers, not booleans
        assert_eq!(schema_31["exclusiveMinimum"], 0);
        assert_eq!(schema_31["exclusiveMaximum"], 100);
        assert!(schema_31.get("minimum").is_none());
        assert!(schema_31.get("maximum").is_none());
    }

    #[test]
    fn test_type_array_nullable() {
        let ty = TypeArray::nullable("string");
        assert!(ty.is_nullable());

        let json = serde_json::to_value(&ty).unwrap();
        assert_eq!(json, serde_json::json!(["string", "null"]));
    }

    #[test]
    fn test_type_array_single() {
        let ty = TypeArray::single("integer");
        assert!(!ty.is_nullable());

        let json = serde_json::to_value(&ty).unwrap();
        assert_eq!(json, serde_json::json!("integer"));
    }

    #[test]
    fn test_make_nullable_idempotent() {
        let ty = TypeArray::single("string").make_nullable().make_nullable();

        // Should only have one "null" entry
        if let TypeArray::Array(types) = ty {
            assert_eq!(types.iter().filter(|t| *t == "null").count(), 1);
        } else {
            panic!("Expected TypeArray::Array");
        }
    }

    #[test]
    fn test_schema_builder_object() {
        let schema = JsonSchema2020::object()
            .with_title("User")
            .with_description("A user object")
            .with_property("id", JsonSchema2020::integer())
            .with_property("name", JsonSchema2020::string())
            .with_property("email", JsonSchema2020::string().with_format("email"))
            .with_required("id")
            .with_required("name");

        assert!(schema.properties.is_some());
        let props = schema.properties.as_ref().unwrap();
        assert!(props.contains_key("id"));
        assert!(props.contains_key("name"));
        assert!(props.contains_key("email"));

        assert_eq!(
            schema.required,
            Some(vec!["id".to_string(), "name".to_string()])
        );
    }

    #[test]
    fn test_spec_with_license_spdx() {
        // Test SPDX license identifier (new in 3.1)
        let spec =
            OpenApi31Spec::new("Test API", "1.0.0").license(License::spdx("MIT License", "MIT"));

        assert!(spec.info.license.is_some());
        let license = spec.info.license.as_ref().unwrap();
        assert_eq!(license.name, "MIT License");
        assert_eq!(license.identifier, Some("MIT".to_string()));
    }

    #[test]
    fn test_spec_with_summary() {
        // Test info.summary (new in 3.1)
        let spec = OpenApi31Spec::new("Test API", "1.0.0").summary("A brief summary of the API");

        assert_eq!(
            spec.info.summary,
            Some("A brief summary of the API".to_string())
        );
    }

    #[test]
    fn test_spec_with_security_scheme() {
        let spec = OpenApi31Spec::new("Test API", "1.0.0")
            .security_scheme("bearerAuth", SecurityScheme::bearer("JWT"))
            .security_scheme("apiKey", SecurityScheme::api_key("X-API-Key", "header"))
            .security_requirement("bearerAuth", vec![]);

        assert!(spec.components.is_some());
        let components = spec.components.as_ref().unwrap();
        assert!(components.security_schemes.is_some());

        let schemes = components.security_schemes.as_ref().unwrap();
        assert!(schemes.contains_key("bearerAuth"));
        assert!(schemes.contains_key("apiKey"));
    }

    #[test]
    fn test_spec_with_servers() {
        let spec = OpenApi31Spec::new("Test API", "1.0.0").server(
            Server::new("https://{environment}.api.example.com")
                .description("Main API server")
                .variable(
                    "environment",
                    ServerVariable::new("production")
                        .enum_values(vec![
                            "development".to_string(),
                            "staging".to_string(),
                            "production".to_string(),
                        ])
                        .description("Server environment"),
                ),
        );

        assert!(spec.servers.is_some());
        let servers = spec.servers.as_ref().unwrap();
        assert_eq!(servers.len(), 1);
        assert!(servers[0].variables.is_some());
    }

    #[test]
    fn test_spec_to_json_serialization() {
        let spec = OpenApi31Spec::new("Test API", "1.0.0")
            .description("Test API description")
            .server(Server::new("https://api.example.com"))
            .schema(
                "User",
                JsonSchema2020::object()
                    .with_property("id", JsonSchema2020::integer())
                    .with_property("name", JsonSchema2020::string()),
            );

        let json = spec.to_json();

        assert_eq!(json["openapi"], "3.1.0");
        assert_eq!(json["info"]["title"], "Test API");
        assert_eq!(json["info"]["version"], "1.0.0");
        assert!(json.get("servers").is_some());
        assert!(json.get("components").is_some());
    }

    #[test]
    fn test_callback_definition() {
        let callback = Callback::new().expression(
            "{$request.body#/callbackUrl}",
            Webhook::new().post(
                WebhookOperation::new()
                    .summary("Callback notification")
                    .response("200", WebhookResponse::new("OK")),
            ),
        );

        assert!(callback
            .expressions
            .contains_key("{$request.body#/callbackUrl}"));
    }

    #[test]
    fn test_nested_schema_transformation() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "nullable": true
                        }
                    }
                },
                "items": {
                    "type": "array",
                    "items": {
                        "type": "integer",
                        "nullable": true
                    }
                }
            }
        });

        let transformed = SchemaTransformer::transform_30_to_31(schema);

        // Check nested properties transformed
        assert_eq!(
            transformed["properties"]["user"]["properties"]["name"]["type"],
            serde_json::json!(["string", "null"])
        );
        assert_eq!(
            transformed["properties"]["items"]["items"]["type"],
            serde_json::json!(["integer", "null"])
        );
    }
}
