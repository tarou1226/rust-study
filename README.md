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
Cargo.lock = バージョンを参照するためのファイル (npmでいうところのpackage-lock.json)  
mutable(可変)とimmutable(不可変)がある。  
mutであっても、型は変更できない。
型変更をするためには、シャドーイング（被せて宣言）を行う。  