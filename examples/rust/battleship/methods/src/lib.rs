#![no_std]

#[cfg(not(target_arch = "riscv32"))]
pub mod methods {
    const ID_SIZE: usize = 384;

    pub const INIT_ID: &[u8; ID_SIZE] = include_bytes!(concat!(env!("OUT_DIR"), "/init.id"));
    pub const INIT_ELF_PATH: &str = env!("RISC0_INIT_ELF_PATH");

    pub const TURN_ID: &[u8; ID_SIZE] = include_bytes!(concat!(env!("OUT_DIR"), "/turn.id"));
    pub const TURN_ELF_PATH: &str = env!("RISC0_TURN_ELF_PATH");
}
