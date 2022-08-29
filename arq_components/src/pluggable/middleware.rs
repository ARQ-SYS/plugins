
use std::{any::Any, mem};
use rocket::fairing::{Fairing, self};



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


/// This struct is used to export the component's middlewares to the CORE.
/// It must be used alongside the macro `declare_middleware!`, so the ComponentManager can find the MiddlewareComponent.
pub struct MiddlewareFactory {
    pub middlewares: Vec<Box<dyn Fairing>>
}

impl MiddlewareFactory {

    pub fn new() -> Self {
        MiddlewareFactory { middlewares: Vec::new() }
    }

    pub fn add_middleware(mut self, middleware: Box<dyn Fairing>) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn export(mut self) -> (*mut Box<dyn Fairing>, usize, usize) {


        self.middlewares.shrink_to_fit();
        assert!(self.middlewares.len() == self.middlewares.capacity());

        let ptr = self.middlewares.as_mut_ptr();
        let len = self.middlewares.len();

        mem::forget(self.middlewares);

        (ptr, len, len)

    }

}

