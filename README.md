# nlytics

A lightweight, high-performance event analytics backend built with Rust. Designed to collect and store arbitrary JSON events from your applications, while tracking real-time online sessions via Redis.

## Features

- 📥 **Event ingestion** — collect arbitrary JSON events via a simple HTTP API
- 📊 **Event retrieval** — query stored events with private key protection
- 🟢 **Online presence tracking** — track active user sessions in real-time using Redis TTL
- 🔐 **Dual API key authentication** — separate public and private keys with HMAC-SHA256 request signing
- 🚀 **Built with Rust** — Actix-web, SQLx, deadpool-redis

---

## Tech Stack

| Layer       | Technology                  |
|-------------|-----------------------------|
| Runtime     | Rust                        |
| Web         | Actix-web                   |
| Database    | PostgreSQL + SQLx           |
| Cache       | Redis + deadpool-redis      |
| Auth        | HMAC-SHA256 request signing |
| Deployment  | Docker + Docker Compose     |

---

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/your-username/nlytics.git
cd nlytics
```

### 2. Configure environment

Create a `.env` file in the project root:

```env
# PostgreSQL
POSTGRES_USER=postgres
POSTGRES_PASSWORD=yourpassword
POSTGRES_DB=nlytics

# Application
HOST=0.0.0.0
PORT=8080
DATABASE_URL=postgres://postgres:yourpassword@db:5432/nlytics
REDIS_URL=redis://redis:6379

# Authentication
PUBLIC_API_KEY=your_public_key
PRIVATE_API_KEY=your_private_key
SECRET_KEY=your_hmac_secret
```

> ⚠️ **Never use default or weak values for `SECRET_KEY`, `PUBLIC_API_KEY`, or `PRIVATE_API_KEY` in production.**

### 3. Run with Docker Compose

```bash
docker compose up --build -d
```

The API will be available at `http://localhost:8080`.

---

## Authentication

Every request must include two headers:

| Header        | Description                                        |
|---------------|----------------------------------------------------|
| `X-API-KEY`   | Your public or private API key                     |
| `X-SIGNATURE` | Base64-encoded HMAC-SHA256 signature of the request body |

### Key types

| Key type          | Access level                                     |
|-------------------|--------------------------------------------------|
| `PUBLIC_API_KEY`  | Write-only — post events and manage sessions     |
| `PRIVATE_API_KEY` | Full access — includes reading events and stats  |

### Generating a signature

Compute `HMAC-SHA256` over the raw request body using your `SECRET_KEY`, then Base64-encode the result.

**Example (Python):**
```python
import hmac, hashlib, base64, json

secret = b"your_hmac_secret"
body = json.dumps({"type": "page_view", "data": {"url": "/home"}}).encode()

signature = base64.b64encode(
    hmac.new(secret, body, hashlib.sha256).digest()
).decode()
```

**Example (Node.js):**
```js
const crypto = require("crypto");

const secret = "your_hmac_secret";
const body = JSON.stringify({ type: "page_view", data: { url: "/home" } });

const signature = crypto
  .createHmac("sha256", secret)
  .update(body)
  .digest("base64");
```

> For `GET` requests with an empty body, compute the HMAC over an empty string `""`.

---

## API Reference

### Events

#### `POST /api/v1/events`
Record a new event.

**Auth:** Public or Private key

**Headers:**
```
X-API-KEY: <public_or_private_key>
X-SIGNATURE: <base64_hmac_signature>
Content-Type: application/json
```

**Request body:**
```json
{
  "type": "page_view",
  "data": {
    "url": "/home",
    "user_agent": "Mozilla/5.0"
  }
}
```

| Field  | Type   | Required | Description                          |
|--------|--------|----------|--------------------------------------|
| `type` | string | ✅       | Event type identifier                |
| `data` | object | ✅       | Arbitrary JSON payload for the event |

**Response `201 Created`:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "type": "page_view",
  "data": {
    "url": "/home",
    "user_agent": "Mozilla/5.0"
  }
}
```

---

#### `GET /api/v1/events`
Retrieve recorded events (up to 100, ordered by newest first).

**Auth:** Private key only 🔒

**Headers:**
```
X-API-KEY: <private_key>
X-SIGNATURE: <base64_hmac_of_empty_body>
```

**Response `200 OK`:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "type": "page_view",
    "data": { "url": "/home" }
  }
]
```

---

### Online Presence

#### `POST /api/v1/online/init`
Initialize a new session. Returns a `session_id` to be stored on the client and used for subsequent pings. The session expires after **60 seconds** if not refreshed.

**Auth:** Public or Private key

**Headers:**
```
X-API-KEY: <public_or_private_key>
X-SIGNATURE: <base64_hmac_of_empty_body>
Content-Type: application/json
```

**Request body:** `{}`

**Response `200 OK`:**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

---

#### `POST /api/v1/online/ping`
Refresh a session TTL to keep it alive. Call this periodically (e.g. every 30 seconds).

**Auth:** Public or Private key

**Headers:**
```
X-API-KEY: <public_or_private_key>
X-SIGNATURE: <base64_hmac_signature>
Content-Type: application/json
```

**Request body:**
```json
{
  "session": {
    "id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

**Response `200 OK`:** empty body

---

#### `GET /api/v1/online`
Get the count of currently active sessions.

**Auth:** Private key only 🔒

**Headers:**
```
X-API-KEY: <private_key>
X-SIGNATURE: <base64_hmac_of_empty_body>
```

**Response `200 OK`:**
```json
{
  "current": 42
}
```

---

## Error Responses

| Status | Meaning                                          |
|--------|--------------------------------------------------|
| `401`  | Missing or invalid `X-API-KEY` or `X-SIGNATURE` |
| `413`  | Request body exceeds 1 MB limit                  |
| `500`  | Internal server error                            |

---

## Database Migrations

Migrations are managed automatically by SQLx on application startup. Files are located in the `migrations/` directory.

---

## License

This project is licensed under the **MIT License**.

MIT is chosen because nlytics is a self-hosted analytics tool intended to be freely used, modified, and integrated into both open-source and commercial projects without restriction.

```
MIT License

Copyright (c) 2026 nlytics contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

