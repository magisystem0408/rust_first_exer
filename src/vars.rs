// 親でも使いたい場合はpubをつける必要がある
pub mod sub_a;
pub mod sub_b;

pub fn run(){
    println!("hello mamushi");
    sub_a::func_a();
    sub_b::func_b();
}