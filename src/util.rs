//! Some utility stuff

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Creates a future that resolves to the provided value.
pub fn ready<T>(val: T) -> Ready<T> {
    Ready(Some(val))
}

/// Complete on first poll with the provided value.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Ready<T>(Option<T>);

impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("`Ready` polled after completion"))
    }
}
