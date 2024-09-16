use std::ptr;
use std::alloc::{alloc, Layout};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    // حجم حافظه مورد نیاز به بایت (مثلاً 5 گیگابایت)
    let size_in_gb = 13;
    let size_in_bytes = size_in_gb * 1024 * 1024 * 1024;

    // ایجاد طرح حافظه برای تخصیص
    let layout = Layout::from_size_align(size_in_bytes, 8).unwrap();

    // تخصیص حافظه به صورت Unsafe
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        panic!("Failed to allocate memory");
    }

    // پر کردن حافظه با صفر
    unsafe {
        ptr::write_bytes(ptr, 0, size_in_bytes);
    }

    println!("Memory allocated and filled. The program will now keep the memory for 30 seconds...");

    // نگه‌داشتن برنامه به مدت 30 ثانیه
    sleep(Duration::new(30, 0));

    // آزادسازی حافظه
    unsafe {
        std::alloc::dealloc(ptr, layout);
    }

    println!("Memory deallocated after 30 seconds.");
}
