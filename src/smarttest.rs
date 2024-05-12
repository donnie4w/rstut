use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;


// 定义一个简单的结构体，用于展示指针用法
#[derive(Debug)]
struct SimpleStruct {
    value: i32,
}

// 使用Box<T>在堆上分配SimpleStruct实例，并通过引用访问其内容
#[test]
fn use_box() {
    // 在堆上分配 SimpleStruct 实例
    let boxed_struct = Box::new(SimpleStruct { value: 42 });

    // 通过引用访问 Box 中的数据
    // 这里，&* 操作符用于从 Box 指针获取引用
    let ref_to_struct = &*boxed_struct;
    println!("Ref to struct value: {}", ref_to_struct.value);

    // 传递引用给函数，展示如何不转移所有权而访问数据
    display_value_by_reference(&ref_to_struct.value);
    
    // 直接传递 Box<T> 给接受引用的函数也是可行的，因为 Rust 允许自动 dereference
    display_value_by_box(boxed_struct);
}

// 接受 SimpleStruct 字段的引用作为参数的函数
fn display_value_by_reference(value: &i32) {
    println!("Displaying value by reference: {}", value);
}

// 通过自动 dereference，此函数也能接受 Box<T> 作为参数
fn display_value_by_box(ss: Box<SimpleStruct>) {
    println!("Displaying box content: {:?}", ss);
}

#[derive(Debug)]
struct SimpleStruct2 {
    value: i32,
    // 假设我们想让 SimpleStruct 的某个字段在 Rc 环境下仍可变
    mutable_value: RefCell<i32>,
}
// 使用Rc<T>实现非线程安全的共享所有权
#[test]
fn use_rc() {
    let shared_struct = Rc::new(SimpleStruct2 {
        value: 100,
        mutable_value: RefCell::new(200),
    }); // 创建一个引用计数的共享指针

    // 克隆引用，增加引用计数
    let another_ref = Rc::clone(&shared_struct);

    // 使用 deref coercion 自动解引用打印
    println!("Rc shared struct: {:?}", shared_struct);

    // 显示引用计数
    println!("Reference count: {}", Rc::strong_count(&shared_struct));

    // 修改内部可变字段的值，使用RefCell
    let mut mutable_borrow = shared_struct.mutable_value.borrow_mut();
    *mutable_borrow += 50;
    println!("Modified mutable value: {}", *mutable_borrow);

    // 当 another_ref 超出作用域时，引用计数减1
}

/********************************************************************************************************** */

// 使用Arc<T>实现线程安全的共享所有权
#[test]
fn use_arc() {
    let shared_arc = Arc::new(SimpleStruct { value: 200 }); // 创建一个线程安全的引用计数指针

    // 在一个新的作用域中克隆 `Arc`，以展示生命周期管理
    {
        let another_ref = Arc::clone(&shared_arc);
        println!("Arc shared struct in inner scope: {:?}", another_ref);
    } // 此作用域结束，`another_ref` 离开作用域，引用计数减少

    println!("Arc shared struct (after inner scope): {:?}", shared_arc);

    // 展示线程间共享
    let thread_handle = {
        let shared_for_thread = Arc::clone(&shared_arc);
        thread::spawn(move || {
            println!("Thread sees Arc shared struct: {:?}", shared_for_thread);
        })
    };

    thread_handle.join().unwrap(); // 等待线程完成
    println!("======end======")
    // 主线程继续执行，直到结束，此时所有克隆的 `Arc` 都离开了作用域，内存自动释放
}

// 使用RefCell<T>实现内部可变性
#[test]
fn use_ref_cell() {
    // 使用 RefCell 创建一个可变的 SimpleStruct 实例，允许内部可变性
    let ref_cell_struct = RefCell::new(SimpleStruct { value: 300 });

    // 获取可变借用，用于修改 SimpleStruct 的内部值
    {
        // `borrow_mut` 提供了内部可变性，允许修改 `value`
        let mut borrowed = ref_cell_struct.borrow_mut();
        borrowed.value *= 2; // 修改 `value` 字段
    } // `borrow_mut` 的作用域结束，释放可变借用

    // 使用 `borrow` 获取不可变借用，以显示修改后的结构体内容
    let immutable_borrow = ref_cell_struct.borrow();
    println!(
        "RefCell struct after modification: {:?}",
        immutable_borrow
    );
}

// 使用Mutex<T>实现线程安全的内部可变性
#[test]
fn use_mutex() {
    let mutex_struct = Mutex::new(SimpleStruct { value: 400 }); // 使用Mutex包裹结构体，提供线程安全的内部可变性
    {
        let mut locked_struct = mutex_struct.lock().unwrap(); // 加锁并获取可变引用
        locked_struct.value += 100; // 在锁保护下修改值
    }
    println!(
        "Mutex struct after modification: {:?}",
        mutex_struct.lock().unwrap()
    );
}

// 使用裸指针(*const T 和 *mut T)，通常较少直接使用，更常见于FFI或低级编程
#[test]
fn use_raw_pointers() {
    let simple_struct = SimpleStruct { value: 500 };
    let raw_ptr: *const SimpleStruct = &simple_struct as *const _; // 获取不可变裸指针
    let mut simple_struct_mut = SimpleStruct { value: 600 };
    let mut_ptr: *mut SimpleStruct = &mut simple_struct_mut as *mut _; // 获取可变裸指针
                                                                       // 注意：裸指针的使用需要极其小心，以避免悬挂指针和数据竞争
    unsafe {
        assert_eq!((*raw_ptr).value, 500); // 通过裸指针访问数据
        (*mut_ptr).value = 700; // 修改通过可变裸指针指向的数据
    }
    println!("Mutated value via raw pointer: {}", simple_struct_mut.value);
}


#[test]
fn test_square() {
    // 定义一个数字列表
    let mut numbers = vec![1, 2, 3, 4, 5];

    // 定义一个匿名闭包，用于计算平方
    let square = |x: i32| -> i32 { x * x }; // 注意这里类型标注可以省略，Rust会自动推导

    // 调用apply_to_each函数，将数字列表和计算平方的闭包作为参数
    apply_to_each(numbers.clone(), square);

    // 如果你喜欢使用更简洁的语法，Rust也允许这样做
    apply_to_each(numbers, |x| x * x);
}

fn apply_to_each(numbers: Vec<i32>, operation: impl Fn(i32) -> i32) {
    // 遍历数字列表，并对每个数字应用operation闭包
    for num in numbers.iter() {
        println!("原数: {}, 平方后: {}", num, operation(*num));
    }
}