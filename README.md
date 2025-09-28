# 🔗 Axum URL Shortener

A simple **URL shortener API** built with Rust, Axum, and SQLite.  
It lets you shorten long URLs and redirect them using short codes.

---

## ✨ Features
- 📝 Create short links with a simple POST request
- 🔀 Redirect from short code to original URL
- 💾 Data persistence with SQLite
- ⚡ Async, lightweight, and fast (Axum + Tokio)
- 🔒 Easy to extend with auth, rate-limiting, analytics

---

## 📦 Installation

Clone the repo and build:

git clone https://github.com/ahmetarabaci42/axum-url-shortener.git
cd axum-url-shortener
cargo build --release

The compiled binary will be at:
target/release/axum-url-shortener

---

## 🚀 Usage

Start the server:

cargo run

By default it runs on:
http://localhost:3000

---

## 🧪 API Examples

### Shorten a URL
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.rust-lang.org"}'

Response:
{
  "code": "abcd1234",
  "short_url": "http://localhost:3000/abcd1234"
}

### Resolve a short code
curl -v http://localhost:3000/abcd1234
# → 302 Redirect https://www.rust-lang.org

---

## ⚙️ Dependencies
- tokio – async runtime
- axum – web framework
- tower – middleware
- serde / serde_json – serialization
- sqlx – SQLite database
- uuid – short code generation
- anyhow – error handling

---

## 🛠️ Roadmap
- [ ] Add custom short codes
- [ ] Add expiration dates for links
- [ ] Add analytics (click counts, timestamps, IP logging)
- [ ] Add authentication for admin panel

---

## 🤝 Contributing
Pull requests and issues are welcome.  
Follow Conventional Commits style for commits.

---

