# my-data

API for personal data storage.

## Initial setup

1. Copy .env.template to .env

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
