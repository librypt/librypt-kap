/// A Key Agreement Protocol (KAP).
pub trait KeyAgreementProtocol {
    /// Custom type for shared data to facilitate the key exchange.
    type SharedKey;

    /// Custom type for the resulting shared secret.
    ///
    /// WARNING: The computed secret should **not** be used directly as key material, but rather fed through a Key Derivation Function (KDF).
    type SharedSecret: AsRef<[u8]>;

    /// Compute the public/shared parts of the ephemeral key.
    fn shared_key(&self) -> Self::SharedKey;

    /// Compute the shared secret given an external public/shared key.
    fn compute_shared_secret(&self, shared_key: &Self::SharedKey) -> Self::SharedSecret;
}
