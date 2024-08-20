//! # TapMint Contract
//!
//! This contract implements a token minting system with cooldown periods and mint keys.
//! Players can request mint keys and use them to mint tokens, subject to cooldown restrictions.
//!
//! ## Features
//!
//! - Initialize contract with a token supply
//! - Generate mint keys for players
//! - Mint tokens with valid keys and after cooldown period
//! - Check token balances
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Map, String,
};

const MINT_COOLDOWN_SECONDS: u64 = 3 * 60; // 3 minutes
const MINT_AMOUNT: u64 = 20; // How many tokens a player gets each time they 'mint'.

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    PlayerMintTime,
    PlayerMintKey,
    TokenAddress,
}

#[derive(Clone)]
#[contracttype]
pub struct MintKey {
    pub player: String,
    pub key: i128,
    pub generated_at: u64,
}

#[contract]
pub struct TapMintContract;

#[contractimpl]
impl TapMintContract {
    /// Initializes the contract with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `env` - The environment in which the contract is running.
    /// * `from` - The address of the account initializing the contract.
    /// * `mint_supply` - The initial supply of tokens for minting.
    /// * `token_address` - The address of the token contract.
    pub fn initialize(env: Env, from: Address, mint_supply: i128, token_address: Address) {
        if is_initialized(&env) {
            panic!("Contract has already been initialized");
        }

        // Give contract tokens to mint to others
        from.require_auth();
        token::Client::new(&env, &token_address).transfer(
            &from,
            &env.current_contract_address(),
            &mint_supply,
        );

        // Initialize storage maps
        let mint_time_map: Map<String, u64> = Map::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::PlayerMintTime, &mint_time_map);

        let mint_key_map: Map<String, MintKey> = Map::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::PlayerMintKey, &mint_key_map);

        // Store token address and mark as initialized
        env.storage()
            .instance()
            .set(&DataKey::TokenAddress, &token_address);
        env.storage().instance().set(&DataKey::Init, &());
    }

    /// Mints tokens for a player if conditions are met.
    ///
    /// # Arguments
    ///
    /// * `env` - The environment in which the contract is running.
    /// * `player` - The address of the player minting tokens.
    /// * `mint_key` - The key required for minting.
    ///
    /// # Returns
    ///
    /// The current timestamp after successful minting.
    pub fn mint(env: Env, player: Address, mint_key: i128) -> u64 {
        if !is_initialized(&env) {
            panic!("Contract is not yet initialized");
        }

        player.require_auth();

        let player_pub_key = player.to_string();
        if !is_cooldown_over(&env, &player_pub_key) {
            panic!("Mint cooldown not over for player");
        }
        if !is_mint_key_valid(&env, &player_pub_key, mint_key) {
            panic!("Invalid mint key");
        }

        // Transfer tokens to player
        let token_address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .unwrap();
        token::Client::new(&env, &token_address).transfer(
            &env.current_contract_address(),
            &player,
            &(MINT_AMOUNT as i128),
        );

        // Remove used mint key
        let mut mint_key_map: Map<String, MintKey> = env
            .storage()
            .instance()
            .get(&DataKey::PlayerMintKey)
            .unwrap();
        mint_key_map.remove(&player_pub_key);
        env.storage()
            .instance()
            .set(&DataKey::PlayerMintKey, &mint_key_map);

        // Update player's last mint time
        let mut mint_map: Map<String, u64> = env
            .storage()
            .instance()
            .get(&DataKey::PlayerMintTime)
            .unwrap();
        let current_time = env.ledger().timestamp();
        mint_map.set(player_pub_key.clone(), current_time);
        env.storage()
            .instance()
            .set(&DataKey::PlayerMintTime, &mint_map);

        // Publish mint event
        env.events()
            .publish((symbol_short!("Mint"), player_pub_key), MINT_AMOUNT);

        // Extend contract instance TTL
        env.storage().instance().extend_ttl(2000, 10000);
        current_time
    }

    /// Generates a new mint key for a player.
    ///
    /// # Arguments
    ///
    /// * `env` - The environment in which the contract is running.
    /// * `player` - The address of the player requesting a mint key.
    ///
    /// # Returns
    ///
    /// A new `MintKey` for the player.
    pub fn get_mint_key(env: Env, player: Address) -> MintKey {
        if !is_initialized(&env) {
            panic!("Contract is not yet initialized");
        }
        player.require_auth();

        let player_pub_key = player.to_string();
        if !is_cooldown_over(&env, &player_pub_key) {
            panic!("Mint cooldown not over for player");
        }

        let mint_key = generate_mint_key(&env, &player_pub_key);
        let mut mint_key_map: Map<String, MintKey> = env
            .storage()
            .instance()
            .get(&DataKey::PlayerMintKey)
            .unwrap();
        mint_key_map.set(player_pub_key, mint_key.clone());
        env.storage()
            .instance()
            .set(&DataKey::PlayerMintKey, &mint_key_map);

        env.storage().instance().extend_ttl(2000, 10000);
        mint_key
    }

    /// Retrieves the balance of the specified address.
    ///
    /// # Arguments
    ///
    /// * `env` - The environment in which the contract is running.
    /// * `address` - The address to check the balance for.
    pub fn balance(env: Env, address: Address) -> i128 {
        env.storage().instance().extend_ttl(2000, 10000);
        let token_address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .unwrap();
        token::Client::new(&env, &token_address).balance(&address)
    }
}

