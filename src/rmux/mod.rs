mod crypto;
mod event;
mod message;
mod session;
mod stream;

pub use self::crypto::{read_rmux_event, write_encrypt_event, CryptoContext};
pub use self::event::{new_auth_event, Event, FLAG_AUTH};
pub use self::message::{AuthRequest, AuthResponse};
pub use self::session::{
    create_stream, dump_session_state, get_channel_session_size, handle_rmux_session,
    process_rmux_session, routine_all_sessions, MuxContext,
};

pub const DEFAULT_RECV_BUF_SIZE: usize = 64 * 1024;
