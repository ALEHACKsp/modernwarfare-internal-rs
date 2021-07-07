use std::time::{Duration, Instant};

use crate::util::Global;
use crate::memory::MEMORY;
use crate::sdk;

pub struct Decryption {
    pub client_info: Option<u64>,
    pub client_base: Option<u64>,
    pub bone_base: Option<u64>,
    pub last_update: Instant,
}

impl Default for Decryption {
    fn default() -> Self {
        Self {
            client_info: None,
            client_base: None,
            bone_base: None,
            last_update: unsafe { std::mem::zeroed() },
        }
    }
}

pub static DECRYPTION: Global<Decryption> = Global::new();

extern "C" {
    fn decrypt_client_info(image_base: u64, peb: u64) -> u64;
    fn decrypt_client_base(client_info: u64, image_base: u64, peb: u64) -> u64;
}

impl Decryption {
    pub unsafe fn update(&mut self) {
        if self.last_update.elapsed() < Duration::from_secs(1) {
            return;
        }
        if self.valid() && sdk::get_camera().is_some() {
            // We already have valid so we don't need to update again
            return;
        }
        self.last_update = Instant::now();

        let client_info = decrypt_client_info(MEMORY.image_base, MEMORY.peb);
        if client_info == 0 {
            self.reset();
            return;
        }
        self.client_info = Some(client_info);

        let client_base = decrypt_client_base(client_info, MEMORY.image_base, MEMORY.peb);
        if client_base == 0 {
            self.client_base = None;
            return;
        }
        self.client_base = Some(client_base);
    }

    pub fn valid(&self) -> bool {
        self.client_info.is_some() && self.client_base.is_some()
    }

    pub fn valid_bone(&self) -> bool {
        self.valid() && self.bone_base.is_some()
    }

    fn reset(&mut self) {
        self.client_info = None;
        self.client_base = None;
        self.bone_base = None;
    }
}