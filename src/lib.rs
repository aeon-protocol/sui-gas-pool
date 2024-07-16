// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
extern crate dwallet_config as sui_config;
extern crate dwallet_json as sui_json;
extern crate dwallet_json_rpc as sui_json_rpc;
extern crate dwallet_json_rpc_types as sui_json_rpc_types;
extern crate dwallet_metrics as mysten_metrics;
extern crate dwallet_move_core_types as move_core_types;
extern crate dwallet_open_rpc as sui_open_rpc;
extern crate dwallet_package_resolver as sui_package_resolver;
extern crate dwallet_protocol_config as sui_protocol_config;
extern crate dwallet_rest_api as sui_rest_api;
extern crate dwallet_sdk as sui_sdk;
extern crate dwallet_telemetry_subscribers as telemetry_subscribers;
extern crate dwallet_transaction_builder as sui_transaction_builder;
extern crate dwallet_types as sui_types;

pub mod benchmarks;
pub mod command;
pub mod config;
pub mod errors;
pub mod gas_pool;
pub mod gas_pool_initializer;
pub mod metrics;
pub mod rpc;
pub mod storage;
pub mod sui_client;
#[cfg(test)]
pub mod test_env;
pub mod tx_signer;
pub mod types;

pub const AUTH_ENV_NAME: &str = "GAS_STATION_AUTH";

pub fn read_auth_env() -> String {
    std::env::var(AUTH_ENV_NAME)
        .ok()
        .unwrap_or_else(|| panic!("{} environment variable must be specified", AUTH_ENV_NAME))
        .parse::<String>()
        .unwrap()
}
