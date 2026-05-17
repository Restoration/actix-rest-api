# Actix REST API - Clean Architecture 設計規約

## プロジェクト概要
Rust + actix-web 4 + SeaORM 0.12 による REST API。Clean Architecture に基づくレイヤー分離を採用。

## ディレクトリ構成

```
src/
├── main.rs            # エントリーポイント (DI組み立て・サーバー起動)
├── config/            # 環境変数からの設定読み込み (envy)
│   ├── mod.rs
│   └── config.rs      # Config struct (DATABASE_URL, PORT)
├── domain/            # ドメイン層 (フレームワーク非依存)
│   ├── mod.rs
│   ├── user.rs        # エンティティ (User, UserId, Users)
│   └── error.rs       # AppError enum (thiserror)
├── port/              # ポート層 (trait定義)
│   ├── mod.rs
│   └── user_port.rs   # UserPort trait (repository が実装する)
├── interactor/        # ユースケース層
│   ├── mod.rs
│   └── user_interactor.rs  # UserUseCase trait + UserInteractor 実装
├── repository/        # インフラ層 (port の実装)
│   ├── mod.rs
│   └── user_repository.rs  # UserRepository (SeaORM で UserPort を実装)
├── dao/               # SeaORM エンティティ定義
│   ├── mod.rs
│   └── user.rs        # sea_orm Entity/Model (テーブル: users)
├── presenter/         # プレゼンテーション層 (HTTP ハンドラ)
│   ├── mod.rs
│   ├── user_presenter.rs        # get_user, get_users ハンドラ + UserResponse
│   ├── health_check_presenter.rs # health_check ハンドラ
│   └── error.rs                  # AppError → HTTP StatusCode 変換
├── router/            # ルーティング定義
│   ├── mod.rs
│   └── router.rs      # routes() 関数
└── container/         # DI コンテナ
    ├── mod.rs
    └── container.rs   # Container struct (Arc<dyn UserUseCase>)
```

## レイヤー依存関係

```
presenter → interactor(trait) → port(trait) ← repository
                ↑                                ↓
            container (DI)                     dao (SeaORM)
```

- **domain**: 他のどの層にも依存しない。純粋な Rust 型。
- **port**: domain のみに依存。async_trait で trait を定義。
- **interactor**: port trait に依存。UserUseCase trait を公開し、UserInteractor が実装。
- **repository**: port trait を実装。SeaORM + dao に依存。
- **dao**: SeaORM のエンティティ定義のみ。
- **presenter**: actix-web ハンドラ。Container 経由で UserUseCase trait を呼ぶ。
- **container**: Arc<dyn UserUseCase> を保持する DI コンテナ。
- **router**: actix-web のルーティング設定。

## 技術スタック

| Crate | 用途 |
|-------|------|
| actix-web 4 | Web フレームワーク |
| sea-orm 0.12 | ORM (MySQL, sqlx-mysql) |
| envy | 環境変数 → Config struct |
| thiserror | エラー型定義 |
| async-trait | async fn in trait |
| serde | JSON シリアライズ/デシリアライズ |
| env_logger + log | ログ |

## コーディング規約

### 新しいエンティティ追加時の手順
1. `domain/` にエンティティ struct + 型エイリアスを定義
2. `port/` に trait を定義 (`async_trait`, `Send + Sync`)
3. `interactor/` に UseCase trait + Interactor struct を実装
4. `dao/` に SeaORM エンティティを定義
5. `repository/` に port trait の実装を追加
6. `presenter/` に HTTP ハンドラ + Response struct を追加
7. `router/router.rs` にルートを追加
8. `container/container.rs` に UseCase を追加
9. `main.rs` で DI を組み立て

### テスト方針
- **domain**: 単体テスト (純粋な Rust)
- **interactor**: MockPort を使った単体テスト
- **presenter**: MockUseCase + actix_web::test でハンドラテスト
- テストは各ファイル内に `#[cfg(test)] mod tests` で記述

### エラーハンドリング
- `domain::AppError` で全エラーを定義 (thiserror)
- `presenter::error` で `ResponseError` を実装し HTTP ステータスへ変換
- repository 層では `map_err` で DB エラーを `AppError` に変換しログ出力

### DI パターン
- `Container` struct が `Arc<dyn UseCase>` を保持
- `web::Data<Container>` として actix-web に登録
- presenter は `web::Data<Container>` を受け取り trait 経由で呼び出し

## コマンド

```sh
# ビルド
cargo build

# テスト
cargo test

# 実行
cargo run

# マイグレーション
sea-orm-cli migrate up
```
