// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

//! Nostr SDK Network

pub mod error;
pub mod message;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "native")]
pub mod native;
#[cfg(target_arch = "wasm32")]
pub mod web;

pub use self::message::*;
