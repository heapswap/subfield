use crate::*;
use chacha20poly1305::{
	aead::{generic_array, Aead, AeadCore, KeyInit, OsRng},
	ChaCha20Poly1305,
};

pub type Plaintext = Vec<u8>;
pub type Ciphertext = Vec<u8>;
pub type CipherSecretKey = V256;
pub type CipherSecretKeyArray = [u8; 32];
const NONCE_LENGTH: usize = 12;
pub type Nonce = Vec<u8>; // [u8; NONCE_LENGTH];
pub type NonceArray = [u8; NONCE_LENGTH];

#[derive(Debug)]
pub enum CipherError {
	InvalidNonce,
	InvalidKey,
}

#[wasm_bindgen]
pub struct Cipher {
	secret: V256,
	cipher: ChaCha20Poly1305,
}

impl Cipher {
	/*
	Constructors
	*/

	pub fn new(secret: CipherSecretKey) -> Cipher {
		let key: [u8; 32] = secret.data().clone().try_into().unwrap();
		let cipher = ChaCha20Poly1305::new(&generic_array::GenericArray::<
			u8,
			generic_array::typenum::U32,
		>::from(key));

		Cipher { secret, cipher }
	}

	pub fn random_key() -> CipherSecretKey {
		V256::random256()
	}

	pub fn random() -> Cipher {
		Cipher::new(Cipher::random_key())
	}

	/*
	Getters
	*/
	pub fn secret(&self) -> &CipherSecretKey {
		&self.secret
	}

	/*
	Decrypt
	*/
	pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CipherError> {
		// check that the ciphertext is at least as long as the nonce
		if ciphertext.len() < NONCE_LENGTH {
			return Err(CipherError::InvalidNonce);
		}

		// Extract nonce from the ciphertext
		let nonce: NonceArray = ciphertext[0..NONCE_LENGTH]
			.try_into()
			.map_err(|_| CipherError::InvalidNonce)?;

		// Decryption
		Ok(self
			.cipher
			.decrypt(&nonce.into(), &ciphertext[NONCE_LENGTH..])
			.unwrap())
	}

	/*
	Encrypt
	*/
	pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
		// Generate nonce
		let nonce: NonceArray =
			ChaCha20Poly1305::generate_nonce(&mut OsRng).into();

		// Convert nonce to Vec<u8> using Vec<u8>::copy_from_slice
		let encrypted_data = self
			.cipher
			.encrypt(&nonce.into(), plaintext.to_vec().as_slice())
			.unwrap();

		// Concatenate nonce and encrypted data
		[&nonce, encrypted_data.as_slice()].concat()
	}
}

#[wasm_bindgen]
impl Cipher {
	/*
	Constructors
	*/

	#[wasm_bindgen(constructor)]
	pub fn _js_new(secret: CipherSecretKey) -> Cipher {
		Cipher::new(secret)
	}

	#[wasm_bindgen(js_name = "randomKey")]
	pub fn _js_random_key() -> CipherSecretKey {
		Cipher::random_key()
	}

	#[wasm_bindgen(js_name = "random")]
	pub fn _js_random() -> Cipher {
		Cipher::random()
	}

	/*
	Getters
	*/
	#[wasm_bindgen(getter, js_name = "key")]
	pub fn _js_key(&self) -> CipherSecretKey {
		self.secret().clone()
	}

	/*
	Decrypt
	*/

	#[wasm_bindgen(js_name = "decrypt")]
	pub fn _js_decrypt(&self, ciphertext: Uint8Array) -> Uint8Array {
		self.decrypt(&ciphertext.to_vec())
			.map(|plaintext| plaintext.as_slice().into())
			.unwrap_or_else(|_| panic!("Invalid ciphertext"))
	}

	/*
	Encrypt
	*/

	#[wasm_bindgen(js_name = "encrypt")]
	pub fn _js_encrypt(&self, plaintext: Uint8Array) -> Uint8Array {
		self.encrypt(&plaintext.to_vec()).as_slice().into()
	}
}
