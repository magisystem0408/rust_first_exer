// 親でも使いたい場合はpubをつける必要がある
pub mod sub_a;
pub mod sub_b;

// 定数はconstで型を指定するときはコロンの後ろに書く。
const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("hello mamushi");
    sub_a::func_a();
    sub_b::func_b();

    // 値を変更したい時には、letの後にmut(mutable)をつけて行う
    // letは関数のスコープでしか定義できない
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);

    // 使っていない変数はアンダーバーをつける
    let _i1 = 3;
    let _f1 = 0.1;
    println!("{}", usize::BITS);

    // 変数が格納されているアドレスを表示する
    println!("Memory address of const is : {:p}", &MAX_POINTS);

    // 8バイト分のデータ
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // デフォルトは32bit
    let y = 5;
    println!("Stack address of y is:{:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);

    {
        // このローカルの値では0になるが
        // 外ではyの値は6で保たれる
        let y = 0;
        println!("the valiue of y is :{}", y);
    }

    println!("The value of y is: {}", y);

    let t1 = (500, 6.4, "dummy");

    // オブジェクトのようにアクセスする
    println!("The value if t1 is :{}{}{}", t1.0, t1.1, t1.2);


    // 分割で受け取ることも可能
    let (_x, _y, _z) = t1;

    // タプルを変更したい場合はmut
    let mut t2 = ((0, 1), (2, 3));

    // 分割するにはrefを使用する。
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;

    // ポインタが指す部分にアクセスできるようになる。
    // t2のタプルを書き換えることができる
    *x1_ptr = 5;
    *y1_ptr = -5;
    // タブルとか構造体など複雑なものを出力するためには:?マークを書く必要がある。
    println!("{:?}", t2);

    // 配列の定義
    let a1 = [1, 2, 3, 4, 5, 6];
    // 全て0で10こ作ってくれる
    let a2 = [0; 10];
    println!("{:?}{:?}{}{}", a1, a2, a1[1], a1[3]);

}