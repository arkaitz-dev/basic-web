pub mod home;
pub mod about;
pub mod experience;
pub mod projects;
pub mod contact;

pub use home::home_handler;
pub use about::about_handler;
pub use experience::experience_handler;
pub use projects::projects_handler;
pub use contact::{contact_handler, contact_submit};