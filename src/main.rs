fn main() {
    println!("Hello, world!");
    test()
}

fn test() {
    let value1 = String::from("hello");
    let value2 = value1;
    let value3 = value2.clone(); // cloneはメモリ上の別のアドレスに値のコピーを作り出す
    // プリミティブのようにヒープメモリ上に展開されない値はコピーされる
    // ヒープメモリ上に展開される値は所有権が移動する（ムーブになる）
    // 言語実装としてはCopyトレイトが実装されているか否かの差
    // println!("{}", value1);
    println!("{}", &value2);

    // 関数の引数に通常通り引数を渡すと所有権が移転する
    test2(value3);
    // 参照を引数に受け取り借用することで元の変数の所有権は放棄されなくなる
    // 参照と借用のルール
    // ・可変参照は一度に一つ（レースコンディションの防御）
    // ・不変参照と可変参照は混在できない（読み込み時の予期せぬ変更の防御）
    // ・参照は常に有効（nullポのような状況の排除）
    test3(&value2);
}

fn test2(src: String) {
    println!("{}", src);
}

fn test3(src: &String) {
    println!("{}", src);
}

// 参照には有効なライフタイムが存在する
// 明示的に指定する必要があることも出てくるかも・・・→ライフタイム注釈
// 特に関数の戻り値として返す値をどこでどう使うかあたりで意識が必要になりそう
fn test4(x: &String, y: &String) -> String  {
    if x.len() > y.len() {
        x.to_owned()
    } else {
        y.to_owned()
    }
}