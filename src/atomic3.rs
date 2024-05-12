use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;

struct SharedString {
    ptr: AtomicPtr<String>,
}

impl SharedString {
    //创建一个AtomicPtr 指针对象
    fn new(s: String) -> Self {
        SharedString {
            ptr: AtomicPtr::new(Box::into_raw(Box::new(s))),
            //使用Box::new将这个String实例装箱。装箱操作会将值放置在堆上，并返回一个指向该值的智能指针（Box）。
            //这样做的原因是String作为复杂类型，其大小不固定，不能直接存放在栈上作为原生指针的目标。
            //装箱还负责String的生命周期管理，当Box离开作用域时，它会自动清理堆上的内存
            //然后，使用Box::into_raw方法将Box<String>转换为一个原始指针（*mut String）。
            //这一步实际上剥离了智能指针的自动内存管理特性，将控制权交给了调用者，意味着你现在需要手动管理这个指针的生命周期，包括分配和释放
        }
    }

    fn update(&self, new_val: String) {
        let new_ptr = Box::into_raw(Box::new(new_val));
        loop {  //自旋：自旋在预期锁被持有的时间很短的情况下可以减少上下文切换的开销，但如果锁被长时间持有，自旋会浪费CPU周期，导致忙等待，影响整体性能，并可能增加系统功耗
            let current_ptr = self.ptr.load(Ordering::Acquire);
            // 尝试原子性地更新指针，这里简化处理，实际应确保旧值仍然有效
            if self
                .ptr
                .compare_exchange_weak(current_ptr, new_ptr, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                // 如果成功，安全地删除旧字符串
                unsafe {
                    drop(Box::from_raw(current_ptr));
                }
                break;
            } 
        }
    }

    fn get(&self) -> String {
        let ptr = self.ptr.load(Ordering::Acquire); //使用load方法以适当的内存顺序从AtomicPtr中安全地加载原始指针
        if !ptr.is_null() {
            //要在unsafe代码块中正确解引用并转换为String，需要克隆这个String的值
            unsafe { (*ptr).clone() }
        } else {
            String::new()
        }
    }
}

#[test]
fn testmain() {
    let shared_str = Arc::new(SharedString::new("Initial Value".to_string()));
    let mut handles = vec![];
    for i in 0..50 {
        // 直接在这里克隆，确保每次迭代都有独立的Arc实例
        let shared_str_clone = shared_str.clone();
        let update_thread = thread::spawn(move || {
            let new_val = format!("Value-{}", i);
            println!("{}", new_val);
            shared_str_clone.update(new_val);
            thread::sleep(std::time::Duration::from_millis(100));
        });
        handles.push(update_thread);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Value: {}", shared_str.get());
}
