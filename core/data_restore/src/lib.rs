#[macro_use]
extern crate log;

pub mod accounts_state;
pub mod data_restore_driver;
pub mod events;
pub mod events_state;
pub mod rollup_ops;
pub mod genesis_state;
pub mod helpers;
pub mod storage_interactor;

use crate::data_restore_driver::DataRestoreDriver;
use storage::ConnectionPool;

/// Step of the considered blocks ethereum block
const ETH_BLOCKS_DELTA: u64 = 250;
/// Delta between last ethereum block and last watched ethereum block to prevent restart in case of reorder
const END_ETH_BLOCKS_DELTA: u64 = 25;

/// Creates data restore driver state
///
/// # Arguments
///
/// * `connection_pool` - Database connection pool
///
fn create_data_restore_driver(connection_pool: ConnectionPool, web3_url: String, contract_eth_addr: H160, contract_genesis_tx_hash: H256) -> DataRestoreDriver {
    DataRestoreDriver::new(connection_pool, web3_url, contract_eth_addr, contract_genesis_tx_hash, ETH_BLOCKS_DELTA, END_ETH_BLOCKS_DELTA)
    info!("Driver created");
}

/// Loads states from storage and start update
fn load_state(driver: &mut DataRestoreDriver) {
    driver
        .load_state_from_storage()
        .expect("Cant load state");
}

/// Runs states updates
///
/// # Arguments
///
/// * `driver` - DataRestore Driver config
///
fn run_state_updates(driver: &mut DataRestoreDriver) {
    driver.run_state_updates().expect("Cant run updates");
}

fn stop_state_updates(driver: &mut DataRestoreDriver) {
    driver.stop_state_updates().expect("Cant stop updates");
}