/// Checks if the contract has been initialized.
///
/// # Arguments
///
/// * `env` - The environment in which the contract is running.
///
/// # Returns
///
/// `true` if the contract is initialized, `false` otherwise.
fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Init)
}

/// Validates the mint key for a given player.
///
/// # Arguments
///
/// * `env` - The environment in which the contract is running.
/// * `player_pub_key` - The public key of the player.
/// * `key` - The mint key to validate.
///
/// # Returns
///
/// `true` if the mint key is valid, `false` otherwise.
fn is_mint_key_valid(env: &Env, player_pub_key: &String, key: i128) -> bool {
    let mint_key_map: Map<String, MintKey> = env
        .storage()
        .instance()
        .get(&DataKey::PlayerMintKey)
        .unwrap();
    if let Some(mint_key) = mint_key_map.get(player_pub_key) {
        mint_key.player == *player_pub_key && mint_key.key == key
    } else {
        false
    }
}

/// Generates a new mint key for a player.
///
/// # Arguments
///
/// * `env` - The environment in which the contract is running.
/// * `player_pub_key` - The public key of the player.
///
/// # Returns
///
/// A new `MintKey` for the player.
fn generate_mint_key(env: &Env, player_pub_key: &String) -> MintKey {
    let timestamp = env.ledger().timestamp();
    let pseudo_random = env.prng().gen_range(0..10000);
    MintKey {
        player: player_pub_key.clone(),
        key: (timestamp.saturating_add(pseudo_random) as i128),
        generated_at: timestamp,
    }
}

/// Checks if the cooldown period for minting has passed for a player.
///
/// # Arguments
///
/// * `env` - The environment in which the contract is running.
/// * `player_pub_key` - The public key of the player.
///
/// # Returns
///
/// `true` if the cooldown period has passed, `false` otherwise.
fn is_cooldown_over(env: &Env, player_pub_key: &String) -> bool {
    let mint_map: Map<String, u64> = env
        .storage()
        .instance()
        .get(&DataKey::PlayerMintTime)
        .unwrap();
    if let Some(last_mint_time) = mint_map.get(player_pub_key) {
        let ledger_timestamp = env.ledger().timestamp();
        return ledger_timestamp >= (last_mint_time + (MINT_COOLDOWN_SECONDS as u64));
    }
    true
}

mod test;
