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

extern crate vade_signer;

use std::{env, error::Error};
use vade_signer::signing::{LocalSigner, RemoteSigner, Signer};

pub const DEFAULT_VADE_EVAN_SIGNING_URL: &str =
    "http://localhost:7070/key/sign";
pub const LOCAL_SIGNER_1_PRIVATE_KEY: &str =
    "dfcdcb6d5d09411ae9cbe1b0fd9751ba8803dd4b276d5bf9488ae4ede2669106";
pub const LOCAL_SIGNER_1_SIGNED_MESSAGE_HASH: &str =
    "0x216f85bc4d561a7c05231d12139a2d1a050c3baf3d33e057b8c25dcb3d7a8b94";
pub const REMOTE_SIGNER_1_PRIVATE_KEY: &str =
    "dfcdcb6d5d09411ae9cbe1b0fd9751ba8803dd4b276d5bf9488ae4ede2669106";
pub const REMOTE_SIGNER_1_SIGNED_MESSAGE_HASH: &str =
    "0x52091d1299031b18c1099620a1786363855d9fcd91a7686c866ad64f83de13ff";


#[tokio::test]
async fn can_sign_messages_locally() -> Result<(), Box<dyn Error>> {
    let signer = LocalSigner::new();
    let (_signature, message): ([u8; 65], [u8; 32]) = signer
        .sign_message("one two three four", LOCAL_SIGNER_1_PRIVATE_KEY)
        .await?;
    let message_hash = format!("0x{}", hex::encode(message));
    assert_eq!(message_hash, LOCAL_SIGNER_1_SIGNED_MESSAGE_HASH);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn can_sign_messages_remotely() -> Result<(), Box<dyn Error>> {
    let signer: Box<dyn Signer> = Box::new(RemoteSigner::new(
        env::var("VADE_EVAN_SIGNING_URL")
            .unwrap_or_else(|_| DEFAULT_VADE_EVAN_SIGNING_URL.to_string()),
    ));
    let (_signature, message): ([u8; 65], [u8; 32]) = signer
        .sign_message("one two three four", REMOTE_SIGNER_1_PRIVATE_KEY)
        .await?;
    let message_hash = format!("0x{}", hex::encode(message));
    assert_eq!(message_hash, REMOTE_SIGNER_1_SIGNED_MESSAGE_HASH);

    Ok(())
}