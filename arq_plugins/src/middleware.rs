
use std::{mem, any::Any};
use rocket::{fairing::Fairing, Request, Data, Response};



/// This trait is used when one wants to add middleware to the ARQ_CORE.
/// This acts as a request guard and so should be used with caution, as it could interfiere with the base CORE routes.
pub trait MiddlewareComponent: Any + Send + Sync {
    /// This function should return the name of the middleware.
    fn name(&self) -> &'static str;
    /// This function is fired before the middleware is mounted by CORE.
    fn on_middleware_load(&self) {}
    /// This function is fired after the middleware is unmounted by CORE.
    fn on_middleware_unload(&self) {}
    /// This function should return the routes that should be mounted by CORE.
    /// The returning should be handled by the `arq_components::pluggable::middleware::MiddlewareFactory`.
    fn middlewares(&self) -> (*mut Box<dyn Fairing>, usize, usize);
}

/// A struct that will act as a wrapper for `dyn Fairing` since rocket cannot attach `dyn Trait` as a middleware
/// This should only be used by the ComponentManager, users do not need to worry about this
pub struct DynFairing {
    inner: Box<dyn Fairing>
}

impl DynFairing {
    /// Constructs a new DynFairing
    pub fn from(inner: Box<dyn Fairing>) -> Self {
        Self {
            inner
        }
    }   
}

#[rocket::async_trait]
impl Fairing for DynFairing {
    fn info(&self) -> rocket::fairing::Info {
        self.inner.info()
    }
    async fn on_request(&self, request: &mut Request<'_>, data: &mut Data<'_>) {
        self.inner.on_request(request, data);
    }
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        self.inner.on_response(request, response);
    }
}