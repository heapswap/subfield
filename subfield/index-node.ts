import init, * as sf from "./pkg-node"
export * from "./pkg-node"

// initialize the wasm module
// await init()

// string encoding
const encoder = new TextEncoder()
export const fromString = (str: string) => encoder.encode(str)
const decoder = new TextDecoder()
export const toString = (buf: Uint8Array) => decoder.decode(buf)
