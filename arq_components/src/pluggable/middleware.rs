
use std::any::Any;
use rocket::fairing::Fairing;



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
    fn middlewares(&self) -> (*mut dyn Fairing, usize, usize);
}


/// This struct is used to export the component's middlewares to the CORE.
/// It must be used alongside the macro `declare_middleware!`, so the ComponentManager can find the MiddlewareComponent.
pub struct MiddlewareFactory {
    pub middlewares: Vec<Box<dyn Fairing>>
}

impl MiddlewareFactory {}

