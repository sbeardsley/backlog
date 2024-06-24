pub mod app;
pub mod builder;

pub mod domain {
    pub mod entities;
    pub mod errors;
    pub mod models;
    pub(crate) mod services;
    pub(crate) mod usecases;
}
pub mod repositories;
