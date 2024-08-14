# Rust Starter Kit

Rust Starter Kit is a web application project built with Rust. It leverages the following technologies:

- **Actix Web**: A powerful, pragmatic, and extremely fast web framework for Rust.
- **Diesel**: A safe, extensible ORM and Query Builder for Rust.
- **Tera**: A template engine for Rust that is easy to use and integrates well with Actix Web.

## Author

Derek Galbraith

## Installation

### 1. Install Rust

Rust is the programming language required to build and run this project. Follow the instructions below for your operating system.

#### Linux or macOS

Open a terminal and run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to install Rust.

#### Windows

    Download the Rust installer.
    Run the installer and follow the prompts.

### 2. Clone the Repository

Once Rust is installed, clone the Rust Starter Kit repository:

```bash
git clone https://github.com/derekg23/rust-starter-kit.git
cd rust-starter-kit
```

### 3. Build the Project

Navigate to the project directory and run the following command to build the project:

```bash
cargo build
```

### 4. Creating a PostgreSQL Database

Before running the application, you'll need to create a PostgreSQL database.

#### 1. **Install PostgreSQL**: If you don't already have PostgreSQL installed, you can install it using the following commands:

   - **Linux (Ubuntu):**
     ```bash
     sudo apt update
     sudo apt install postgresql postgresql-contrib
     ```

   - **macOS (using Homebrew):**
     ```bash
     brew install postgresql
     ```

   - **Windows:**
     Download and install PostgreSQL from the [official website](https://www.postgresql.org/download/windows/).

#### 2. **Create a Database**:

   - Start the PostgreSQL service:
     - **Linux (Ubuntu):**
       ```bash
       sudo service postgresql start
       ```
     - **macOS:**
       ```bash
       brew services start postgresql
       ```
     - **Windows**:
       Start the PostgreSQL service from the Services management console or via `pgAdmin`.

   - Switch to the PostgreSQL interactive terminal (psql):
     ```bash
     psql -U postgres
     ```

   - Create a new database:
     ```sql
     CREATE DATABASE <your-database-name>;
     ```

   - (Optional) Create a new user and grant privileges:
     ```sql
     CREATE USER rust_user WITH PASSWORD 'your_password';
     GRANT ALL PRIVILEGES ON DATABASE <your-database-name> TO rust_user;
     ```

### 5. Update environment file

Copy .env.local to .env and update .env with your database information

```bash
cp .env.local .env
```

### 6. Run the Migrations

Before running the application, you'll need to set up the database. Diesel makes this easy with its migration system:

```bash
diesel migration run
```

## Running the Application

To start the application, use the following command:

```bash
cargo run
```

The application will start, and you can access it in your browser at http://localhost:8080.

## License

This project is licensed under the MIT License.
