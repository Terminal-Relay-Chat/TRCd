pub mod socket_server;
// the Server module is instantiated by the socket_server, and needs to be so for functionality,
// therefore it is not public
mod server;

pub const MAX_CHANNEL_NAME_LENGTH_BYTES: usize = size_of::<char>() * 30; // 30 basic characters
                                                                         // long.

