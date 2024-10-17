# PreDB IRC Bot

This is a Rust-based IRC bot designed to monitor and log pre-release information from IRC channels. It's my first Rust project, built for learning purposes.

## Project Overview

The PreDB IRC Bot connects to a specified IRC server and channel, listens for pre-release announcements, parses the messages, and stores the information in a PostgreSQL database. It's designed to be efficient, reliable, and easy to configure.

## Features

- Connects to IRC servers and joins specified channels
- Parses PRE messages to extract release information
- Stores release data in a PostgreSQL database
- Handles IRC formatting and color codes
- Implements logging for better debugging and monitoring

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- `.env` file for configuration

## Configuration

Create a `.env` file in the root directory of the project with the following content:

```
DATABASE_URL=postgres://username:password@localhost/database_name
```

Replace `username`, `password`, and `database_name` with your PostgreSQL credentials.

## How to Run

1. Clone the repository:

   ```
   git clone https://github.com/parnexcodes/predb-irc-bot.git
   cd predb-irc-bot
   ```

2. Install dependencies:

   ```
   cargo build
   ```

3. Set up the database:
   Ensure your PostgreSQL server is running and the database specified in the `DATABASE_URL` exists.

4. Run the application:
   ```
   cargo run
   ```

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/irc_client.rs`: Handles IRC connection and message processing
- `src/database.rs`: Manages database operations
- `src/message_parser.rs`: Parses IRC messages to extract relevant information
- `src/utils.rs`: Contains utility functions for string manipulation

## Customization

You can customize the IRC server, channel, and bot nickname by modifying the `Config` struct in the `run_irc_client` function in `src/irc_client.rs`:

## Database Schema

The application uses a PostgreSQL database with the following schema:

```
sql
CREATE TABLE IF NOT EXISTS releases (
id UUID PRIMARY KEY,
release_name TEXT NOT NULL,
group_name TEXT NOT NULL,
category TEXT NOT NULL,
created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
)
```

## Message Parsing

The bot parses PRE messages using a regular expression. The expected format is:

```
(PRE) (category) (release_name)
```

The group name is extracted from the release name.

## Logging

The application uses the `log` and `env_logger` crates for logging. You can adjust the log level by setting the `RUST_LOG` environment variable before running the application:

```
export RUST_LOG=debug
cargo run
```

## Error Handling

The application implements error handling and logging for various scenarios, including:

- Database connection and query errors
- IRC connection and message parsing errors
- General runtime errors

## License

This project is open-source and available under the MIT License.

## Future Improvements

- Add unit tests for message parsing and database operations
- Implement more robust error recovery mechanisms
- Add support for multiple IRC channels
- Create a web interface for viewing stored release information
