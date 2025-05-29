# Webhook Mock Server (Tested with Jira webhooks)

A simple mock webhook server in Rust that simulates receiving webhooks from **Jira** (or any other service), and stores them in memory for inspection.

This is useful for testing, local development, or debugging webhook integrations without needing to call real services.

---

## 🚀 Features

- Accepts POST requests at `/jira-webhook`
- Stores each webhook payload (including headers, body, and timestamp) in memory
- Provides a GET endpoint at `/webhooks` to list all received webhooks
- Built with [Actix Web](https://actix.rs/), fast and ergonomic Rust web framework

---

## 🔧 Setup & Usage

### Prerequisites

- [Rust toolchain](https://rust-lang.org)
- Cargo (comes with Rust)

### Clone the repo

```bash
git clone https://github.com/yourusername/jira-webhook-mock.git
cd jira-webhook-mock
```

### Build and run

```bash
cargo run
```

The server will start on port `9998` (or any available port):

```
Starting mock Jira webhook server on http://localhost:9998
```

---

## 📡 Endpoints

### `POST /jira-webhook`

Used by external services to send webhook payloads.

- Accepts JSON body
- Captures headers and timestamp
- Returns: `{"status": "success"}` on success

### `GET /webhooks`

Returns a JSON array of all captured webhook payloads:

```json
[
  {
    "timestamp": "1712345678",
    "headers": [["content-type", "application/json"], ...],
    "body": {"issue": {...}}
  },
  ...
]
```

---

## 🧪 Example Request

You can simulate a webhook using `curl`:

```bash
curl -X POST http://localhost:9998/jira-webhook \
     -H "Content-Type: application/json" \
     -H "X-Event-Type: jira:issue_created" \
     -d '{"issue": {"key": "PROJ-123"}}'
```

Then retrieve it:

```bash
curl http://localhost:9998/webhooks
```

---

## 📁 Project Structure

- `main.rs`: Main application logic
- `Cargo.toml`: Rust project configuration
- `README.md`: This file

---

## 🧑‍💻 Contributing

Feel free to open issues or PRs to improve this mock server — contributions are welcome!

---

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
