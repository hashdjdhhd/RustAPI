//! Swagger UI HTML generation

/// Generate Swagger UI HTML page
pub fn generate_swagger_html(openapi_url: &str) -> String {
    let mut html = String::with_capacity(2000000); // Pre-allocate ~2MB for assets
    html.push_str(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>API Documentation - RustAPI</title>
    <style>
"#,
    );
    html.push_str(include_str!("assets/swagger-ui.css"));
    html.push_str(
        r#"
        body {
            margin: 0;
            padding: 0;
        }
        .swagger-ui .topbar {
            display: none;
        }
        .swagger-ui .info .title {
            font-size: 2.5rem;
        }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script>
"#,
    );
    html.push_str(include_str!("assets/swagger-ui-bundle.js"));
    html.push_str(
        r#"
    </script>
    <script>
"#,
    );
    html.push_str(include_str!("assets/swagger-ui-standalone-preset.js"));
    html.push_str(
        r#"
    </script>
    <script>
        window.onload = function() {
            SwaggerUIBundle({
                url: ""#,
    );
    html.push_str(openapi_url);
    html.push_str(
        r#"",
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                layout: "StandaloneLayout"
            });
        };
    </script>
</body>
</html>"#,
    );
    html
}
