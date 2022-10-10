fn main() {
    chap2_1();
    chap2_2();
}

// 変数とデータ型
fn chap2_1() {
    // 変数、束縛、代入
    let message;
    if true {
        message = "true type";
    } else {
        message = "false type";
    }
    println!("{}", message);

    // 文字型
    let c: char = 'c';
    println!("{}", c);

    // タプル
    let tuple = ("morino", 16);
    println!("{}", tuple.0);
    println!("{}", tuple.1);

    // ベクター(可変長配列)
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // HashMap
    let mut scores = std::collections::HashMap::new();
    scores.insert("morino", 100);
    scores.insert("arisugawa", 200);
    scores.entry("morino").or_insert(150);
    println!("{:?}",scores);
}

// 関数の実装
fn chap2_2(){

}