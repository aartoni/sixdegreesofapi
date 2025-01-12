# Six Degrees of API

A standalone API layer for "Six Degrees of" projects, written from scratch to match the requirements of [Six Degrees of Frontend](https://github.com/aartoni/sixdegreesoffrontend).

## Usage

The program can be started using Cargo.

```sh
cargo run --release
```

When running a full "Six Degrees of" project, you'll likely want to run this as part of your `compose.yml`.

```yml
services:
  api:
    environment:
      - PORT
    ports:
      - "${PORT}:${PORT}"
  db:       # ...
  frontend: #...
```

## Contributing

Contributions to the project are welcome!
