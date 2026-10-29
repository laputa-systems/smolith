use std::future::poll_fn;
use std::task::Poll;

use async_task::Runnable;

#[test]
fn local_task_wakes_itself() {
    let (sender, receiver) = flume::unbounded::<Runnable>();
    let schedule = move |runnable| sender.send(runnable).unwrap();

    let mut polled = false;
    let future = poll_fn(move |cx| {
        if !polled {
            polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(7)
        }
    });

    let (runnable, task) = async_task::spawn_local(future, schedule);
    runnable.run();
    while let Ok(runnable) = receiver.try_recv() {
        runnable.run();
    }
    assert_eq!(smol::future::block_on(task), 7);
}
