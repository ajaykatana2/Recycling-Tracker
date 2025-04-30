# Recycling Tracker

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

## Project Title

**Recycling Tracker**

## Project Description

A decentralized solution to track and reward recycling behavior using blockchain. Users earn points based on the amount and type of material they recycle.

## Project Vision

To incentivize eco-friendly habits by making recycling transparent, verifiable, and rewarding—powered by the Soroban smart contract platform.

## Key Features

- ♻️ **Log Recycling**: Users record recycling actions with material type and weight.
- 🎯 **Earn Points**: Automatically calculate reward points per recycling action.
- 📜 **View History**: Track personal recycling log over time.
- 🔒 **Immutable Data**: Records stored securely on-chain.

## Contract Details

### Contract Address: CCZQJZSBVXWXYWQG7SQ676IEYKB65ZZ3R7HUOM3U36SD22PYBDHSAXN6

<img width="1425" alt="image" src="https://github.com/user-attachments/assets/c349d23e-2417-4737-965b-0f5e1ae83cf8" />


### 1. `log_recycling(user, material, weight_kg)`
- Logs a recycling event and rewards points.
- 10 points per kilogram recycled (can be adjusted).
- Requires user authentication.

### 2. `get_points(user) -> u64`
- Returns the total recycling points accumulated by the user.

### 3. `get_log(user) -> Vec<RecyclingEntry>`
- Returns the full recycling history for a user.

Each `RecyclingEntry` includes:
- `material`: Type of recycled material (e.g., plastic, paper).
- `weight_kg`: Amount recycled in kilograms.
- `timestamp`: When the recycling occurred.
- `points_earned`: Points awarded for the entry.

---

**Make an impact. Get rewarded.**  
Track your green journey on the blockchain. 🌍♻️  
Built with [Soroban](https://soroban.stellar.org).
