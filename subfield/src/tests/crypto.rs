use crate::*;

/*
   Cipher
*/
#[test]
fn test_cipher() {
	let cipher = Cipher::random();
	let plaintext = b"hello world";
	let ciphertext = cipher.encrypt(plaintext);
	let decrypted = cipher.decrypt(&ciphertext).unwrap();
	assert_eq!(plaintext.to_vec(), decrypted);
}

/*
   Noise
*/
#[test]
fn test_noise() {
	let mut initiator = Noise::initiator();
	let mut responder = Noise::responder();

	// handshake
	let initiator_message = initiator.handshake_step_1().unwrap();
	let responder_message =
		responder.handshake_step_2(&initiator_message).unwrap();
	let _ = initiator.handshake_step_3(&responder_message).unwrap();

	// encrypt from initiator to responder
	let data = b"hello world";
	let encrypted = initiator.encrypt(data).unwrap();
	let decrypted = responder.decrypt(&encrypted).unwrap();
	assert_eq!(data.to_vec(), decrypted);

	// encrypt from responder to initiator
	let data = b"hello world";
	let encrypted = responder.encrypt(data).unwrap();
	let decrypted = initiator.decrypt(&encrypted).unwrap();
	assert_eq!(data.to_vec(), decrypted);

	// test a large message
	let data = vec![0; 1024 * 1024];
	let encrypted = initiator.encrypt(&data).unwrap();
	let decrypted = responder.decrypt(&encrypted).unwrap();
	assert_eq!(data, decrypted);
}

/*
   Keypair
*/
#[test]
fn test_sign_and_verify() {
	let keypair = Keypair::random();
	let message = b"hello world";
	let signature = keypair.sign(message);
	assert!(keypair.public_key().verify(message, &signature).unwrap());
}

#[test]
fn test_shared_secret() {
	let alice = Keypair::random();
	let bob = Keypair::random();
	let alice_shared_secret = alice.shared_secret(&bob.public_key());
	let bob_shared_secret = bob.shared_secret(&alice.public_key());
	assert_eq!(alice_shared_secret, bob_shared_secret);
}

#[test]
fn test_vanity() {
	let prefix = "aa";
	let keypair = Keypair::vanity(prefix).unwrap();
	assert!(keypair.public_key().to_string().starts_with(prefix));
}