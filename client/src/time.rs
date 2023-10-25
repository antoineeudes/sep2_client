//! Time Function Set

use std::{
    sync::atomic::AtomicI64,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use sep2_common::packages::{primitives::Int64, time::Time};

static TIME_OFFSET: AtomicI64 = AtomicI64::new(0);

/// Return the current time, as an Int64
pub fn current_time() -> Int64 {
    let current_time = SystemTime::now();
    let duration = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    Int64(duration.as_secs() as i64)
}

/// Return the current time, as an Int64, with the global time offset supplied.
pub fn current_time_with_offset() -> Int64 {
    let current_time = SystemTime::now();
    let duration = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    Int64(duration.as_secs() as i64 + TIME_OFFSET.load(std::sync::atomic::Ordering::Relaxed))
}

/// Given a Time resource, calculate it's offset from the system time,
/// and set that offset to be applied to all future calls to [`current_time_with_offset`]
pub fn update_time_offset(time: Time) {
    let offset = time.current_time.get() - current_time().get();
    TIME_OFFSET.store(offset, std::sync::atomic::Ordering::Relaxed);
}

/// Intermittently sleep until the provided instant,
/// waking at an interval defined by `rate`.
/// This uses `tokio::time:sleep`, which, like `thread::sleep` does not make progress while the device itself is asleep,
/// hence the intermittent wakeups.
pub async fn sleep_until(timestamp: Instant, tickrate: Duration) {
    loop {
        tokio::time::sleep(tickrate).await;
        if Instant::now() > timestamp {
            break;
        }
    }
}
