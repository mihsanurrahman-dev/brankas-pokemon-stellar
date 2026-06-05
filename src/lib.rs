#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const KARTU: Symbol = symbol_short!("KARTU");

#[contract]
pub struct BrankasPokemon;

#[contractimpl]
impl BrankasPokemon {

    pub fn tambah_kartu(env: Env, jumlah: u32) -> u32 {
        let mut total: u32 = env.storage().instance().get(&KARTU).unwrap_or(0);
        total += jumlah;
        env.storage().instance().set(&KARTU, &total);
        total
    }

    pub fn lihat_isi(env: Env) -> u32 {
        env.storage().instance().get(&KARTU).unwrap_or(0)
    }
}
