//! 5G Service Based Interface Data Types and Stubs

mod generator;
pub use generator::Generator;

mod utils;

mod schema;
pub use schema::{sanitize_str_for_ident, AnyOfHandler};

mod anyof_handlers;
pub use anyof_handlers::default_anyof_handler;
