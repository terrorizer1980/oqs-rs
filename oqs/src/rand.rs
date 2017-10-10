// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::{mem, ptr};
use std::fmt;

use oqs_sys::rand as ffi;

/// Enum representation of the supported PRNG algorithms.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OqsRandAlg {
    Default = ffi::OQS_RAND_alg_name::OQS_RAND_alg_default as u32,
    UrandomChacha20 = ffi::OQS_RAND_alg_name::OQS_RAND_alg_urandom_chacha20 as u32,
    UrandomAesctr = ffi::OQS_RAND_alg_name::OQS_RAND_alg_urandom_aesctr as u32,
}

impl From<OqsRandAlg> for ffi::OQS_RAND_alg_name {
    fn from(alg: OqsRandAlg) -> Self {
        unsafe { mem::transmute_copy::<OqsRandAlg, ffi::OQS_RAND_alg_name>(&alg) }
    }
}

impl Default for OqsRandAlg {
    fn default() -> Self {
        OqsRandAlg::Default
    }
}

pub struct OqsRand {
    algorithm: OqsRandAlg,
    pub(crate) oqs_rand: *mut ffi::OQS_RAND,
}

impl OqsRand {
    /// Initializes and returns a new PRNG based on the given algorithm.
    pub fn new(algorithm: OqsRandAlg) -> Result<Self> {
        let oqs_rand = unsafe { ffi::OQS_RAND_new(ffi::OQS_RAND_alg_name::from(algorithm)) };
        if oqs_rand != ptr::null_mut() {
            Ok(OqsRand {
                algorithm,
                oqs_rand,
            })
        } else {
            Err(Error)
        }
    }

    /// Returns the algorithm backing this PRNG.
    pub fn algorithm(&self) -> OqsRandAlg {
        self.algorithm
    }
}

impl Drop for OqsRand {
    fn drop(&mut self) {
        unsafe { ffi::OQS_RAND_free(self.oqs_rand) };
    }
}


pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Hash)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        use std::error::Error;
        self.description().fmt(f)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "Error during PRNG initialization"
    }
}