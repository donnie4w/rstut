use std::sync::atomic::{AtomicUsize, Ordering,AtomicPtr};
use std::sync::Arc;
use std::thread;

#[derive(Clone, Debug)]
struct SharedData {
    // 使用AtomicUsize实现线程安全的计数器
    counter: Arc<AtomicUsize>,
    // 通过Arc共享不可变的字符串
    message: Arc<String>,
}

impl SharedData {
    fn new(message: &str) -> Self {
        SharedData {
            counter: Arc::new(AtomicUsize::new(0)),
            message: Arc::from(message.to_owned()),
        }
    }

    // 原子地增加计数器的值
    fn increment_counter(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }

    // 获取当前计数器的值
    fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }

    // 安全地获取共享的不可变消息
    fn get_message(&self) -> &str {
        &self.message
    }
}

#[test]
fn testmain() {
    // 创建共享数据实例
    let shared_data = SharedData::new("Initial Message");

    // 创建并运行线程来并发增加计数器的值
    let mut handles = vec![];
    for _ in 0..10 {
        let data_ref = shared_data.clone();
        let handle = thread::spawn(move || {
            // 增加计数器
            for _ in 0..100 {
                data_ref.increment_counter();
            }
            // 可以安全地读取但不可修改消息
            println!("Thread sees message: {}", data_ref.get_message());
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出最终结果
    println!("Final Counter: {}", shared_data.get_counter());
    println!("Shared Message: {}", shared_data.get_message());
}