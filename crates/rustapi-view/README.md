# RustAPI View

**Server-side rendering for RustAPI using Tera.**

Create dynamic HTML web applications with the powerful Jinja2-like syntax of Tera.

## Features

- **Type-Safe Context**: Pass Rust structs directly to templates.
- **Auto-Reload**: Templates reload automatically in debug mode—no restart required.
- **Includes & Inheritance**: Master pages, blocks, and macros supported.

## Example

**`src/main.rs`**
```rust
use rustapi_view::{View, Context};

#[get("/")]
async fn index() -> View {
    let mut ctx = Context::new();
    ctx.insert("title", "My Blog");
    ctx.insert("posts", &vec!["Post 1", "Post 2"]);
    
    View::new("index.html", ctx)
}
```

**`templates/index.html`**
```html
<h1>{{ title }}</h1>
<ul>
{% for post in posts %}
    <li>{{ post }}</li>
{% endfor %}
</ul>
```
