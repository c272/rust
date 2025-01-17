//! System bindings for the wasm/web platform
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for wasm. Note that this wasm is *not* the emscripten
//! wasm, so we have no runtime here.
//!
//! This is all super highly experimental and not actually intended for
//! wide/production use yet, it's still all in the experimental category. This
//! will likely change over time.
//!
//! Currently all functions here are basically stubs that immediately return
//! errors. The hope is that with a portability lint we can turn actually just
//! remove all this and just omit parts of the standard library if we're
//! compiling for wasm. That way it's a compile time error for something that's
//! guaranteed to be a runtime error!

#![deny(unsafe_op_in_unsafe_fn)]

pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod env;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
#[path = "../unsupported/stdio.rs"]
pub mod stdio;
#[path = "../unsupported/thread_local_dtor.rs"]
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
#[path = "../unsupported/time.rs"]
pub mod time;

cfg_if::cfg_if! {
    if #[cfg(target_feature = "atomics")] {
        #[path = "../unix/locks"]
        pub mod locks {
            #![allow(unsafe_op_in_unsafe_fn)]
            mod futex_condvar;
            mod futex_mutex;
            mod futex_rwlock;
            pub(crate) use futex_condvar::Condvar;
            pub(crate) use futex_mutex::Mutex;
            pub(crate) use futex_rwlock::RwLock;
        }
        #[path = "atomics/futex.rs"]
        pub mod futex;
        #[path = "atomics/thread.rs"]
        pub mod thread;
    } else {
        #[path = "../unsupported/locks/mod.rs"]
        pub mod locks;
        #[path = "../unsupported/once.rs"]
        pub mod once;
        #[path = "../unsupported/thread.rs"]
        pub mod thread;
        #[path = "../unsupported/thread_parking.rs"]
        pub mod thread_parking;
    }
}

#[path = "../unsupported/common.rs"]
#[deny(unsafe_op_in_unsafe_fn)]
mod common;
pub use common::*;
