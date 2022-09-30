fn main() {
    let x = 5;
    println!("{}", x);

    {
        let x = x * 3;
        println!("{}", x)
    }

    let x = x + 1;
    println!("{}", x);

    let spaces = "   ";
    println!("{}", spaces);

    let spaces = spaces.len();
    println!("{}", spaces);
    // 定数はconstで書く。
    // 定数の場合は、型を書かなければならない。
    // 定数名は大文字アンダーバーが推奨されている。
    const MAX_FIGURE: i32 = 100;
    println!("{}", MAX_FIGURE);

    /*
    let mut spaces = "   ";
    spaces = spaces.len();
    ↑これはエラーが吐かれる。
    理由は、型が変換されているから。
    */
}