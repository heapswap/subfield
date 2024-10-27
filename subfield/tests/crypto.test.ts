import * as sf from "../index-web.ts"
import { expect, test } from "bun:test"

test("hash", async () => {
	const hash = sf.hash("hello")

	const verify = sf.hashVerify("hello", hash)

	expect(verify).toBe(true)
})

test("cipher", async () => {
	const cipherKey = sf.Cipher.randomKey()

	const cipher = new sf.Cipher(cipherKey)

	
	const encrypted = cipher.encrypt(sf.fromString("hello"))

	const decrypted = cipher.decrypt(encrypted)

	expect(sf.toString(decrypted)).toBe("hello")
})

test("keypair - sign/verify/shared", async () => {
	const alice = sf.Keypair.random()
	const bob = sf.Keypair.random()

	// Alice signs a message
	const message = sf.fromString("hello")
	const signature = alice.sign(message)

	// Bob verifies the message
	expect(alice.publicKey.verify(message, signature)).toBe(true)
	expect(bob.publicKey.verify(message, signature)).toBe(false)

	// compute shared secret
	const aliceShared = alice.sharedSecret(bob.publicKey)
	const bobShared = bob.sharedSecret(alice.publicKey)

	expect(aliceShared.toString()).toEqual(bobShared.toString())
})

test("keypair - vanity", async () => {
	// 1 character - Instant
	// 2 characters - <1s
	// 3 characters - <1m
	// 4 characters - <20m
	// 5 characters - <10h
	const prefix = "aa"
	const keypair = await sf.Keypair.vanity(prefix)
	expect(keypair.publicKey.toString().slice(0, prefix.length)).toBe(prefix)
})

test("keypair - serialization", async () => {
	let keypair = sf.Keypair.random()
	let serialized = keypair.toBytes()
	let deserialized = sf.Keypair.fromBytes(serialized)
	expect(keypair.toString()).toBe(deserialized.toString())
})
