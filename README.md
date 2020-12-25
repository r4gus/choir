# Choir

## Database

This application uses a database for users and other data. The Diesel ORM is used to integrate
the database.

### Settings

The application is setup to use Postgres as it's database but other database types can theoretically
be used to. A unit like struct `DbConn` is setup as fairing and used to interact with the database. The database
decorator uses the same name as the mentioned environment variable below (`#[database("postgres_db")]`) to
specify which database the application should connect to.

| Environment variable | Data | Description |
|:---------------------|-----:|------------:|
| POSTGRES\_DB         | postgres://username:password@localhost/DB\_demo | Address of the postgers server to use |

Alternatively the variables can also be set within the `Rocket.toml` file.

```
# Within Rocket.toml
[global.databases]
postgres_db = { url = "/path/to/postgerql/database" }
```

Please don't forget to set the `DATABASE_URL` either manually using `export` or
via the `.env` file to use the `diesel_cli` command line interface.

```
// in .env
DATABASE_URL=postgres://username:password@path/to/database
```

### Diesel

First install the __diesel__ command line application.

```
$ cargo install diesel_cli --no-default-features --features postgres
```

Then set the **DATABASE_URL** so diesel knows where to find the database.

```
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

or

```
export  DATABASE_URL=postgres://username:password@localhost/diesel_demo
```

Because the migration directory with all schemas does already exist we don't need
to run the `diesel setup` command.

#### Create new migration

To add or alter tables one can use migrations. To create a new migration you can run
`diesel migration generate migration_name` and then edit the newly created `up.sql` and
`down.sql` files.

#### Running migrations

To run migrations you can execute:

```
diesel migration run
```

To redo all migrations you can execute:

```
diesel migration redo
```
