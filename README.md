# Blog Backend

A RESTful API for efficiently processing and managing blog entries, providing functionalities such as creating, editing, deleting, and retrieving posts while adhering to modern security standards and best practices.

## Navigation
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
- [Contributing](#contributing)
- [License](#license)
- [Docker Compose Setup](#docker-compose-setup)
- [Cargo Dependencies](#cargo-dependencies)

## Features

- Create, edit, delete, and retrieve blog posts
- Secure authentication and authorization
- SQL database connection
- Dockerized SQL server hosting

## Technologies Used

- **Rust**: The main programming language used for building the API.
- **SQL**: For managing and storing blog entries.
- **Docker**: For containerizing the SQL server and the Rust application.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/xenya52/blog_backend.git
    cd blog_backend
    ```

2. Set up the environment variables:

    Create a `.env` file in the root directory and add the following:

    ```env
    DATABASE_URL=your_database_url
    ```

3. Start the application and the SQL server using Docker Compose:

    ```sh
    docker-compose up -d
    ```

## API Endpoints

### Blog Posts

- **GET** `/posts` - Retrieve all blog posts
- **GET** `/posts/{id}` - Retrieve a specific blog post by ID
- **POST** `/posts` - Create a new blog post
- **PUT** `/posts/{id}` - Update a blog post by ID
- **DELETE** `/posts/{id}` - Delete a blog post by ID

## Contributing

Contributions are welcome! Please fork the repository and create a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Cargo Dependencies

- ```actix-web = "4"```
- ```serde = { version = "1.0", features = ["derive"] }```
- ```serde_json = "1.0"```
- ```tokio = { version = "1", features = ["full"] }```
- ```sqlx = { version = "0.5", features = ["postgres", "runtime-tokio-rustls"] }```
- ```dotenv = "0.15"```
