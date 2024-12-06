# Unwritten Take-Home Assignment

## Assignment

> Spin up a simple Rust API framework of your choice (Axum, Actix, Rocket etc). Create a connection to a PostgreSQL database and fill it with some dummy data (hint: you can create a new database using neon.tech or other third parties). 
>
> Ingest the data, convert it from PostgreSQL to a Polars LazyFrame and print the output. This conversion must be done in an efficient manner, avoiding excessive row iteration.

## Installation

### System prerequisites
- Git
- Rust with MSRV 1.81.0
### Clone the repo
### Create a database
- Use [neon.tech](neon.tech)
- Follow details [below](#database)
### Create a `.env` file based on [template.env](./template.env)

## Running
- Start the server with `cargo run` or `cargo run --release`
- In another terminal, send a POST request to 127.0.0.1:8080
- Observe the output in the first terminal

## Implementation

### API Framework
- Actix is the web framework used.
- A single route is defined: a POST action to the root ("/").
- This triggers ingestion and printing of the data, and returns 200 OK with no body on success.
- Handled application errors are logged, and 500 is returned (see [errors.rs](./src/errors.rs)]
- Logging middleware is added to the Actix App instances (at [main.rs:60](./src/main.rs:60)), to benefit from the framework's logging capabilities

### Database
The database is populated roughly according to [this Neon tutorial](https://neon.tech/docs/get-started-with-neon/signing-up#step-3-add-sample-data), but with more rows added

In summary:
- A database named `playing_with_neon` is created with the columns `id SERIAL PRIMARY KEY, name TEXT NOT NULL, value REAL`
- 10,000 rows are inserted with random values

Here is the code snippet to accomplish this:
```sql
CREATE TABLE IF NOT EXISTS playing_with_neon(id SERIAL PRIMARY KEY, name TEXT NOT NULL, value REAL);
INSERT INTO playing_with_neon(name, value)
  SELECT LEFT(md5(i::TEXT), 10), random() FROM generate_series(1, 10) s(i);
```

### Data ingestion and conversion to LazyFrame
- The data is ingested using an `ARRAY_AGG` query, aggregating each of the columns into a single array.
- The response to the query is a single row containing each of these arrays.
- From here, it is straightforward to instantiate a LazyFrame from the columns.
- The aggregation query is likely to have linear time complexity, since each row must be visited once by the database engine.
- The conversion to a DataFrame (in [data_columns.rs](./src/data_columns.rs)) is also linear. Thus the overall complexity is O(n) where n is the number of rows
