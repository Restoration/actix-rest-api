# actix rest api

## Overview
This repository, a study for Rust with Clean architecture.
It implemented REST API which includes seoORM, actix framework, and any more.

## Dependency
- cargo
- sea-orm-cli
- docker-compose

## How to use
If you not installed sea-orm-cli, please install it.
```
$ cargo install sea-orm-cli
```

Launch API and prepare database.
```
$ cp dotenv .env
$ docker-compose up

// Then use another terminal.
$ cd migration
$ cargo run -- up

$ cd ../
$ cargo run
```

You can access to ``.


## Author
[Ryota](https://www.developer-ryota.com/)


## License
[MIT License]()