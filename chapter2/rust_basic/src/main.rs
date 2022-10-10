fn main() {
    chap2_1();
    chap2_2();
    chap2_3();
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
    fn add(x: i32, y: i32) -> i32{
        return x + y;
    }
    let sum = add(1,2);
    println!("{}", sum);

    // if式
    let a = if true{
        10
    } else{
        20
    };
    println!("{}", a);   

    let some: Result<&str, &str> = Ok("ok");
    println!("{:?}", some);
    let err: Result<&str, &str> = Err("err");
    println!("{:?}", err);

    // ?演算子
    fn always_error() -> Result<(), String>{
        Err("常にエラーだよ".to_string())
    }

    fn might_fail() -> Result<(), String> {
        let _reault = always_error()?;
        // alwats_error()? で早期リターンされるのでOkまで進まない
        Ok(())
    }
    
    let message = match might_fail() {
        Ok(_) => "処理に成功".to_string(),
        Err(cause_message) => cause_message,
    };
    println!("{}", message);
}

fn chap2_3(){

}