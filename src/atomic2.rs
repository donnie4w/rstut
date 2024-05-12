use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

struct ThreadSafeData {
    // 使用Mutex来保护字符串，使其线程安全
    message: Mutex<String>,
    // 使用AtomicUsize来实现线程安全的计数器
    counter: Arc<AtomicUsize>,
    ptr: Arc<AtomicPtr<String>>,
}

impl ThreadSafeData {
    fn new(message: String) -> ThreadSafeData {
        ThreadSafeData {
            message: Mutex::new(message),
            counter: Arc::new(AtomicUsize::new(0)),
            ptr: Arc::new(AtomicPtr::new(Box::into_raw(Box::new(
                "AtomicPtr>>>".to_string(),
            )))),
        }
    }

    // 增加计数的方法，使用原子操作
    fn increment_counter(&self) {
        self.counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    // 修改消息的方法，使用Mutex保护
    fn update_message(&self, new_message: &str) {
        let mut msg = self.message.lock().unwrap();
        *msg = new_message.to_string();
    }

    // 获取消息的方法，同样使用Mutex保护
    fn get_message(&self) -> String {
        self.message.lock().unwrap().clone()
    }

    // 获取计数的方法，直接读取原子变量
    fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }

    fn get_pre(&self) -> String {
        let ptr = self.ptr.load(Ordering::Acquire);
        if ptr.is_null() {
            String::new()
        } else {
            unsafe { (*ptr).clone() }
        }
    }
}

#[test]
fn testmain() {
    let shared_data = Arc::new(ThreadSafeData::new("Initial Message".to_string()));

    // 创建并运行线程来更新计数和消息
    let threads: Vec<_> = (0..10)
        .map(|_| {
            let data_ref = shared_data.clone();
            thread::spawn(move || {
                for _ in 0..100 {
                    data_ref.increment_counter();
                }
                data_ref.update_message("Updated by thread");
            })
        })
        .collect();

    // 等待所有线程完成
    for t in threads {
        t.join().unwrap();
    }

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

    println!("Final Counter: {}", shared_data.get_counter());
    println!("Final get_pre: {}", shared_data.get_pre());
    println!("Final Message: {:?}", shared_data.get_message());
}
