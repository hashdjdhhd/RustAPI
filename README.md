# RustAPI — PRD + Manifesto (v0.1 Draft)

> **Amaç:** Rust ile API geliştirmeyi *FastAPI kadar hızlı ve “kolay hissettiren”*, ama Rust’ın performans + bellek güvenliği avantajlarını koruyan bir framework tasarlamak.

Bu doküman, RustAPI’nin ürün vizyonunu, gereksinimlerini, teknik mimarisini, DX (Developer Experience) hedeflerini, roadmap’ini ve örnek kullanım desenlerini **çok detaylı** şekilde anlatır. Repo’nun `README.md` dosyası veya ekibe sunacağın bir “Manifesto” olarak kullanılabilir.

---

## İçindekiler

1. [Yönetici Özeti](#1-yönetici-özeti)
2. [Vizyon & Misyon](#2-vizyon--misyon)
3. [Problem Tanımı](#3-problem-tanımı)
4. [Hedef Kitle & Kullanım Senaryoları](#4-hedef-kitle--kullanım-senaryoları)
5. [Ürün İlkeleri](#5-ürün-ilkeleri)
6. [Kapsam & Kapsam Dışı](#6-kapsam--kapsam-dışı)
7. [Developer Experience Tasarımı](#7-developer-experience-tasarımı)
8. [Fonksiyonel Gereksinimler](#8-fonksiyonel-gereksinimler)
9. [Non-Functional Gereksinimler](#9-non-functional-gereksinimler)
10. [Mimari Strateji](#10-mimari-strateji)
11. [Crate Yapısı & Modül Tasarımı](#11-crate-yapısı--modül-tasarımı)
12. [Routing & Handler Modeli](#12-routing--handler-modeli)
13. [Request Extractors (DI / Magic)](#13-request-extractors-di--magic)
14. [Validasyon Sistemi (Pydantic-Style)](#14-validasyon-sistemi-pydantic-style)
15. [OpenAPI + Swagger UI Otomasyonu](#15-openapi--swagger-ui-otomasyonu)
16. [Response Modeli & Hata Tasarımı](#16-response-modeli--hata-tasarımı)
17. [Middleware & Ekosistem](#17-middleware--ekosistem)
18. [Konfigürasyon & Ortam Yönetimi](#18-konfigürasyon--ortam-yönetimi)
19. [Güvenlik](#19-güvenlik)
20. [Gözlemlenebilirlik (Logs/Tracing/Metrics)](#20-gözlemlenebilirlik-logstracingmetrics)
21. [Test Stratejisi](#21-test-stratejisi)
22. [Performans Stratejisi](#22-performans-stratejisi)
23. [Örnek Proje: Mini CRUD](#23-örnek-proje-mini-crud)
24. [Roadmap & Milestones](#24-roadmap--milestones)
25. [Başarı Kriterleri](#25-başarı-kriterleri)
26. [Riskler & Mitigasyon](#26-riskler--mitigasyon)
27. [Katkı Rehberi (Contributing)](#27-katkı-rehberi-contributing)
28. [Lisans & İsimlendirme](#28-lisans--i̇simlendirme)

---

## 1. Yönetici Özeti

**RustAPI**, Rust’ın performansını ve güvenliğini, FastAPI’nin “sadece iş mantığı yaz” yaklaşımıyla birleştiren, yüksek seviyeli bir web framework’tür.

Mevcut Rust framework’leri güçlüdür; ancak yeni başlayanlar için “ilk CRUD’u ayağa kaldırma” süreci **yüksek bilişsel yük** gerektirir:
- import / trait / extractor karmaşıklığı,
- validasyonun parçalı olması,
- OpenAPI’nin manuel veya ek iş olarak kalması,
- hata çıktılarının yönlendirici olmaması.

RustAPI’nin vaadi:
- **5 satırın altında Hello World**
- **Derleme zamanında doğrulama + şema üretimi**
- **“Batteries Included” ama modüler**
- **İleri düzey kullanımda kaçış kapısı (escape hatch)**
- **DX-first**: Rust’ı web tarafında “kolay hissettirmek”

---

## 2. Vizyon & Misyon

### Vizyon
Rust ile backend geliştirmeyi, Python/Node geliştiricileri için olduğu kadar erişilebilir, hızlı ve keyifli hale getirmek. “Rust zordur” algısını, web geliştirme dikeyinde kırmak.

### Misyon
1. **Boilerplate’i yok etmek:**  
   Endpoint yazmak = 1 fonksiyon + 1 attribute macro.
2. **Dokümantasyonu otomatize etmek:**  
   OpenAPI + Swagger UI, kodla aynı anda güncel.
3. **Güvenliği standartlaştırmak:**  
   Validasyon, tip güvenliği ve standart hata cevapları varsayılan.

---

## 3. Problem Tanımı

Rust web ekosisteminde tipik problem alanları:

### 3.1 Yüksek Bilişsel Yük
- Basit JSON dönmek için bile “framework’e özgü” response tipleri,
- `State`, `Extension`, `Data`, `Json<T>` gibi wrapper’ların çoğalması,
- “Neyi nerede kayıt etmem gerekiyor?” sorusu (router/app state bağlama),
- Birçok yerde `trait bound` patlamaları.

### 3.2 Validasyonun Parçalı Olması
- Serde ile deserialize kolay; fakat doğrulama *çatıdan* gelmiyor.
- Her takım farklı crate kombinasyonu kullanıyor; ortak hata formatı yok.

### 3.3 Dokümantasyon Kopukluğu
- Kod değişir, OpenAPI geride kalır.
- Şema üretimi kütüphanesi seçimi + entegrasyonu geliştiriciye kalır.
- Swagger UI çoğu zaman “ek paket” ve “ek wiring”.

### 3.4 Hata Mesajları (DX) Kötü Deneyim
- Compile error’lar Rust’ın doğası gereği uzun; framework katmanı bunu kötüleştirebiliyor.
- Validasyon hatası JSON formatı standart değil.
- “Ne yapmalıyım?” yönlendirmesi zayıf.

---

## 4. Hedef Kitle & Kullanım Senaryoları

### 4.1 Persona: “FastAPI’den Rust’a Geçen”
- Python/FastAPI biliyor, Rust öğreniyor.
- İstediği: hızlı endpoint, otomatik docs, kolay validasyon.

### 4.2 Persona: “Performans İçin Rust”
- Node/Python servisleri yavaşladı → Rust’a geçiyor.
- İstediği: throughput + düşük latency; ama DX de iyi olsun.

### 4.3 Persona: “Rust Ustası”
- Axum/Actix kullanabiliyor.
- İstediği: daha az boilerplate; ama gerektiğinde low-level kontrol.

### 4.4 Kullanım Senaryoları (Use Cases)
- JSON CRUD API (REST)
- Auth + JWT
- Admin panel backend
- Event ingestion endpoint’leri
- Servis-to-servis internal API
- Prototip (hızlı MVP)
- Kurumsal standart: ortak hata formatı + dokümantasyon

---

## 5. Ürün İlkeleri

1. **DX First**
   - Rust zaten hızlı.
   - %1 performans kaybı, DX iyileşmesi için kabul edilebilir.

2. **Convention over Configuration**
   - Geliştirici belirtmezse:
     - JSON request/response
     - 400 / 422 validasyon hata formatı
     - 500 internal error
     - `/docs` hazır

3. **Type-Driven Development**
   - Şema = Rust struct
   - “Single Source of Truth”

4. **Güvenli Varsayılanlar**
   - Validasyon default açık
   - Güvenlik header’ları (opsiyonel)
   - Error leak önleme (prod modda stack trace kapalı)

5. **Escape Hatch**
   - İleri kullanıcılar için:
     - raw request/response erişimi
     - Tower middleware katmanına erişim
     - Hyper seviyesine yakın kontrol

---

## 6. Kapsam & Kapsam Dışı

### 6.1 Kapsam (v0.x)
- GET/POST/PUT/PATCH/DELETE routing (macro ile)
- JSON body parsing (serde)
- Query/path param extraction
- State/DI injection
- Declarative validation
- OpenAPI schema generation
- Swagger UI `/docs`
- Standard error response model
- Tower middleware uyumu
- Tokio runtime

### 6.2 Kapsam Dışı (ilk etap)
- GraphQL (sonra olabilir)
- WebSocket (sonra)
- gRPC (sonra)
- Template engine (ama ekosistem ile)
- Full ORM (SQLx integration örnekleri yeterli)
- “Magic” runtime reflection (Rust’ta yok; compile-time macro + trait ile)

---

## 7. Developer Experience Tasarımı

RustAPI’nin DX hedefi: **“Okunabilirlik + yazım hızı + hata mesajı kalitesi”**.

### 7.1 İlk Deneyim (Onboarding)
- `cargo add rustapi` (veya workspace template)
- tek dosyada `main.rs`
- `#[rustapi::main]` ile server ayağa kalkar

### 7.2 “Bir Endpoint Yazmak”
Hedef kullanım:

```rust
use rustapi::prelude::*;

#[derive(Schema, Validate, Deserialize)]
struct RegisterRequest {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Schema, Serialize)]
struct UserOut {
    id: i64,
    email: String,
}

#[rustapi::post("/register")]
async fn register(state: AppState, body: RegisterRequest) -> Result<UserOut> {
    // body burada VALIDATED gelir
    let user = state.users.create(body).await?;
    Ok(UserOut { id: user.id, email: user.email })
}

#[rustapi::main]
async fn main() -> Result<()> {
    RustApi::new()
        .state(AppState::new())
        .mount(register)
        .run("0.0.0.0:8080")
        .await
}
```

### 7.3 DX “Altın Kurallar”
- Handler imzası “dokümantasyon gibi okunmalı”
- `Result<T>` döndüğünde JSON dönmeli
- Hata olursa standard JSON dönmeli
- Compile error mesajları “RustAPI hint” içermeli (mümkün olduğunca)

---

## 8. Fonksiyonel Gereksinimler

### 8.1 Otomatik OpenAPI (Swagger UI)
**Gereksinim:** Handler imzalarından endpoint şeması çıkar.  
**Sunum:** `/docs` altında Swagger UI.  
**Derleme Zamanı:** Request/response/error modelleri derleme zamanında şemaya dönüştürülür.

**OpenAPI’de kapsanacaklar:**
- Path parametreleri
- Query parametreleri
- JSON request body
- Response body (200/201)
- Error models (400/401/403/404/409/422/500 vb.)
- Tags, summary, description (macro parametresiyle opsiyonel)

### 8.2 “Magic” Dependency Injection (Extractors)
Handler parametreleri tipine göre otomatik çözülür:
- `AppState` / `State<T>`
- `DbPool` (extension olarak)
- `AuthUser` (middleware’den)
- `Headers`, `Cookies`, `IpAddr` gibi request context verileri

Örnek:
```rust
#[rustapi::put("/me")]
async fn update_me(db: DbPool, user: AuthUser, body: ProfileUpdate) -> Result<ProfileOut> {
    Ok(db.users.update(user.id, body).await?)
}
```

### 8.3 Deklaratif Validasyon (Pydantic-Style)
**Hedef:** Validasyon kuralı struct alanında tanımlı.  
**Framework:** Kurallara uymayan request → otomatik **400/422 JSON error**.

Validasyon örnekleri:
- `email`
- `url`
- `length(min, max)`
- `range(min, max)`
- `regex("...")`
- `non_empty`
- nested validation (sub-struct)
- list item validation (Vec<T>)

### 8.4 Makro Tabanlı Routing
- `#[rustapi::get("/items/{id}")]`
- `#[rustapi::post("/items")]`
- `#[rustapi::delete("/items/{id}")]`
- `#[rustapi::route(method="GET", path="/...")]` (genel)

### 8.5 Basitleştirilmiş Return Tipleri
- `Result<T>` → 200 JSON
- `Result<Created<T>>` → 201 JSON
- `Result<NoContent>` → 204
- `Result<Html<String>>` → text/html
- `Result<Text<String>>` → text/plain
- `Result<Bytes>` → application/octet-stream

---

## 9. Non-Functional Gereksinimler

### 9.1 Performans
- Axum/Actix seviyesine yakın throughput hedefi (DX uğruna minimal overhead)
- Serde zero-copy pattern’leri (mümkün olduğunca)
- Router: O(segments) + prefix tree / radix tree

### 9.2 Güvenilirlik
- Panic sınırları kontrol altında
- Graceful shutdown
- Backpressure (tower)
- Request size limit (default)

### 9.3 Güvenlik
- Default olarak:
  - structured error (stack trace yok)
  - input validation
  - body size limit
- CORS/JWT gibi middleware’ler “batteries included” fazında.

### 9.4 Gözlemlenebilirlik
- `tracing` entegre
- request id
- structured logs

### 9.5 Uyum
- Tokio + Hyper + Tower ekosistemiyle uyumlu
- SQLx, Redis, reqwest gibi kütüphanelerle sorunsuz entegre

---

## 10. Mimari Strateji

Tekerleği yeniden icat etmeden:

- **HTTP Engine:** `hyper`
- **Runtime:** `tokio`
- **Middleware:** `tower` (+ tower-http)
- **Serialization:** `serde`
- **OpenAPI:** `utoipa` (veya benzeri; wrapper)
- **Validation:** `validator` crate veya custom derive (uzun vadede kendi)
- **RustAPI Katmanı:** procedural macros + trait tabanlı extract/response sistemi

Mimari katmanlar:

```
[User Code]
  |  (attribute macros)
[RustAPI Macros]  --->  compile-time route + schema registry
  |
[RustAPI Core]  --->  router + extractor + response + error + state
  |
[Tower Service] --->  middleware stack
  |
[Hyper Server]
  |
[Tokio Runtime]
```

---

## 11. Crate Yapısı & Modül Tasarımı

Önerilen workspace:

```
rustapi/
  crates/
    rustapi/           # public crate: prelude, RustApi builder, types
    rustapi-core/      # router, extractor traits, response traits, runtime glue
    rustapi-macros/    # procedural macros: get/post/main/Schema/Validate helpers
    rustapi-openapi/   # openapi registry + swagger ui serving
    rustapi-validate/  # validation engine + error formatting
    rustapi-extras/    # optional batteries (jwt, cors, rate limit)
  examples/
    hello-world/
    todo-crud/
  README.md
  Cargo.toml
```

### 11.1 `rustapi` (public facade)
- `prelude::*` ile en sık kullanılan şeyleri tek yerden ver
- `RustApi` builder:
  - `.state(T)`
  - `.mount(handler_fn)`
  - `.nest("/v1", router)`
  - `.layer(middleware)`
  - `.run(addr)`

### 11.2 `rustapi-core`
- Router
- Request context
- Extractor trait’leri
- Response conversion (`IntoResponse`)
- Error conversion (`IntoApiError`)
- Extension/State store

### 11.3 `rustapi-macros`
- `#[rustapi::get]`, `#[rustapi::post]`, `#[rustapi::main]`
- `#[derive(Schema)]` (utoipa wrapper) veya re-export
- `#[derive(Validate)]` (validator wrapper) veya custom

### 11.4 `rustapi-openapi`
- Schema registry (compile-time metadata + runtime serve)
- Swagger UI assets
- `/openapi.json` endpoint

### 11.5 `rustapi-validate`
- Validation rule parser + runtime validator runner
- Unified validation error format

---

## 12. Routing & Handler Modeli

### 12.1 Handler İmzası = Kontrat
Handler parametreleri *extractor*’lar ile eşlenir.

Örnek imza kategorileri:
- `id: Path<i64>`
- `q: Query<SearchQuery>`
- `body: Body<T>` (ama wrapper’ı gizleyebiliriz)
- `state: AppState`
- `user: AuthUser`

RustAPI hedefi: wrapper kullanımını minimize etmek.  
Yani `Path<i64>` yerine direkt `id: i64` yazılabilsin (macro bunu bilir).

### 12.2 Route Kaydı
`#[rustapi::get("/items/{id}")]` makrosu, derleme zamanında:
- Path pattern’i parse eder
- Handler parametrelerini inceler
- Router’a “metadata” üretir
- OpenAPI registry’ye route bilgisini ekler

### 12.3 Router Yapısı
- Radix tree / trie
- segment bazlı eşleşme
- param segment: `{id}`

Route conflict detection (compile-time veya startup-time):
- Aynı method + path iki kez mount edilirse hata

---

## 13. Request Extractors (DI / Magic)

### 13.1 Extractor Kuralları (Öneri)
Parametre türlerine göre öncelik:

1. **State**: `AppState` gibi registered state type
2. **Auth**: `AuthUser` gibi request extensions
3. **Path params**: primitive ve `FromStr`
4. **Query params**: struct + serde/qs
5. **Body**: JSON deserialize
6. **Fallback**: headers/cookies/ip vb.

### 13.2 Extractor Trait Taslağı

```rust
pub trait FromRequest: Sized {
    type Rejection: IntoApiError;
    async fn from_request(ctx: &RequestContext) -> Result<Self, Self::Rejection>;
}
```

### 13.3 State Injection
- `.state(AppState)` ile tek state
- Advanced: `.with_state::<T>(value)` multiple typed state registry

### 13.4 AuthUser Injection
Auth middleware request içine extension yazar:
- `ctx.extensions.insert(AuthUser { ... })`
Extractor bunu okur.

### 13.5 Body / JSON
- `Content-Type: application/json` default
- `body size limit` + error format

---

## 14. Validasyon Sistemi (Pydantic-Style)

### 14.1 Tasarım Hedefleri
- Kurallar struct alanında tanımlı
- Nested validation
- Çoklu hata raporu (tüm alanlar)
- Hata mesajı: alan + kural + human readable message

### 14.2 Hata Formatı (Öneri)
HTTP 422 (veya 400) — *tercih*: 422 “Unprocessable Entity”

```json
{
  "error": {
    "type": "validation_error",
    "message": "Request validation failed",
    "fields": [
      { "field": "email", "code": "email", "message": "Invalid email format" },
      { "field": "password", "code": "length", "message": "Must be at least 8 characters", "min": 8 }
    ]
  },
  "request_id": "req_01J..."
}
```

### 14.3 Validasyon Kuralları
MVP set:
- `email`
- `length(min, max)`
- `range(min, max)` (numbers)
- `regex("...")`
- `non_empty`
- `optional` fields: `Option<T>` validate only if Some

### 14.4 Nested Validation
```rust
#[derive(Validate)]
struct Address {
  #[validate(length(min=2))]
  city: String,
}

#[derive(Validate)]
struct UserIn {
  #[validate(nested)]
  address: Address,
}
```

### 14.5 OpenAPI Entegrasyonu
Validasyon kuralları OpenAPI schema’larına yansıtılır:
- `minLength`, `maxLength`
- `format: email`
- `pattern`
- numeric `minimum/maximum`

---

## 15. OpenAPI + Swagger UI Otomasyonu

### 15.1 Endpoint’ler
- `GET /openapi.json` → OpenAPI spec
- `GET /docs` → Swagger UI (OpenAPI’yı buradan okur)

### 15.2 Route Metadata
Macro route için:
- method
- path
- request schema
- response schema
- errors
- tags/summary/description (opsiyon)

### 15.3 Swagger UI Sunumu
- Static asset’leri embed et (include_bytes!)
- veya feature flag ile external CDN (opsiyon)

### 15.4 “Docs-First” İyileştirmeler
- `#[rustapi::tag("Users")]`
- `#[rustapi::summary("Register a new user")]`
- `#[rustapi::description("Creates a user and returns minimal profile")]`

---

## 16. Response Modeli & Hata Tasarımı

### 16.1 `IntoResponse` Modeli
Her handler sonucu, bir `Response`’a çevrilir.

- `T: Serialize` → JSON 200
- `Created<T>` → 201
- `NoContent` → 204
- `ApiError` → hata response

### 16.2 Result Tipi
Hedef:
```rust
pub type Result<T> = std::result::Result<T, ApiError>;
```
veya generic:
```rust
pub type Result<T, E = ApiError> = std::result::Result<T, E>;
```

### 16.3 Standart ApiError
Kategori tabanlı:
- `ValidationError`
- `Unauthorized`
- `Forbidden`
- `NotFound`
- `Conflict`
- `Internal`

Örnek JSON:
```json
{
  "error": {
    "type": "not_found",
    "message": "Item not found"
  },
  "request_id": "req_..."
}
```

### 16.4 Error Leaking (Prod)
- Debug modda: detaylı (opsiyon)
- Prod modda: internal details gizli
- “error_id” ile log correlate

---

## 17. Middleware & Ekosistem

### 17.1 Tower Uyumu
RustApi builder:
```rust
RustApi::new()
  .layer(tower_http::trace::TraceLayer::new_for_http())
  .layer(cors_layer())
  .mount(...)
```

### 17.2 Batteries Included (Faz 3)
- JWT auth middleware
- CORS presets
- Rate limiting
- Request ID
- Compression
- Timeout

Feature flags:
- `rustapi = { version="...", features=["jwt", "cors"] }`

### 17.3 SQLx Integration
RustAPI “db pool”ı state/extension üzerinden verir:
- `DbPool` extractor

---

## 18. Konfigürasyon & Ortam Yönetimi

### 18.1 Config Kaynakları
- `.env`
- env vars
- `config.toml` (opsiyon)
- `RustApi::config(Config { ... })`

### 18.2 Örnek Config
```rust
#[derive(Clone)]
struct Config {
  port: u16,
  log_level: String,
  jwt_secret: String,
}
```

### 18.3 Ortam Profilleri
- `dev`: docs açık, debug errors daha detay
- `prod`: docs opsiyonel, strict security

---

## 19. Güvenlik

### 19.1 Varsayılanlar
- Body size limit: örn 1MB
- JSON parse error → standard error
- Internal error → generic message

### 19.2 JWT
- Bearer token parse
- claims struct
- `AuthUser` extractor

### 19.3 CORS
- sane defaults
- allowlist origin

### 19.4 Rate Limit
- ip based
- token bucket

---

## 20. Gözlemlenebilirlik (Logs/Tracing/Metrics)

### 20.1 Tracing
- her request → span
- request_id
- latency
- status code

### 20.2 Metrics
- prometheus exporter (opsiyonel feature)
- request count, latency buckets

---

## 21. Test Stratejisi

### 21.1 Unit Tests
- router matching
- extractor conversion
- validation rules

### 21.2 Integration Tests
- in-memory server
- request/response golden tests

### 21.3 Doc Tests
- README code blocks compile

### 21.4 Compatibility Tests
- tower layer interplay
- hyper version compatibility

---

## 22. Performans Stratejisi

- Router: radix tree
- Avoid allocations:
  - path param parse minimal
  - reuse buffers
- JSON:
  - serde_json default; advanced: simd-json (feature)

Benchmark hedefleri:
- Hello World throughput
- JSON echo
- CRUD endpoint (DB mock)

---

## 23. Örnek Proje: Mini CRUD

### 23.1 Model
```rust
#[derive(Schema, Serialize, Deserialize, Validate)]
struct TodoIn {
  #[validate(length(min=1))]
  title: String,
}

#[derive(Schema, Serialize)]
struct TodoOut {
  id: i64,
  title: String,
  done: bool,
}
```

### 23.2 Endpoints
```rust
#[rustapi::post("/todos")]
async fn create_todo(state: AppState, body: TodoIn) -> Result<Created<TodoOut>> {
  let todo = state.todos.create(body).await?;
  Ok(Created(todo))
}

#[rustapi::get("/todos/{id}")]
async fn get_todo(state: AppState, id: i64) -> Result<TodoOut> {
  Ok(state.todos.get(id).await?)
}

#[rustapi::get("/todos")]
async fn list_todos(state: AppState) -> Result<Vec<TodoOut>> {
  Ok(state.todos.list().await?)
}

#[rustapi::delete("/todos/{id}")]
async fn delete_todo(state: AppState, id: i64) -> Result<NoContent> {
  state.todos.delete(id).await?;
  Ok(NoContent)
}
```

### 23.3 App
```rust
#[rustapi::main]
async fn main() -> Result<()> {
  RustApi::new()
    .state(AppState::new())
    .mount(create_todo)
    .mount(get_todo)
    .mount(list_todos)
    .mount(delete_todo)
    .docs("/docs")
    .run("127.0.0.1:8080")
    .await
}
```

---

## 24. Roadmap & Milestones

### Faz 1: MVP
- Minimal router + macros
- JSON request/response
- `#[rustapi::main]`
- basic state injection
- standard error model (minimal)

**Milestone kriterleri**
- Hello World < 5 satır
- 1 endpoint + state + json body çalışır

### Faz 2: Validasyon + OpenAPI
- derive validate
- validation error JSON
- OpenAPI registry + `/openapi.json`
- Swagger UI `/docs`

**Milestone kriterleri**
- Struct kuralları OpenAPI’ye yansır
- Swagger UI endpointleri listeler

### Faz 3: Batteries Included
- JWT
- CORS
- Rate limit
- SQLx example

**Milestone kriterleri**
- 15 dakikada CRUD + Auth tutorial

### Faz 4: Ergonomi + Stabilizasyon
- compile errors’ı iyileştirme
- docs & guides
- v1.0 hazırlığı

---

## 25. Başarı Kriterleri

- **Hello World** 5 satırın altında
- Yeni başlayan biri **15 dk** içinde CRUD
- Hata mesajları:
  - validasyon: açık ve alan bazlı
  - route conflict: anlaşılır
- Docs daima güncel:
  - `/docs` ile canlı

---

## 26. Riskler & Mitigasyon

### 26.1 Procedural Macro Karmaşıklığı
**Risk:** macro debug zor.  
**Çözüm:**
- macro output’u debug feature ile göster
- minimal macro + core’da çoğu işi trait’e bırak

### 26.2 OpenAPI / Validation bağımlılıkları
**Risk:** third-party crate değişiklikleri.  
**Çözüm:** wrapper crate ile API’yi stabilize et.

### 26.3 “Too Magic” Algısı
**Risk:** ileri kullanıcılar kontrol kaybeder.  
**Çözüm:** escape hatch:
- raw request extractor
- tower layer config
- explicit wrappers opsiyon

### 26.4 Compile Times
**Risk:** derive + schema compile time uzatır.  
**Çözüm:** feature flags:
- `openapi` kapatılabilir
- `validate` kapatılabilir
- incremental compile önerileri

---

## 27. Katkı Rehberi (Contributing)

- `cargo test` tüm workspace
- `cargo fmt` zorunlu
- `cargo clippy -D warnings`
- PR’larda:
  - yeni özellik → örnek + doc
  - breaking change → changelog notu

Issue şablonları:
- bug report
- feature request
- docs improvement

---

## 28. Lisans & İsimlendirme

- Lisans: MIT veya Apache-2.0 (seçim projeye göre)
- “RustAPI” adı crates.io’da uygun değilse geçici kod adı:
  - `rustapi-rs`
  - `rustapi-framework`
  - `oxideapi`

---

# Ek: Tasarım Notları (Derin Teknik)

## A. Macro Çıktı Mantığı (Yüksek Seviye)

`#[rustapi::get("/x/{id}")]` macro’su:
1. handler fn token stream parse
2. signature analysis:
   - param list
   - return type
3. route registry’ye bir `RouteDef` üretir:
   - method
   - path template
   - handler pointer
   - schema ids
4. compile-time registry:
   - `inventory` crate veya generated static list

> Not: inventory, link-time registry yaklaşımı sağlar; DX iyi.

## B. Error Model: RFC 7807 (Problem Details) Uyumu
İstersen hata formatını RFC 7807 tarzına yaklaştırabilirsin:
- `type`, `title`, `status`, `detail`, `instance`
Ama FastAPI stili “fields array” ile daha pratik.

## C. Response Status Inference
- `Ok(T)` default 200
- `Ok(Created(T))` 201
- `Ok(NoContent)` 204

## D. “Rust Zor” Algısını Kırma
- örnekler + template repo
- “copy/paste ready” guides
- compile error’larda custom hints

---

## Son Söz

RustAPI’nin ürünü “sadece bir framework” değil; aynı zamanda **Rust’ta web geliştirme ergonomisi standardı** kurma denemesi.

- Basit başlayıp (MVP),
- validasyon + docs ile “wow” etkisi yaratıp,
- batteries included ile pratik hale getirip,
- v1.0’da stabilize etmek.
