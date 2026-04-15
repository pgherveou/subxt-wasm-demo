// Vendored from subxt-lightclient 0.50.0 (Apache-2.0 OR GPL-3.0)

use super::wasm_socket::WasmSocket;

use core::time::Duration;
use futures_util::{FutureExt, future};

pub fn now_from_unix_epoch() -> Duration {
    web_time::SystemTime::now()
        .duration_since(web_time::SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|_| {
            panic!("Invalid systime cannot be configured earlier than `UNIX_EPOCH`")
        })
}

pub type Instant = web_time::Instant;

pub fn now() -> Instant {
    web_time::Instant::now()
}

pub type Delay = future::BoxFuture<'static, ()>;

pub fn sleep(duration: Duration) -> Delay {
    futures_timer::Delay::new(duration).boxed()
}

#[pin_project::pin_project]
pub struct Stream(
    #[pin]
    pub  smoldot::libp2p::with_buffers::WithBuffers<
        future::BoxFuture<'static, Result<WasmSocket, std::io::Error>>,
        WasmSocket,
        Instant,
    >,
);
