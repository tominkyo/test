// here we create general components that are reusable by anyapp
// Don't import anything this folder from outside.

pub mod get_users;
pub use get_users::get_users_data;
