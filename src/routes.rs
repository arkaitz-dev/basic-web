use crate::controllers;
use axum::response::IntoResponse;
use axum_htmx::HxRequest;

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

    pub fn handler(&self) -> fn(HxRequest) -> impl Future<Output = impl IntoResponse> {
        match self {
            Route::Home => controllers::home::handler,
            Route::About => controllers::about::handler,
            Route::Contact => controllers::contact::handler,
            Route::Experience => controllers::experience::handler,
            Route::Projects => controllers::projects::handler,
        }
    }
}
