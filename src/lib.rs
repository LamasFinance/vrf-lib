use anchor_lang::{prelude::*, InstructionData};

#[event]
pub struct RequestVrf {
    pub ix_sighash: [u8; 8],
    pub ix_data: Vec<u8>,
    pub accounts: Vec<AccountMetaRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub struct AccountMetaRef {
    pub pubkey: Pubkey,
    pub is_writable: bool,
}

impl AccountMetaRef {
    pub fn mutable(mut self) -> Self {
        self.is_writable = true;
        self
    }
}

pub fn account_meta(pubkey: &impl anchor_lang::Key) -> AccountMetaRef {
    AccountMetaRef {
        pubkey: pubkey.key(),
        is_writable: false,
    }
}

pub fn request_random<T: InstructionData>(ix: T, accounts: Vec<AccountMetaRef>) {
    let data = ix.data();

    emit!(RequestVrf {
        ix_sighash: data[0..8].try_into().unwrap(),
        ix_data: data[8..].to_vec(),
        accounts
    });
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VrfResult {
    pub random: [u8; VrfResult::RANDOM_BYTE_LEN],
    pub request_transaction: [u8; VrfResult::SIGNATURE_BYTE_LEN],
}

impl Default for VrfResult {
    fn default() -> Self {
        Self {
            random: [0u8; VrfResult::RANDOM_BYTE_LEN],
            request_transaction: [0u8; VrfResult::SIGNATURE_BYTE_LEN],
        }
    }
}

impl VrfResult {
    pub const RANDOM_BYTE_LEN: usize = 16;
    pub const SIGNATURE_BYTE_LEN: usize = 64;
}
