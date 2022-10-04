use std::io;
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // タプルの扱い方
    let tup = (32, 3.2, "Hello");
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    println!("{}", tup.0);

    let a = [1, 2, 3, 4, 5];

    //let first = a[0];
    //let second = a[1];
    //println!("{}, {}", first, second);

    println!("Please enter an array index.");
    // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
              // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
        // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );
}