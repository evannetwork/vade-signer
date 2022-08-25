/*
  Copyright (c) 2018-present evan GmbH.

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

//! Small helper module to support signing in [`Vade`] plugins.
//!
//! ## Configuring signer for vade projects
//!
//! Some [`Vade`] plugins need a signer to be able to sign messages. The signer is usually created and passed to the plugin`s constructor, e.g. for using a local signer:
//!
//! ```rust
//! use vade_evan_substrate::resolver::{ResolverConfig, VadeEvanSubstrate};
//! use vade_signer::{LocalSigner, Signer},
//! let signer: Box<dyn Signer> = Box::new(LocalSigner::new());
//! let resolver = VadeEvanSubstrate::new(ResolverConfig {
//!     signer,
//!     target: "127.0.0.1".to_string(),
//! });
//! ```
//!
//! When signing remotely, `signing_url` will be called with a POST request. The messages that should be signed is passed to the server alongside a reference to a key like this:
//!
//! ```json
//! {
//!     "key": "some-key-id",
//!     "type": "some-key-type",
//!     "message": "sign me please"
//! }
//! ```
//!
//! Two types of of responses are supported. Successful signing results are give in this format:
//!
//! ```json
//! {
//!   "messageHash": "0x52091d1299031b18c1099620a1786363855d9fcd91a7686c866ad64f83de13ff",
//!   "signature": "0xc465a9499b75eed6fc4f658d1764168d63578b05ae15305ceedc94872bda793f74cb850c0683287b245b4da523851fbbe37738116635ebdb08e80b867c0b4aea1b",
//!   "signerAddress": "0x3daa2c354dba8d51fdabc30cf9219b251c74eb56"
//! }
//! ```
//!
//! Errors can be signaled this way:
//!
//! ```json
//! {
//!     "error": "key not found"
//! }
//! ```
//!
//! [`Vade`]: https://docs.rs/vade/*/vade/struct.Vade.html

mod signing;
pub use signing::*;
