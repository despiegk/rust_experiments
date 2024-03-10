use secp256k1::{Secp256k1, Message, schnorrsig::{PublicKey, KeyPair}, rand::thread_rng};
use sha2::{Sha256, Digest};

fn main() {
    let secp = Secp256k1::new();
    let mut rng = thread_rng();

    // Generate key pairs for two participants
    let key_pair1 = KeyPair::new(&secp, &mut rng);
    let key_pair2 = KeyPair::new(&secp, &mut rng);

    // Compute public keys for the key pairs
    let pub_key1 = PublicKey::from_keypair(&secp, &key_pair1);
    let pub_key2 = PublicKey::from_keypair(&secp, &key_pair2);

    // Hash the message using SHA-256
    let message = "This is a very important message.";
    let hasher = Sha256::new().chain_update(message.as_bytes());
    let message_hash_result = hasher.finalize();
    let message_hash = Message::from_slice(&message_hash_result).expect("32 bytes");

    // Rest of your code...
}
