// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use dwallet_gas_pool::command::Command;

#[tokio::main]
async fn main() {
    print!("hi");

    let command = Command::parse();
    command.execute().await;
}
