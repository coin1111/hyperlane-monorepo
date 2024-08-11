use aptos_sdk::crypto::ed25519::Ed25519PublicKey;
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::types::transaction::authenticator::AuthenticationKey;
use solana_sdk::signer::keypair::Keypair;

#[derive(Debug)]
/// Signer for aptos chain
pub struct AptosSigner(pub Keypair);

impl AptosSigner {
    /// create new signer
    pub fn new(keypair: Keypair) -> Self {
        AptosSigner(keypair)
    }
}

/// aptos uses a special way to generate account address from public key
/// the code is borrowed from aptos_cli code
pub fn account_address_from_public_key(public_key: &Ed25519PublicKey) -> AccountAddress {
    let auth_key = AuthenticationKey::ed25519(public_key);
    AccountAddress::new(*auth_key.account_address())
}

#[cfg(test)]
mod tests {
    use crate::signers::account_address_from_public_key;
    use aptos_sdk::crypto::ValidCryptoMaterialStringExt;

    #[test]
    fn test_private_key_to_public_key() {
        use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
        use aptos_sdk::crypto::PrivateKey;

        // Replace with your actual private key in hex format
        let private_key_hex = "8cb68128b8749613f8df7612e4efd281f8d70f6d195c53a14c27fc75980446c1";

        // Convert the hex string to a private key
        let private_key = Ed25519PrivateKey::from_encoded_string(private_key_hex)
            .expect("Failed to create private key from hex string");

        // Derive the public key from the private key
        let public_key = private_key.public_key();
        println!("Public Key: {}", hex::encode(&public_key.to_bytes()));

        // Derive the account address from the public key

        let account_address = account_address_from_public_key(&public_key);

        // Print the account address
        println!("Account Address: {}", account_address);
        let expected_account_address =
            "0x8b4376073a408ece791f4adc34a8afdde405bae071711dcbb95ca4e5d4f26c93";
        assert_eq!(account_address.to_string(), expected_account_address);
    }
}
