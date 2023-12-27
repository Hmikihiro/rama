//! `async fn serve(&self, Context<S>, Request) -> Result<Response, Error>`
//!
//! Heavily inspired by [tower-service](https://docs.rs/tower-service/0.3.0/tower_service/trait.Service.html)
//! and the vast [Tokio](https://docs.rs/tokio/latest/tokio/) ecosystem which makes use of it.
//!
//! Initially the goal was to rely on `tower-service` directly, but it turned out to be
//! too restrictive and difficult to work with, for the use cases we have in Rama.
//! See <https://ramaproxy.org/man/faq.html> for more information regarding this and more.

mod context;
pub use context::Context;

mod svc;
pub use svc::{BoxService, Service};

mod svc_fn;
pub use svc_fn::{service_fn, ServiceFn};

pub mod layer;
pub use layer::Layer;

mod builder;
pub use builder::ServiceBuilder;
