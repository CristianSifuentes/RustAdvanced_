//! Async module: custom mini executor + async pipeline + cancellation token style.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;
use std::time::Duration;

/// Step 9: tiny cancellation token using atomics.
#[derive(Clone, Default)]
pub struct CancelToken {
    stop: Arc<AtomicBool>,
}

impl CancelToken {
    pub fn cancel(&self) {
        self.stop.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}

/// Step 10: custom `block_on` executor to show low-level async mechanics.
pub fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
    let mut cx = Context::from_waker(&waker);
    // SAFETY: future is never moved after pinning.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => thread::sleep(Duration::from_millis(1)),
        }
    }
}

/// Step 11: async task with bounded channel for simple backpressure.
pub async fn run_pipeline(values: Vec<f64>, token: CancelToken) -> Vec<f64> {
    let (tx, rx) = mpsc::sync_channel::<f64>(4);
    let producer = thread::spawn(move || {
        for value in values {
            if token.is_cancelled() {
                break;
            }
            if tx.send(value * value).is_err() {
                break;
            }
        }
    });

    let consumer = thread::spawn(move || {
        let mut output = Vec::new();
        while let Ok(value) = rx.recv_timeout(Duration::from_millis(10)) {
            output.push(value);
        }
        output
    });

    let _ = producer.join();
    consumer.join().unwrap_or_default()
}

const NOOP_RAW_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(noop_clone, noop_wake, noop_wake_by_ref, noop_drop);

const fn noop_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &NOOP_RAW_WAKER_VTABLE)
}

unsafe fn noop_clone(_: *const ()) -> RawWaker {
    noop_raw_waker()
}

unsafe fn noop_wake(_: *const ()) {}

unsafe fn noop_wake_by_ref(_: *const ()) {}

unsafe fn noop_drop(_: *const ()) {}
