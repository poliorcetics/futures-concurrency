use std::sync::Arc;
use std::sync::Mutex;
use std::task::Waker;

use super::{InlineWaker, ReadinessVec};
use crate::utils;

/// A collection of wakers which delegate to an in-line waker.
pub(crate) struct WakerVec {
    wakers: Vec<Waker>,
    readiness: Arc<Mutex<ReadinessVec>>,
}

impl WakerVec {
    /// Create a new instance of `WakerVec`.
    pub(crate) fn new(len: usize) -> Self {
        let readiness = Arc::new(Mutex::new(ReadinessVec::new(len)));
        Self {
            wakers: (0..len)
                .map(|i| Arc::new(InlineWaker::new(i, readiness.clone())).into())
                .collect(),
            readiness,
        }
    }

    pub(crate) fn get(&self, index: usize) -> Option<&Waker> {
        self.wakers.get(index)
    }

    /// Access the `Readiness`.
    pub(crate) fn readiness(&self) -> &Mutex<ReadinessVec> {
        self.readiness.as_ref()
    }
}
