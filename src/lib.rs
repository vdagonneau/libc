// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! libc - Raw FFI bindings to platforms' system libraries

#![crate_name = "libc"]
#![crate_type = "rlib"]
#![cfg_attr(not(feature = "rustc-dep-of-std"), deny(warnings))]
#![allow(bad_style, overflowing_literals, improper_ctypes, unknown_lints)]
#![cfg_attr(cross_platform_docs, feature(no_core, lang_items, const_fn))]
#![cfg_attr(cross_platform_docs, no_core)]
// Attributes needed when building as part of the standard library
#![cfg_attr(
    feature = "rustc-dep-of-std",
    feature(cfg_target_vendor, link_cfg, no_core)
)]
// Enable extra lints:
#![cfg_attr(feature = "extra_traits", deny(missing_debug_implementations))]
#![deny(missing_copy_implementations, safe_packed_borrows)]
// Enable no_std:
#![cfg_attr(
    not(any(
        feature = "use_std",
        feature = "rustc-dep-of-std",
        cross_platform_docs
    )),
    no_std
)]

#[cfg(all(not(cross_platform_docs), feature = "use_std"))]
extern crate std as core;

#[macro_use]
mod macros;

cfg_if! {
    if #[cfg(feature = "rustc-dep-of-std")] {
        extern crate rustc_std_workspace_core as core;
        #[allow(unused_imports)]
        use core::iter;
        #[allow(unused_imports)]
        use core::option;
    }
}

cfg_if! {
    if #[cfg(not(cross_platform_docs))] {
        cfg_if! {
            if #[cfg(libc_priv_mod_use)] {
                #[cfg(libc_core_cvoid)]
                #[allow(unused_imports)]
                use core::ffi;
                #[allow(unused_imports)]
                use core::fmt;
                #[allow(unused_imports)]
                use core::hash;
                #[allow(unused_imports)]
                use core::num;
                #[allow(unused_imports)]
                use core::mem;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::clone::Clone;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::marker::Copy;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::option::Option;
            } else {
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::fmt;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::hash;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::num;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::mem;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::clone::Clone;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::marker::Copy;
                #[doc(hidden)]
                #[allow(unused_imports)]
                pub use core::option::Option;
            }
        }
    }
}

cfg_if! {
    if #[cfg(cross_platform_docs)] {
        mod dox;
        pub use self::dox::*;
    }
}

cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use windows::*;
    } else if #[cfg(target_os = "redox")] {
        mod redox;
        pub use redox::*;
    } else if #[cfg(target_os = "cloudabi")] {
        mod cloudabi;
        pub use cloudabi::*;
    } else if #[cfg(target_os = "fuchsia")] {
        mod fuchsia;
        pub use fuchsia::*;
    } else if #[cfg(target_os = "switch")] {
        mod switch;
        pub use switch::*;
    } else if #[cfg(unix)] {
        mod unix;
        pub use unix::*;
    } else if #[cfg(target_env = "sgx")] {
        mod sgx;
        pub use sgx::*;
    }  else {
        // non-supported targets: empty...
    }
}
