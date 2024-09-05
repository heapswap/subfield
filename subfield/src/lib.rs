
/*
   Prelude
*/
mod crypto;
mod misc;

pub mod prelude {
	pub use crate::crypto::*;
	pub use crate::misc::*;
}
pub use crate::prelude::*;

#[cfg(test)]
pub mod tests;

/*
   Reexports
*/
pub use bytes::{Buf, BufMut, Bytes, BytesMut};
pub use bincode::{serialize, deserialize};
pub use either::Either;
pub use futures::prelude::*;
pub use futures::{AsyncReadExt, AsyncWriteExt, FutureExt, SinkExt, StreamExt};
pub use getset::{CopyGetters, Getters, MutGetters, Setters};
pub use itertools::Itertools;
pub use js_sys::Uint8Array;
pub use lazy_static::lazy_static;
pub use once_cell::sync::{Lazy, OnceCell};
pub use rand::{prelude::*, random, thread_rng, Rng};
pub use serde::{
	de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer,
};
pub use std::collections::{HashMap, HashSet};
pub use std::fmt;
pub use std::pin::Pin;
pub use std::sync::Arc;
pub use strum;
pub use tracing;
pub use wasm_bindgen::prelude::*;

/*
   WASM Setup
*/
use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;
use tracing_wasm::{WASMLayer, WASMLayerConfig};

pub fn try_set_as_global_default_with_config(
	config: WASMLayerConfig,
) -> Result<(), SetGlobalDefaultError> {
	tracing::subscriber::set_global_default(
		Registry::default().with(WASMLayer::new(config)),
	)
}

/*
   WASM Entrypoint
*/
#[wasm_bindgen(start)]
pub async fn main() {
	console_error_panic_hook::set_once();
	let level = tracing::Level::INFO;
	let tracing_cfg = tracing_wasm::WASMLayerConfigBuilder::new()
		.set_max_level(level)
		.build();
	let _ = try_set_as_global_default_with_config(tracing_cfg);
}