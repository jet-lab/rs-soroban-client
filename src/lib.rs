//! A rust client library
//! for interacting with Soroban smart contracts on the stellar blockchain
pub mod friendbot;
pub mod http_client;
pub mod jsonrpc;
pub mod soroban_rpc;
pub use http_client::{HTTPClient, VERSION};
pub use stellar_baselib::*;
pub mod server;
pub mod transaction;
