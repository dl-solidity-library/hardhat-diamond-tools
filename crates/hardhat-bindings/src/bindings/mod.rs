//! This module provides binding of Hardhat types to Rust types.

use js_sys::Promise;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod artifacts;
pub mod runtime;

use runtime::HardhatRuntimeEnvironment;

/// The [`ActionType`] bindings for Rust.
///
/// NOTE: First value if the arguments of the task defined by plugin developer, that's why
/// we use `any` (see [`JsValue`]). The third one `runSuper`.
///
/// [`ActionType`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/runtime.ts#L161
pub type ActionType = dyn Fn(JsValue, HardhatRuntimeEnvironment, JsValue) -> Promise;

/// The [`ConfigurableTaskDefinition`] bindings for Rust.
///
/// [`ConfigurableTaskDefinition`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/runtime.ts#L43C18-L43C44
#[wasm_bindgen(module = "hardhat/config")]
extern "C" {
    pub type ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "setDescription")]
    pub fn set_description(
        this: &ConfigurableTaskDefinition,
        description: &str,
    ) -> ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "setAction")]
    pub fn set_action(
        this: &ConfigurableTaskDefinition,
        action: &ActionType,
    ) -> ConfigurableTaskDefinition;
}

#[wasm_bindgen(module = "hardhat/config")]
extern "C" {
    #[wasm_bindgen(js_name = "task")]
    pub fn task(name: &str, description: &str) -> ConfigurableTaskDefinition;
}