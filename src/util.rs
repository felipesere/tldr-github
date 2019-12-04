use std::future::Future;
use std::pin::Pin;

// shamelessly taken from https://github.com/http-rs/tide/blob/master/src/utils.rs

/// An owned dynamically typed [`Future`] for use in cases where you can't
/// statically type your result or need to add some indirection.
pub(crate) type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
