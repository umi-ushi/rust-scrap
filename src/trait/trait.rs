// Trait 複数の型で共通の機能を提供する仕組み
// traitもしくは構造体のいずれかが自身のクレーと内で定義されていれば定義可能

pub trait Calc {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
}

impl Calc for i32 {
    fn add(self, other: Self) -> Self {
        self + other
    }
    fn sub(self, other: Self) -> Self {
        self - other
    }
}

// 動的ディスパッチ
fn sample() {
    let objects: Vec<Box<dyn ToString>> = vec![
        Box::new(MyStruct1{ f: "sample1"}),
        Box::new(MyStruct2{ f: "sample2"}),
    ];
}

#[derive(Debug, Default)]
pub struct MyStruct1 {
    pub f: String,
}

#[derive(Debug, Default)]
pub struct MyStruct2 {
    pub f: String,
}