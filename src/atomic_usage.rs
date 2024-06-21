use std::{
    sync::atomic::{AtomicU8, Ordering},
    thread,
};

static START: AtomicU8 = AtomicU8::new(0);

pub fn test_start_atomicu8() {
    // 创建 8 个线程对 START 值进行修改

    let mut threads = Vec::with_capacity(8);
    for _ in 0..=threads.capacity() - 1 {
        threads.push(thread::spawn(move || {
            println!("thread id: {:?} -> START = {}", thread::current().id(), START.load(Ordering::Relaxed));
            START.fetch_add(1, Ordering::Relaxed);
            println!("thread id: {:?} -> START = {}", thread::current().id(), START.load(Ordering::Relaxed));
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("{:?}", START);
}
