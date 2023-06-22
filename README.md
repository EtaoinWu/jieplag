# JiePlag

## Local binaries

* `core/src/bin/find_pairs.rs`: Find pairs of files that contain possible plagiarism
* `core/src/bin/compute_matches.rs`: Compute matched text blocks from two source files (and optional teamplte file)

## Run server

Configuration: `server/.env.sample`.

* `server/src/bin/create_user.rs`: Create users in database
* `server/src/bin/server.rs`: Run web server to accept requests
* `client/srv/bin/cli.rs`: CLI tool to submit to server

Setup postgres database in postgres shell:

```sql
create database jieplag;
create user jieplag with encrypted password 'REDACTED';
grant all privileges on database jieplag to jieplag;
\c jieplag postgres
grant all on schema public to jieplag;
```