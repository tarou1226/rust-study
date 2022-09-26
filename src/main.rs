use std::io; //pythonでいうところのimport

fn main() {
    println!("数字当てゲーム！");

    println!("数字を入力してください。");
    // mut→可変(変数)　何も無し→不可変(定数)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("読み込みに失敗しました");
    // {}はformat文
    println!("あなたの数字: {}", guess);
}
