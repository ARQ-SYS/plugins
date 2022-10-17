use rocket::{Route, fairing::Fairing};

/// This struct will be used to export all middlewares and routes to CORE  
/// It must be used alongside the `declare_component!()`/`declare_middleware!()` macro, so that CORE can find it  
#[derive(Default)]
pub struct PluginExporter {
    pub middlewares: Vec<Box<dyn Fairing>>,
    pub routes: Vec<Route>
}

impl PluginExporter {

    /// Constructs a new PluginExporter.
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            middlewares: Vec::new()
        }
    }
    /// Adds routes to be exported  
    /// ```rust
    /// ...add_routes(routes![...])
    /// ```
    pub fn add_routes(mut self, routes: Vec<Route>) -> Self {
        self.routes.extend(routes);
        self
    }
    /// Adds middlewares to be exported
    pub fn add_middlewares(mut self, middlewares: Vec<Box<dyn Fairing>>) -> Self {
        self.middlewares.extend(middlewares);
        self
    }
    /// Exports routes 
    pub fn export_routes(mut self) -> (*mut Route, usize, usize) {
        // This makes sure that the routes' length and capacity are the same.
        self.routes.shrink_to_fit();
        assert!(self.routes.len() == self.routes.capacity());

        let ptr = self.routes.as_mut_ptr();
        let len = self.routes.len();

        // This makes sure that the routes are not dropped.
        std::mem::forget(self.routes);

        (ptr, len, len)
    }
    /// Exports middlewares
    pub fn export_middlewares(mut self) -> (*mut Box<dyn Fairing>, usize, usize) {
        self.middlewares.shrink_to_fit();
        assert!(self.middlewares.len() == self.middlewares.capacity());

        let ptr = self.middlewares.as_mut_ptr();
        let len = self.middlewares.len();

        std::mem::forget(self.middlewares);

        (ptr, len, len)
    }
}