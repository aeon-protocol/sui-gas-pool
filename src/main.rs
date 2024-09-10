// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use dwallet_gas_pool::command::Command;

fn main() {
    let _guard = sentry::init(("https://1dd0fd2be29f39361bf956ece4336d25@o4507907608150016.ingest.de.sentry.io/4507929902841936", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
      }));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let command = Command::parse();
            command.execute().await;
        });
}
