use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Default)]
pub struct TestLock {
    inner: RwLock<()>,
}

impl TestLock {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run_concurrently(&self) -> RwLockReadGuard<'_, ()> {
        self.inner.read().unwrap()
    }

    pub fn run_exclusively(&self) -> RwLockWriteGuard<'_, ()> {
        self.inner.write().unwrap()
    }
}
