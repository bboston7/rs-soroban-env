#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env, U256};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // TODO: Run rust formatter on this file
    // TODO: Update this comment
    // This will write num_write_entries of size size_kilo_bytes. The initial entry will have a key of
    // u32(0), the next key u32(1), etc.
    pub fn do_work(
        e: Env,
        guest_cycles: u64,
        host_cycles: u64,
        num_write_entries: u32,
        rw_size_bytes: u32,
        additional_read_entries: u32
    ) -> U256 {
        if rw_size_bytes == 0 && num_write_entries != 0 {
            panic!("size_bytes must be greater than 0");
        }

        let mut slice = [0_u8; 1024];
        e.prng().fill(&mut slice);
        let mut bytes = Bytes::new(&e);
        let size_kilo_bytes = rw_size_bytes / 1024;
        for _ in 0..size_kilo_bytes {
            bytes.extend_from_slice(&slice);
        }
        let remainder = rw_size_bytes % 1024;
        if remainder > 0 {
            // TODO: Does this need the "as usize" cast?
            bytes.extend_from_slice(&slice[0..remainder as usize]);
        }

        for i in 0..num_write_entries {
            e.storage().persistent().set(&i, &bytes);
        }

        for i in 0..additional_read_entries {
            e.storage().persistent().get::<u32, Bytes>(&(i % num_write_entries));
        }

        let mut val: u32 = 0;
        for _ in 0..guest_cycles {
            if u32::MAX == val {
                val = 0;
            }

            val += 1
        }

        let mut u256_val = U256::from_u32(&e, val);
        let u256_1 = U256::from_u32(&e, 1);
        for _ in 0..host_cycles {
            u256_val = u256_val.add(&u256_1);
        }

        // Return has data dependency on both values to make sure nothing gets optimized out
        u256_val
    }
}
