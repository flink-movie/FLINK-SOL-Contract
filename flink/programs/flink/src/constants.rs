/// 10 000 basis-points = 100 %
pub const BPS_DENOMINATOR: u16 = 10_000;

/// bump seeds (match PDA derivations in `utils`)
pub const FILM_SEED: &[u8]  = b"film";
pub const EXTRA_SEED: &[u8] = b"extra";
pub const SALE_SEED: &[u8]  = b"sale";

/// Account seeds for escrow vaults
pub const PLATFORM_VAULT_SEED: &[u8] = b"platform_vault";
pub const CREATOR_VAULT_SEED:  &[u8] = b"creator_vault";
