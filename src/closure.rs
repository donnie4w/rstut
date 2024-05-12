#[test]
fn test_fn() {
    // 定义一个变量，供闭包借用
    let num = 5;

    // 定义一个Fn类型的闭包，它不可变地借用num
    let add_num = |x: i32| num + x; // 闭包体内的num是通过引用访问的，因此闭包为Fn类型

    // 调用闭包两次，验证它可以多次借用不可变地访问num
    println!("add_num with 10: {}", add_num(10)); // 输出 15
    println!("add_num with 20: {}", add_num(20)); // 输出 25
}

#[test]
fn test_fn_mut() {
    // 定义一个可变变量，供闭包借用和修改
    let mut counter = 0;

    // 定义一个FnMut类型的闭包，它可以可变地借用counter
    let mut increment_counter = || {
        counter += 1; // 闭包体内部修改了counter，因此是FnMut类型
    };

    // 调用闭包两次，每次调用都会增加counter的值
    increment_counter();
    increment_counter();
    println!("Counter after incrementing: {}", counter); // 输出 2
}

#[test]
fn test_fn_once() {
    // 定义一个变量，将被闭包移动
    let message = "Hello, World!".to_string();

    // 定义一个FnOnce类型的闭包，它通过move关键字获取message的所有权
    let print_message = move || {
        println!("{}", message); // 闭包体内部移动了message，因此是FnOnce类型
    };

    // 调用闭包一次，因为message的所有权已被转移
    print_message(); // 输出 Hello, World!

    // 注意：在此之后，不能再访问message，因为它已经被移动到了闭包内部
    // println!("{}", message); // 这里会报错，因为message已经不存在于作用域中
}
