pub enum Route {
    Home,
    About,
    Contact,
    Experience,
    Projects,
}

impl Route {
    pub fn path(&self) -> &'static str {
        match self {
            Route::Home => "/",
            Route::About => "/about",
            Route::Contact => "/contact",
            Route::Experience => "/experience",
            Route::Projects => "/projects",
        }
    }
}
