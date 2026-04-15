mod platform;

use platform::SubxtPlatform;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

/// A smoldot-based JSON-RPC provider for polkadot-api, fully implemented in WASM.
///
/// Usage from JS:
/// ```js
/// const provider = await WasmProvider.new(relaySpec, paraSpec);
/// const jsonRpcProvider = (onMessage) => {
///     provider.start((resp) => onMessage(JSON.parse(resp)));
///     return {
///         send: (msg) => provider.send(JSON.stringify(msg)),
///         disconnect: () => provider.disconnect(),
///     };
/// };
/// const client = createClient(jsonRpcProvider);
/// ```
#[wasm_bindgen]
pub struct WasmProvider {
    client: Arc<Mutex<smoldot_light::Client<SubxtPlatform>>>,
    para_chain_id: smoldot_light::ChainId,
    relay_chain_id: smoldot_light::ChainId,
    responses: Option<smoldot_light::JsonRpcResponses<SubxtPlatform>>,
}

#[wasm_bindgen]
impl WasmProvider {
    /// Initialize smoldot with a relay chain and parachain.
    /// Returns a promise that resolves to a WasmProvider.
    #[wasm_bindgen(constructor)]
    pub fn new(relay_spec: &str, para_spec: &str) -> Result<WasmProvider, JsError> {
        let mut client = smoldot_light::Client::new(SubxtPlatform::new());

        let relay = client.add_chain(smoldot_light::AddChainConfig {
            specification: relay_spec,
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Disabled,
            database_content: "",
            potential_relay_chains: std::iter::empty(),
            user_data: (),
            statement_protocol_config: None,
        }).map_err(|e| JsError::new(&format!("Failed to add relay chain: {e}")))?;

        let para = client.add_chain(smoldot_light::AddChainConfig {
            specification: para_spec,
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: std::num::NonZero::new(u32::MAX).unwrap(),
                max_subscriptions: u32::MAX,
            },
            database_content: "",
            potential_relay_chains: std::iter::once(relay.chain_id),
            user_data: (),
            statement_protocol_config: None,
        }).map_err(|e| JsError::new(&format!("Failed to add parachain: {e}")))?;

        Ok(WasmProvider {
            client: Arc::new(Mutex::new(client)),
            relay_chain_id: relay.chain_id,
            para_chain_id: para.chain_id,
            responses: para.json_rpc_responses,
        })
    }

    /// Start streaming JSON-RPC responses to the given callback.
    /// The callback receives raw JSON strings.
    pub fn start(&mut self, on_response: js_sys::Function) {
        let mut responses = self.responses.take()
            .expect("start() must only be called once");

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(response) = responses.next().await {
                let _ = on_response.call1(
                    &JsValue::NULL,
                    &JsValue::from_str(&response),
                );
            }
        });
    }

    /// Send a JSON-RPC request (as a raw JSON string).
    pub fn send(&self, request: &str) {
        let mut client = self.client.lock().expect("Mutex is poisoned");
        if let Err(e) = client.json_rpc_request(request, self.para_chain_id) {
            web_sys::console::error_1(
                &format!("json_rpc_request failed: {e}").into(),
            );
        }
    }

    /// Disconnect and clean up both chains.
    pub fn disconnect(&self) {
        let mut client = self.client.lock().expect("Mutex is poisoned");
        let _ = client.remove_chain(self.para_chain_id);
        let _ = client.remove_chain(self.relay_chain_id);
    }
}
