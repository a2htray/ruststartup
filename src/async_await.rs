use core::time;
use futures::{executor::block_on, future::BoxFuture};
use std::thread::sleep;
use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

// 异步函数返回一个 Future
async fn do_something() {
    println!("go async");
}

// 异步函数中调用另一个异步函数
async fn do_something2() {
    do_something().await;
    println!("go async 2");
}

pub fn run_future() {
    let future = do_something();
    block_on(future);
}

pub fn run_future2() {
    println!("run future 2");
    let future = do_something2();
    block_on(future);
}

// 定义两个交替运行的异步函数
async fn print_gt_100() {
    for i in 101..=105 {
        println!("print_gt_100 => {}", i);
        sleep(time::Duration::from_secs(1));
    }
}

async fn print_lt_100() {
    for i in 1..=5 {
        println!("print_lt_100 => {}", i);
        sleep(time::Duration::from_secs(1));
    }
}

async fn main_print() {
    let print1 = print_gt_100();
    let print2 = print_lt_100();

    futures::join!(print1, print2);
}

pub fn run_future3() {
    block_on(main_print())
}

// Future 定时器
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

pub struct TimeFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for TimeFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimeFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimeFuture { shared_state }
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUE_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUE_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}
