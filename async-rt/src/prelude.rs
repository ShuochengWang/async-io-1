pub(crate) use alloc::boxed::Box;
pub(crate) use alloc::sync::Arc;
pub(crate) use alloc::vec::Vec;
pub(crate) use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
pub(crate) use core::task::{Context, Poll};
pub(crate) use spin::mutex::Mutex;

pub use core::future::Future;
pub use core::pin::Pin;
pub use futures::future::{BoxFuture, FutureExt};

pub use crate::task_local;

pub type Result<T> = core::result::Result<T, &'static str>;
