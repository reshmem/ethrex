use crate::account_key::AccountPrivateKey;

pub fn get_address_from_secret_key(
    secret_key_bytes: &[u8],
) -> Result<ethrex_common::Address, String> {
    let key = AccountPrivateKey::from_bytes(secret_key_bytes)
        .map_err(|e| format!("Failed to parse account key: {e}"))?;
    Ok(key.address())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethrex_crypto::slh_dsa::{generate_slh_key, slh_pubkey_to_address};

    #[test]
    fn address_derivation_supports_slh_keys() {
        let (sk, pk) = generate_slh_key();
        let address = match get_address_from_secret_key(&sk.to_bytes()) {
            Ok(value) => value,
            Err(err) => {
                assert!(false, "SLH address derivation failed: {err}");
                return;
            }
        };
        assert_eq!(address, slh_pubkey_to_address(&pk));
    }
}
