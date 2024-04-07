
## 构建一个定时器

在创建计时器时创建新线程，休眠特定时间，然后过了时间窗口时通知（signal） 计时器 future

Cargo.toml

```shell
[dependencies]
internal = "0.3"
```

main.rs

```rust
use futures;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future和等待的线程间共享状态
struct SharedState {
    /// 定时(睡眠)是否结束
    completed: bool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 通过检查共享状态，来确定定时器是否已经完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            println!("future ready. execute poll to return.");
            Poll::Ready(())
        } else {
            println!("future not ready, tell the future task how to wakeup to executor");
            // 设置`waker`，这样新线程在睡眠(计时)结束后可以唤醒当前的任务，接着再次对`Future`进行`poll`操作,
            // 下面的`clone`每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更加合理。
            // 选择每次都`clone`的原因是： `TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
            // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 创建新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            // 睡眠指定时间实现计时功能
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                println!("detect future is ready, wakeup the future task to executor.");
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

fn main() {
    // 我们现在还没有实现调度器，所以要用一下futues库里的一个调度器。
    futures::executor::block_on(TimerFuture::new(Duration::new(10, 0)));    
}
```

该demo函数的执行过程如下。

1.最开始执行器会先 poll 一次 Future，此时shared_state.completed为false，future返回Poll::Pending，执行器会挂起该future，并等待waker被唤醒。
2.当定时器线程结束后，会调用waker.wake()来唤醒执行器，执行器就会把它们放入队列并再一次 poll，此时shared_state.completed为true，future返回Poll::Ready(())，执行器会继续执行下一个任务。