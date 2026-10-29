mod support;

#[cfg(test)]
mod boundaries;

pub use support::{abort_after, cpu_time, within};
