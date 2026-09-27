//! Command modules for airmesh-cli.

pub mod daemon;
pub mod discover;
pub mod list;
pub mod pair;
pub mod receive;
pub mod search;
pub mod send;

pub use daemon::execute_daemon_status;
pub use discover::execute_discover;
pub use list::execute_list;
pub use pair::execute_pair;
pub use receive::execute_receive;
pub use search::execute_search;
pub use send::execute_send;
