use std::fmt::Display;

// 定义一个泛型函数 `max`，用于比较两个同类型值并返回较大的那个
// T 表示类型参数，它可以是任何类型，只要该类型实现了 PartialOrd（部分排序）trait
fn max<T: PartialOrd>(x: T, y: T) -> T {
    // 使用 if let 作为简洁的三元表达式，这里利用了 PartialOrd trait 的方法 `ge`
    if x.ge(&y) {
        x
    } else {
        y
    }
}

#[test]
fn test_func() {
    // 使用泛型函数 `max` 比较两个整数
    println!("Max of 10 and 20 is {}", max(10, 20)); // 输出 20

    // 使用泛型函数 `max` 比较两个字符串（在 Rust 中，字符串字面量是 &str 类型）
    println!("Max of 'apple' and 'banana' is {}", max("apple", "banana")); // 输出 'banana'
}

// 定义一个泛型结构体 `Pair`，它包含两个相同类型的值
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    // 为 Pair 结构体定义一个新方法 `new`，用于构造 Pair 实例
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    // 定义一个方法 `cmp_first_to_second`，比较 first 和 second 的大小
    fn cmp_first_to_second(&self) -> bool
    where
        T: PartialOrd, // 添加 trait 约束，要求 T 必须实现了 PartialOrd trait
    {
        self.first.ge(&self.second)
    }
}

#[test]
fn test_struct() {
    // 创建一个 Pair 实例，类型为 i32
    let pair_int = Pair::new(1, 2);
    println!(
        "First greater than second? {}",
        pair_int.cmp_first_to_second()
    ); // 输出 false

    // 创建一个 Pair 实例，类型为 &str
    let pair_str = Pair::new("Tom", "Jerry");
    println!(
        "First greater than second? {}",
        pair_str.cmp_first_to_second()
    ); // 输出 false
}

// 定义一个泛型 trait `Summary`，它有一个方法 `summarize`
trait Summary<T> {
    fn summarize(&self) -> T;
}

// 为 String 类型实现 `Summary` trait，返回其长度作为摘要
impl Summary<usize> for String {
    fn summarize(&self) -> usize {
        self.len()
    }
}

// 为自定义类型实现 `Summary` trait，这里假设我们有某种数据结构需要摘要
struct NewsArticle {
    headline: String,
    content: String,
}

// 实现 `Summary` trait，返回文章的标题作为摘要
impl Summary<String> for NewsArticle {
    fn summarize(&self) -> String {
        self.headline.clone()
    }
}

#[test]
fn test_trait() {
    let article = NewsArticle {
        headline: String::from("New Study Shows Rust Usage Soars"),
        content: String::from(
            "A recent study found that the use of Rust in system programming has skyrocketed.",
        ),
    };

    println!("Article Summary: {}", article.summarize()); // 输出文章标题
    println!("Length of an empty string: {}", String::new().summarize()); // 输出空字符串长度
}

// 自定义一个泛型枚举，表示数学运算的结果
#[derive(Debug)]
enum MathResult<T> {
    Value(T),
    Error(String),
}

#[test]
fn test_enum() {
    // 使用泛型枚举
    let sum_result: MathResult<i32> = MathResult::Value(5 + 3);
    println!("{:?}", sum_result); // 打印：Value(8)

    let divisor = 2.0;
    let numerator = 6.0;
    let divide_result = if divisor != 0.0 {
        MathResult::Value(numerator / divisor)
    } else {
        MathResult::Error("Division by zero".to_string())
    };

    println!("{:?}", divide_result); // 假设正确处理了除零情况，会打印：Value(结果) 或 Error("Division by zero")

    // 错误示例
    let error_result: MathResult<i32> = MathResult::Error("Invalid input".to_string());
    println!("{:?}", error_result); // 打印：Error("Invalid input")
}

struct PairTs<T>
where
    T: Display + Clone,
{
    first: T,
    second: T,
}

impl<T> PairTs<T>
where
    T: Display + Clone,
{
    fn new(first: T, second: T) -> Self {
        PairTs { first, second }
    }

    fn display(&self) {
        println!("First: {}, Second: {}", self.first, self.second);
    }
}

#[test]
fn test_where() {
    let pair = PairTs::new("Rust", "2023");
    pair.display();
}
