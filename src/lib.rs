//! AES-GCM-SST per [draft-mattson-cfrg-aes-gcm-sst-01].
//!
//! <https://www.ietf.org/archive/id/draft-mattsson-cfrg-aes-gcm-sst-01.html>

#![cfg_attr(docs, feature(doc_cfg))]
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(not(any(feature = "std", test)), deny(clippy::std_instead_of_core))]
#![deny(
    clippy::alloc_instead_of_core,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::expect_used,
    clippy::implicit_saturating_sub,
    clippy::indexing_slicing,
    clippy::missing_panics_doc,
    clippy::panic,
    clippy::ptr_as_ptr,
    clippy::string_slice,
    clippy::transmute_ptr_to_ptr,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::wildcard_imports,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

mod tests;

use aes::{Aes128, Aes256};
use gcm_sst::GcmSst;
use typenum::{U10, U4, U8};

/// 128-bit AES-GCM-SST with a four octet (32 bit) authentication
/// tag.
pub type AesGcm128Sst4 = GcmSst<Aes128, U4>;

/// 128-bit AES-GCM-SST with an eight octet (64 bit)
/// authentication tag.
pub type AesGcm128Sst8 = GcmSst<Aes128, U8>;

/// 128-bit AES-GCM-SST with a ten octet (80 bit) authentication
/// tag.
pub type AesGcm128Sst10 = GcmSst<Aes128, U10>;

/// 256-bit AES-GCM-SST with a four octet (32 bit) authentication
/// tag.
pub type AesGcm256Sst4 = GcmSst<Aes256, U4>;

/// 256-bit AES-GCM-SST with an eight octet (64 bit)
/// authentication tag.
pub type AesGcm256Sst8 = GcmSst<Aes256, U8>;

/// 256-bit AES-GCM-SST with a ten octet (80 bit) authentication
/// tag.
pub type AesGcm256Sst10 = GcmSst<Aes256, U10>;
