# dodopayments

## Description

dodopayments is a backend application for managing users and transactions. It provides a secure and reliable platform for handling financial operations.

## Features

*   User registration and authentication
*   Transaction creation and management

## Rate Limiting

The application uses rate limiting to protect against abuse.

## JWT Authentication

The application uses JWT (JSON Web Tokens) for authentication. Users receive a JWT upon successful login, which they can use to access protected resources.

## Technologies Used

*   Backend Language = Rust
*   Web Framework = Axum
*   Database = PostgreSQL
*   ORM = Diesel
*   Asynchronous Runtime = Tokio

## Getting Started

### Prerequisites

*   Rust toolchain: Ensure you have Rust installed. You can download it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
*   PostgreSQL database: You need a running PostgreSQL instance. You can download it from [https://www.postgresql.org/](https://www.postgresql.org/).

### Installation

1.  Clone the repository:

    ```bash
    git clone https://github.com/your-username/dodopayments.git
    cd dodopayments
    ```

2.  Configure the application:

    *   Create a `config/development.toml` file. You can use the provided `config/development.toml` as a template and modify the database credentials to match your PostgreSQL setup.

3.  Run the migrations:

    *   Install diesel cli
        ```bash
        cargo install diesel_cli --no-default-features --features postgres
        ```
    *   Run migrations
        ```bash
        diesel migration run
        ```

4.  Run the application:

    ```bash
    cargo run
    ```

## Docker

### Building the Image

```bash
docker build -t dodopayments .
```

### Running the Container

```bash
docker run -p 3001:3001 dodopayments
```

## Docker Compose

See the `docker-compose.yml` file for a complete setup of the project.

To run the application using Docker Compose, use the following command:

```bash
docker-compose up -d
```

## OpenAPI Specification

The OpenAPI specification for the API is available in the `openapi.yaml` file.
