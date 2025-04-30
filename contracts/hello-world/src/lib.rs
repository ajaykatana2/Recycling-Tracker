#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec, log};

#[contracttype]
pub struct RecyclingEntry {
    pub material: String,
    pub weight_kg: u64,
    pub timestamp: u64,
    pub points_earned: u64,
}

#[contracttype]
pub enum RecyclingKey {
    Log(Address),
    Points(Address),
}

#[contract]
pub struct RecyclingTracker;

#[contractimpl]
impl RecyclingTracker {
    // Log a recycling action and reward points
    pub fn log_recycling(env: Env, user: Address, material: String, weight_kg: u64) {
        user.require_auth();

        // Calculate points (e.g., 10 points per kg)
        let points = weight_kg * 10;

        // Append to user log
        let key = RecyclingKey::Log(user.clone());
        let mut log_list: Vec<RecyclingEntry> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        log_list.push_back(RecyclingEntry {
            material,
            weight_kg,
            timestamp: env.ledger().timestamp(),
            points_earned: points,
        });

        env.storage().persistent().set(&key, &log_list);

        // Update user points
        let point_key = RecyclingKey::Points(user.clone());
        let current_points = env.storage().persistent().get(&point_key).unwrap_or(0u64);
        env.storage().persistent().set(&point_key, &(current_points + points));

        log!(&env, "Recycling logged for user {} with {} points", user, points);
    }

    // Get total points for a user
    pub fn get_points(env: Env, user: Address) -> u64 {
        let key = RecyclingKey::Points(user);
        env.storage().persistent().get(&key).unwrap_or(0u64)
    }

    // View recycling history
    pub fn get_log(env: Env, user: Address) -> Vec<RecyclingEntry> {
        let key = RecyclingKey::Log(user);
        env.storage().persistent().get(&key).unwrap_or(Vec::new(&env))
    }
}
