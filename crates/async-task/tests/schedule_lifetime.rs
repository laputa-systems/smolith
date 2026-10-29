use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use async_task::{Runnable, ScheduleParts};

#[test]
fn dropping_the_runnable_inside_schedule_still_runs_the_end_hook() {
    let ended = Arc::new(AtomicUsize::new(0));
    let count = Arc::clone(&ended);
    let schedule = ScheduleParts::new(
        || {},
        || {},
        |runnable: Runnable| drop(runnable),
        move || {
            count.fetch_add(1, Ordering::Relaxed);
        },
    );

    let (runnable, task) = async_task::spawn(async {}, schedule);
    drop(task);
    runnable.schedule();
    assert_eq!(ended.load(Ordering::Relaxed), 1);
}
