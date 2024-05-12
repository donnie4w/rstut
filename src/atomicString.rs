use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct AtomicString {
    version: Arc<AtomicUsize>,
    ptr: Arc<AtomicPtr<String>>,
}

impl AtomicString {
    fn new(s: String) -> Self {
        AtomicString {
            ptr: Arc::new(AtomicPtr::new(Box::into_raw(Box::new(s)))),
            version: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn update(&self, new_val: String) {
        let mut spin_count = 0;
        let mut yield_count = 0;
        let new_ptr = Box::into_raw(Box::new(new_val));
        loop {
            let current_ptr = self.ptr.load(Ordering::Acquire);
            let current_version = self.version.load(Ordering::Acquire);
            let new_version = current_version + 1;

            if self
                .ptr
                .compare_exchange_weak(current_ptr, new_ptr, Ordering::Release, Ordering::Relaxed)
                .is_ok()
                && self
                    .version
                    .compare_exchange_weak(
                        current_version,
                        new_version,
                        Ordering::Release,
                        Ordering::Relaxed,
                    )
                    .is_ok()
            {
                unsafe {
                    if !current_ptr.is_null() {
                        drop(Box::from_raw(current_ptr));
                    }
                }
                break;
            } else {
                spin_count += 1;
                if spin_count >= 30 {
                    spin_count = 0;
                    yield_count += 1;
                    thread::yield_now();
                    if yield_count >= 15 {
                        thread::sleep(Duration::from_millis(1));
                        yield_count = 0;
                    }
                }
            }
        }
    }

    fn get(&self) -> String {
        let ptr = self.ptr.load(Ordering::Acquire);
        if ptr.is_null() {
            String::new()
        } else {
            unsafe { (*ptr).clone() }
        }
    }
}

impl Drop for AtomicString {
    fn drop(&mut self) {
        let ptr = self.ptr.load(Ordering::Relaxed);
        unsafe {
            if !ptr.is_null() {
                drop(Box::from_raw(ptr));
            }
        }
    }
}

#[test]
fn testvs() {
    let vstring = Arc::new(AtomicString::new("Initial Message".to_string()));

    let mut handles = vec![];
    for i in 0..100 {
        let str_clone = vstring.clone();
        let handle = thread::spawn(move || {
            let new_value = format!("Value updated {}", i);
            println!("Updating to: {}", new_value);
            str_clone.update(new_value);
            // thread::sleep(Duration::from_millis(50)); // 模拟一些工作
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final get: {}", vstring.get());
}
