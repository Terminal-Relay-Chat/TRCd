//! Database module
//! Because TRCd is meant to run minimally, it will (by default) pull all users from a file and
//! store them in memory.
//!
//! If you need a more legit storage setup, please open an issue as it is being considered by the
//! team.

pub mod minimalist; // the minimalist database
