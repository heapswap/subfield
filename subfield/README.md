# Subfield

This package holds the WASM functions for the [Subfield Protocol](https://subfield.org).

# Installation

```bash
npm install subfield
```

# Usage

```typescript
import * as sf from "subfield/web" // for web
import * as sf from "subfield/node" // for node

await sf.init() // Initialize the WASM module, only necessary if the environment does not support top-level await
```

# Features

## VersionedBytes
Hash, PublicKey, PrivateKey, SharedSecret, and CipherKey are all based on VersionedBytes. This allows, for example, a Keypair's SharedKey to be used as a CipherKey for the Cipher functions.

```typescript
// Constructors
const vb = new sf.VersionedBytes(
	version: number, 
	bytes: Uint8Array
	): VersionedBytes
const vb = sf.VersionedBytes.random(): VersionedBytes

// Getters
vb.version: Number
vb.data: Uint8Array // length of 32

// Conversions
vb.toBytes(): Uint8Array
vb.fromBytes(bytes: Uint8Array): VersionedBytes
vb.toString(): string // Base32
vb.fromString(str: string): VersionedBytes
```

## Crypto

### Hashing (BLAKE3)
```typescript
// Usage
sf.hash(bytes: Uint8Array): Hash
sf.verifyHash(bytes: Uint8Array, hash: Hash): boolean
```

### Cipher (CHA-CHA20)
```typescript
// Instantiation
const cipherKey: CipherKey = sf.Cipher.randomKey()
const cipher: Cipher = new sf.Cipher(cipherKey)

// Getters
cipher.key: CipherKey 

// Usage
cipher.encrypt(secret: CipherKey, message: Uint8Array): Uint8Array
cipher.decrypt(secret: CipherKey, encrypted: Uint8Array): Uint8Array
```

### Signatures (Ed25519)
```typescript
// Instantiation
const keypair = new sf.Keypair(private_key: PrivateKey)
const keypair = sf.Keypair.random(): Keypair
const keypair = await sf.Keypair.vanity(prefix: string): Keypair 
	// Prefix must be valid base32, or will throw an error
	// Vanity Keypair search times (single-threaded):
	// On average, requires 2^(letters * 5 - 1) address generations,
	// so each added letter increases average generations by 32x
	// 1 character  ~0s  (2^4  generations)
	// 2 characters ~1s  (2^9  generations)
	// 3 characters ~1m  (2^14 generations) 
	// 4 characters ~20m (2^19 generations)
	// 5 characters ~10h (2^24 generations)

// Getters
keypair.publicKey.edwards(): PublicKey
keypair.publicKey.montgomery(): PublicKey
keypair.privateKey.edwards(): PrivateKey
keypair.privateKey.montgomery(): PrivateKey

// Signatures
keypair.sign(message: Uint8Array): Signature
keypair.verify(message: Uint8Array, signature: Signature): boolean

// Shared secret
keypair.sharedSecret(publicKey: PublicKey): SharedKey
```

### Noise (Noise_NN_25519_ChaChaPoly_BLAKE2s)

```typescript
// Instantation

// The client creates a Noise initiator
const initiator: Noise = sf.Noise.initiator()
const initiator: Noise = sf.Noise.initiatorFromKeypair(keypair: Keypair)
// The server creates a Noise responder
const responder: Noise = sf.Noise.responder()
const responder: Noise = sf.Noise.responderFromKeypair(keypair: Keypair)

// Setup

// step 1 : initiator sends message1 to responder
let message1 = initiator.handshakeStep1()

// step 2 : responder processes message1 and sends message2 to initiator
let message2 = responder.handshakeStep2(message1)

// step 3 : initiator processes message2
initiator.handshakeStep3(message2)

// Setup done!

// Usage

const helloMessage: Uint8Array = sf.fromString("hello")

// initiator -> responder
let encrypted: Uint8Array = initiator.encrypt(helloMessage)
let decrypted: Uint8Array = responder.decrypt(encrypted)

// responder -> initiator
let encrypted: Uint8Array = responder.encrypt(helloMessage)
let decrypted: Uint8Array = initiator.decrypt(encrypted)
```

## Misc

```typescript
// String encoding
sf.fromString(str: string): Uint8Array
sf.toString(bytes: Uint8Array): string

// Base32 encoding
sf.fromBase32(str: string): Uint8Array
sf.toBase32(bytes: Uint8Array): string
```
