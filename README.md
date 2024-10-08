# clean_architecture_rust
クリーンアーキテクチャをRustを使って実装したサンプル  
GraphQLのTODOアプリ

# 環境
- Rust 1.81
- PostgreSql
- Docker
- docker-compose
- make

# 起動
- cp .env-sample .env
- make up
- make m-run
- make app
- cargo run

# 構成
![architecture](./docs/architecture.svg)
<pre>
.
└── src
    ├── domain
    │   ├── model
    │   ├── primitive
    │   └── service
    ├── infrastructure
    │   ├── database
    │   ├── graphql
    │   └── memory
    ├── interface_adapter
    │   ├── adapter
    │   ├── controller
    │   ├── gateway
    │   └── port
    ├── main.rs
    └── shared
</pre>

### domain
ドメイン・ユースケース層。  
- モデル -> model  
- ビジネスロジックの型(値オブジェクト) -> primitive  
- ユースケース -> service  

### interface_adapter
アダプター層。
- 各データオブジェクトの変換 -> adapter  
- プレゼンテーション層へのコントローラ -> controller  
- インフラのインターフェース -> gateway  
- ドメイン・ユースケース層のinput/outputポート -> port

### infrastructure
インフラ層。
- database -> データベースの実装
- graphql -> GraphQLの実装(main.rsで実行されサーバが起動する)
- memory -> インメモリの実装

# 参考文献
- Clean Architecture 達人に学ぶソフトウェアの構造と設計 - Robert C.Martin
- 手を動かしてわかるクリーンアーキテクチャ ヘキサゴナルアーキテクチャによるクリーンなアプリケーション - Tom Hombergs
