// Copyright 2022 labring. All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0

use sealfs::client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = client::init_fs_client();
    println!("client stopped. success = {:?}", result.is_ok());
    Ok(())
}
