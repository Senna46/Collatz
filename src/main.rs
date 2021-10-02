// main.rs, lib.rsがメインファイルなので参照するファイルを指定
mod rubens;

// ファイルの中身を使う
use rubens::hoge;
fn main() {
    println!("Hello, world!");
    let start = 1145148101919u64;
    // mut 変更可能な変数を作成
    // u64は自動コピーで使用，ない場合はclone()して使用
    let mut value = start;
    // 参照を作成
    // let start_ref = &start;
    loop {
        value = f(value);
        println!("{}", value);
        if value == 1 {
            break;
        }
    }

    let success = hoge(13, |v| v - 1);
    println!("success?: {}", success);
}

fn f(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}
