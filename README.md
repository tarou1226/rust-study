# Rustを勉強するリポジトリ
基本的にCargoというビルドシステムで管理を行う。<br>
cargo new filename<br>
↑ プロジェクトの作成を行う。<br>
--vcs こちらのフラグを与えると、バージョン管理システムなどを弄れる。<br>
初期がGitで管理されている。<br>
cargo build<br>
↑ 実行ファイルの作成を行う。<br>
./target/debug/package_name<br>
↑ 実行する。<br>
cargo run --bin [file]<br>
↑ コンパイルして、実行まで行う。<br>
cargo check --bin [file]<br>
↑ コンパイルが行えるかを確認する。<br>
cargo build --release<br>
↑ プロジェクトが完成した場合、コマンドを実行してリリースする。<br>
Cargo.toml = 設定ファイル<br>
Cargo.lock = ビルド時の全ライブラリのバージョンが記載されている。(真偽は不明)(勉強中)<br>