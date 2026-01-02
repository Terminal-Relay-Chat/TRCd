pub mod socket_server;
pub mod server;

pub const MAX_CHANNEL_NAME_LENGTH_BYTES: usize = size_of::<char>() * 30; // 30 basic characters
                                                                         // long.

