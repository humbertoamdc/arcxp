ArcXP Ticketing API (POC)
===
A lightweight, production-ready Rust-based API for managing support tickets,
built with Axum and deployed to AWS Lambda. This project serves as a proof of
concept (POC) for a job application and is designed to be minimal, well-structured,
and production-minded.

### [Public URL](https://arcxp.humbertotech.com)

### [Swagger UI](https://humbertoamdc.github.io/arcxp/#/)

✨ Features
---

* Create support tickets
* Assign tickets to engineers
* Update ticket statuses through a lifecycle
* Bulk import tickets
* RESTful API with OpenAPI documentation (Swagger UI)
* Tested against LocalStack (DynamoDB)
* Deployed to AWS Lambda using GitHub Actions

🎓 Tech Stack
---

* Language: Rust (Axum)
* Infra: AWS Lambda, DynamoDB, API Gateway (via CloudFormation)
* Testing: LocalStack, serial_test, Docker Compose
* CI/CD: GitHub Actions

💡 Requirements
---

- Docker
- [AWS CLI](https://aws.amazon.com/cli/) + AWS credentials for deployment
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [cargo-watch](https://crates.io/crates/cargo-watch) (for local dev)

🚀 Getting Started
===

### 1. Clone the repo

git clone https://github.com/humbertoamdc/arcxp.git

cd arcxp

### 2. Run Locally (with LocalStack)

`make run`

This starts:

LocalStack (via Docker Compose)

The Axum server on port 3000

Auto-reloading with cargo-watch

### 3. Run Tests

`make test`

This spins up a LocalStack testing instance (port 4576), launches the API, and runs tests against it.

Make sure nothing is using ports 3001 or 4576 before running.

🚧 Project Structure
===

```
├── controllers      # API endpoints
├── usecases         # Business logic, orchestrates repository
├── repository       # DynamoDB logic
├── infra/           # CloudFormation YAML templates
├── scripts/         # Dev and test shell scripts
├── openapi.yaml     # API spec for Swagger
├── docker-compose.* # LocalStack environments
└── Makefile         # CLI task shortcuts
```

✅ API Overview
===
The full spec is available here:

https://humbertoamdc.github.io/arcxp/#/

Endpoints:

```
GET  /tickets               - Get all existing tickets
POST /tickets               - Create a new ticket
PUT  /tickets/assignee      - Assign ticket
PUT  /tickets/status        - Update ticket status
POST /tickets/batch         - Bulk create tickets
```

✅ Test Coverage
===
Test files are located in `tests/controllers/`.

Uses serial_test to prevent conflicts during LocalStack execution.

Each test runs against a real local DynamoDB instance.

Test infra defined in docker-compose.test.yaml.
