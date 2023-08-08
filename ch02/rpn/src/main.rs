fn main() {
    // exp変数をRPN形式の文字列に束縛する
    // このRPNは数式 6.1 + 5.2 x 4.3 - 3.4 / 2.5 x 1.6 と等しい
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    // rpn関数を呼び出して計算する。返された値にans変数を束縛する
    let ans = rpn(exp);

    // デバッグビルド時のみ、答えが正しいかチェックする
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    // expとansの値を表示する。ansは小数点以下4桁まで表示する
    println!("{} = {:.4}", exp, ans);
}

fn rpn(exp: &str) -> f64 {
    // 変数stackを空のスタックに束縛する
    // stackはミュータブル(mutable、可変)な変数で、値の更新を許す
    let mut stack = Vec::new();

    // expの要素をスペースで分割し、tokenをそれらに順に束縛する
    // 要素がなくなるまで繰り返す
    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            // tokenがf64の数値だった場合、stackに積む
            stack.push(num);
        } else {
            // tokenが数値でない場合、演算子なのか調べる
            match token {
                // tokenが演算子だった場合、apply2関数で計算する
                // |x, y| x + y はクロージャ。詳しくは後述する
                // 引数x, yを取り、x + yを計算して答えを返す
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                // tokenが演算子でないなら、エラーを起こして終了する
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    // スタックから数値を一つ取り出す。失敗したらエラーを起こして終了する
    stack.pop().expect("Stack underflow")
}

// スタックか数値を2つ取り出して、F型のクロージャーfunで計算し、結果をスタックに積む
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    // F型のトレイト境界。本文参照
    F: Fn(f64, f64) -> f64,
{
    // 変数yとxをstackの最後の2要素に束縛する
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        // クロージャfunで計算し、その結果に変数zを束縛する
        let z = fun(x, y);
        // 変数zの値をstackに積む
        stack.push(z);
    } else {
        // スタックから要素が取り出せなかった場合はエラーを起こして終了する
        panic!("Stack underflow");
    }
}
