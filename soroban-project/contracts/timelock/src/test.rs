//! This module contains unit tests for the TapMint contract.
//! It tests various functionalities such as contract initialization,
//! token minting, cooldown periods, and error handling.

#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::{testutils::Address as _, token, Env};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;
extern crate std;

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (Address, TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        contract_address.clone(),
        TokenClient::new(e, &contract_address),
        TokenAdminClient::new(e, &contract_address),
    )
}

fn create_tap_mint_contract<'a>(e: &Env) -> TapMintContractClient<'a> {
    let contract_id = e.register_contract(None, TapMintContract);
    TapMintContractClient::new(e, &contract_id)
}

struct TapMintTest<'a> {
    env: Env,
    token_client: TokenClient<'a>,
    contract: TapMintContractClient<'a>,
    player1: Address,
}

impl<'a> TapMintTest<'a> {
    fn setup(init_contract: bool) -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let token_admin = Address::generate(&env);
        let token_distributor: Address = Address::generate(&env);
        let player1 = Address::generate(&env);

        let (token_id, token_client, token_admin_client) =
            create_token_contract(&env, &token_admin);
        token_admin_client.mint(&token_distributor, &1000);

        let contract = create_tap_mint_contract(&env);
        if init_contract {
            contract.initialize(&token_distributor, &1000, &token_id);
            assert_eq!(token_client.balance(&contract.address), 1000);
        }
        TapMintTest {
            env,
            token_client,
            contract,
            player1,
        }
    }
}

/// Test the initialization of the TapMint contract.
/// This test verifies that the contract is correctly initialized with the expected token balance.
#[test]
fn test_initialize() {
    let test = TapMintTest::setup(true);
    assert_eq!(test.token_client.balance(&test.contract.address), 1000);
}

/// Test the minting functionality of the TapMint contract.
/// This test checks if:
/// 1. A player can successfully mint tokens
/// 2. The correct amount of tokens is minted
/// 3. The player can mint again after the cooldown period
#[test]
fn test_mint() {
    let test = TapMintTest::setup(true);
    let mint_key = test.contract.get_mint_key(&test.player1);
    let mint_time = test.contract.mint(&test.player1, &mint_key.key);

    // Check if the correct amount of tokens was minted
    assert_eq!(
        test.token_client.balance(&test.player1),
        MINT_AMOUNT as i128
    );

    // Simulate time passing beyond the cooldown period
    let new_timestamp = test.env.ledger().timestamp() + MINT_COOLDOWN_SECONDS + 1000;
    test.env.ledger().set_timestamp(new_timestamp);
    test.env.ledger().set_sequence_number(10);

    // Mint again after cooldown
    let new_mint_key = test.contract.get_mint_key(&test.player1);
    let new_mint_time = test.contract.mint(&test.player1, &new_mint_key.key);
    assert!(new_mint_time > mint_time);
    assert_eq!(
        test.token_client.balance(&test.player1),
        (MINT_AMOUNT * 2) as i128
    );
}

/// Test minting with an invalid key.
/// This test verifies that the contract correctly rejects minting attempts with an invalid key.
#[test]
#[should_panic(expected = "Invalid mint key")]
fn test_mint_invalid_key() {
    let test = TapMintTest::setup(true);
    let invalid_key: i128 = 12345;
    test.contract.mint(&test.player1, &invalid_key);
}

/// Test minting before the cooldown period has elapsed.
/// This test ensures that the contract prevents players from minting tokens too frequently.
#[test]
#[should_panic(expected = "Mint cooldown not over for player")]
fn test_mint_before_cooldown() {
    let test = TapMintTest::setup(true);
    let mint_key = test.contract.get_mint_key(&test.player1);
    test.contract.mint(&test.player1, &mint_key.key);
    // Immediately try to mint again at the same timestamp
    test.contract.mint(&test.player1, &mint_key.key);
}

/// Test minting when the contract is not initialized.
/// This test verifies that the contract correctly prevents minting when it hasn't been initialized.
#[test]
#[should_panic(expected = "Contract is not yet initialized")]
fn test_mint_uninitialized() {
    let test = TapMintTest::setup(false);
    let mint_key = test.contract.get_mint_key(&test.player1);
    test.contract.mint(&test.player1, &mint_key.key);
}
