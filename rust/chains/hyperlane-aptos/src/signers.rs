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