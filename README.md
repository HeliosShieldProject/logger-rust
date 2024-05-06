# Helios | Logger

[![AGPL-3.0 License](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://choosealicense.com/licenses/agpl-3.0/)

The Logger is a simple HTTP server that accepts log entries and stores them in a MongoDB database.

## Features

- Accepts log entries via HTTP POST requests.
- Stores log entries in a MongoDB database.
- Minimal setup and easy to use.

## Prerequisites

Before running the Logger, ensure you have the following installed:

- Rust programming language and Cargo package manager
- MongoDB server

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/HeliosShieldProject/logger-rust.git
    ```

2. Navigate to the project directory:

    ```bash
    cd logger-rust
    ```

3. Install dependencies:

    ```bash
    cargo build --release
    ```

## Usage

1. Configure the environment variables in the `.env` file:

    ```plaintext
    MONGO_URL=<MongoDB connection URL>
    LOGGER_PORT=<Port number for the logger service>
    ```

2. Run the Logger Service:

    ```bash
    cargo run
    ```

3. Once the service is running, it will listen for HTTP POST requests at `http://localhost:<LOGGER_PORT>/log`.

4. Send log entries to the service using HTTP POST requests with JSON payloads containing the log data.

## Example

To send a log entry to the service, you can use tools like cURL or Postman. Here's an example using cURL:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"message": "This is a log message"}' http://localhost:<LOGGER_PORT>/log
```