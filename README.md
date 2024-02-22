# my-data

API for personal data storage.

## Initial setup

1. Clone this repo
2. `cd` into my-data
3. Copy .env.template to .env

### Database
1. Create a postgres database for the API to use.
2. Assign the database url for the database created in the step above to the DATABASE_URL variable in .env
3. Install Diesel CLI (ORM to interact with the database):

```
cargo install diesel_cli --no-default-features --features postgres
```

4. Apply the migrations available in the 'migrations' directory:

```
diesel migration run
```

Ensure that all schemas are imported to the schema file. By default Diesel uses the public schema to generate the schema file. Until Diesel updates their cli to support multiple schemas (there is a ticket open on GitHub atm - [splitting into multiple schema.rs](https://github.com/diesel-rs/diesel/pull/3796)), you will have to manually generate the other schemas by using the `print-schema` command. For example, to add the financial schema, run the following command:

```
diesel print-schema -s financial >> src/schema.rs
```

### API Server
1. Assign the API_URL, API_PORT, and SECRET_TOKEN in .env
2. Run the server:

```
cargo run
```
