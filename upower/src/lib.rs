#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

pub use ffi;
pub use gio;
pub use glib;

#[allow(unused_imports)]
mod auto;
pub use auto::*;

pub mod prelude {
    pub use super::auto::traits::*;
}
