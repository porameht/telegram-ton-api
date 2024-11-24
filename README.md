# Telegram TON API

A robust REST API service built with Rust, providing account and project management functionality for Telegram TON blockchain integration.

## Features
- **Account Management**
  - Create, read, update, and delete accounts
  - Email and wallet address validation
  - Unique email enforcement
  - Project association tracking

- **Project Management**
  - Create, read, update, and delete projects
  - Project name validation
  - Timestamp tracking for creation and updates
  - Package and credential management

- **Technical Features**
  - RESTful API architecture
  - MongoDB integration
  - Async/await support
  - Error handling with custom error types
  - CORS support
  - Environment variable configuration
  - Comprehensive test coverage

## Prerequisites

- Rust (latest stable version)
- MongoDB (4.4 or later)
- Docker (optional, for containerized development)

## Environment Variables

Create a `.env` file in the root directory with the following variables: 

```env
MONGODB_URL=mongodb://localhost:27017
DATABASE_NAME=your_database_name
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/porameht/telegram-ton-api.git
cd telegram-ton-api
```

2. Install dependencies:
```bash
cargo build
```

3. Run the tests:
```bash
cargo test
```

4. Start the server:
```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Endpoints

### Accounts

- `POST /accounts` - Create a new account
- `GET /accounts` - List all accounts
- `GET /accounts/:id` - Get account details
- `PUT /accounts/:id` - Update an account
- `DELETE /accounts/:id` - Delete an account

### Projects

- `POST /projects` - Create a new project
- `GET /projects` - List all projects
- `GET /projects/:id` - Get project details
- `PUT /projects/:id` - Update a project
- `DELETE /projects/:id` - Delete a project

## Project Structure

src/
├── main.rs # Application entry point
├── error/ # Error handling
├── handlers/ # API route handlers
├── models/ # Data models
├── repository/ # Database operations
├── service/ # Business logic
└── logger/ # Logging configuration

## Testing

The project includes comprehensive unit tests for repositories and services. Run tests with:

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

Project Link: [https://github.com/porameht/telegram-ton-api](https://github.com/porameht/telegram-ton-api)
