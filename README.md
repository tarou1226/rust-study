# Rustを勉強するリポジトリ
基本的にCargoというビルドシステムで管理を行う。
cargo new filename
↑ プロジェクトの作成を行う。
--vcs こちらのフラグを与えると、バージョン管理システムなどを弄れる。
初期がGitで管理されている。
cargo build
↑ 実行ファイルの作成を行う。
./target/debug/package_name
↑ 実行する。
cargo run --bin [file]
↑ コンパイルして、実行まで行う。
cargo check --bin [file]
↑ コンパイルが行えるかを確認する。
cargo build --release
↑ プロジェクトが完成した場合、コマンドを実行してリリースする。
Cargo.toml = 設定ファイル
Cargo.lock = ビルド時の全ライブラリのバージョンが記載されている。(真偽は不明)(勉強中)