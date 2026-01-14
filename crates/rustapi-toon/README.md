# RustAPI TOON

**Token-Oriented Object Notation (TOON) support.**

## 🤖 What is TOON?

TOON is an experimental, density-optimized data format designed for **High-Volume LLM Interactions**.

When building agents or APIs that consume massive amounts of structured data via LLMs (GPT-4, Claude), standard JSON is token-expensive due to repeated keys and syntax overhead. TOON eliminates this redundancy.

## Comparison

**JSON (Expensive)**
```json
[
  {"id": 1, "role": "admin", "active": true},
  {"id": 2, "role": "user",  "active": true},
  {"id": 3, "role": "user",  "active": false}
]
```

**TOON (Optimized)**
```
users[3]{id,role,active}:
  1,admin,true
  2,user,true
  3,user,false
```

## Usage

RustAPI handles this transparently via content negotiation.

```rust
use rustapi_toon::Toon;

// Accepts explicit TOON or JSON automatically based on Content-Type
#[post("/ingest")]
async fn ingest(Toon(data): Toon<Vec<User>>) -> impl Responder {
    // ...
}
```
