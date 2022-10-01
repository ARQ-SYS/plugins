use std::{mem, any::Any};
use rocket::Route;


/// A trait that represents a component that can be mounted to the ARQ_CORE.
/// This trait is used when one wants to add certain functionality to the ARQ_CORE.
/// Note: Managed state is not *YET* supported.
pub trait Component: Any + Send + Sync {

    /// This function should return the name of the component.
    fn name(&self) -> &'static str;
    /// This function is fired before the component is mounted by CORE.
    fn on_component_load(&self) {}
    /// This function is fired after the component is unmounted by CORE.
    /// This happens when the CORE is shutting down.
    fn on_component_unload(&self) {}
    /// This function should return the routes that should be mounted by CORE.
    /// The returning should be handled by the `arq_plugins::manager::PluginManager`
    fn routes(&self) -> (*mut Route, usize, usize);
}