# actix-rest-api

## Overview
Rust + Clean Architecture の学習用リポジトリです。
actix-web 4 / SeaORM 0.12 を使った REST API を実装しています。

## Architecture
```
src/
├── config/        # 環境変数からの設定読み込み (envy)
├── domain/        # エンティティ・エラー定義 (フレームワーク非依存)
├── port/          # ユースケースが依存する trait (UserPort)
├── interactor/    # ユースケース実装 (UserInteractor)
├── repository/    # port の実装 / DB アクセス (SeaORM)
├── dao/           # SeaORM エンティティ定義
├── presenter/     # HTTP ハンドラ / レスポンス変換
├── router/        # ルーティング定義
├── container/     # DI コンテナ
└── main.rs
```

## Tech Stack
- [actix-web](https://actix.rs/) 4 — Web フレームワーク
- [SeaORM](https://www.sea-ql.org/SeaORM/) 0.12 — ORM (MySQL)
- [envy](https://github.com/softprops/envy) — 環境変数パーサ
- [thiserror](https://github.com/dtolnay/thiserror) — エラー型定義

## Endpoints
| Method | Path | Description |
|--------|------|-------------|
| GET | `/hc` | ヘルスチェック |
| GET | `/v1/users` | ユーザー一覧取得 |
| GET | `/v1/users/{id}` | ユーザー取得 |

## Prerequisites
- Rust (cargo)
- sea-orm-cli
- Docker Compose

## Getting Started

sea-orm-cli をインストールしていない場合:
```sh
cargo install sea-orm-cli
```

DB 起動 → マイグレーション → API 起動:
```sh
cp dotenv .env
docker-compose up -d

sea-orm-cli migrate up

cargo run
```

API は `http://127.0.0.1:8080` で起動します。

## Environment Variables
| Name | Required | Default | Description |
|------|----------|---------|-------------|
| `DATABASE_URL` | Yes | — | MySQL 接続文字列 |
| `PORT` | No | `8080` | リッスンポート |

## Author
[Ryota](https://www.developer-ryota.com/)

## License
[MIT License]()
