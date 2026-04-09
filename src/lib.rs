use subxt::lightclient::LightClient;
use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen::prelude::*;

const POLKADOT_SPEC: &str = include_str!("../polkadot.json");
const ASSET_HUB_SPEC: &str = include_str!("../asset_hub.json");

fn now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now() / 1000.0
}

fn mark_block() {
    let window = web_sys::window().unwrap();
    let ms = web_sys::window().unwrap().performance().unwrap().now();
    js_sys::Reflect::set(&window, &"_lastBlockTime".into(), &ms.into()).ok();
}

fn append_block(list: &web_sys::Element, number: u64, hash: impl std::fmt::Display) {
    let doc = web_sys::window().unwrap().document().unwrap();
    let li = doc.create_element("li").unwrap();
    li.set_text_content(Some(&format!("#{number} - {hash}")));
    list.prepend_with_node_1(&li).unwrap();
    mark_block();
}

fn el(id: &str) -> web_sys::Element {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
}

fn query_param(key: &str) -> Option<String> {
    let search = web_sys::window().unwrap().location().search().ok()?;
    let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
    params.get(key)
}

async fn run_light_client() {
    let status = el("status");
    let list = el("blocks");
    let t0 = now();

    status.set_text_content(Some("Starting light client (relay chain)..."));
    let (lc, _relay_rpc) = LightClient::relay_chain(POLKADOT_SPEC).unwrap();

    status.set_text_content(Some(&format!("Adding Asset Hub parachain... ({:.1}s)", now() - t0)));
    let asset_hub_rpc = lc.parachain(ASSET_HUB_SPEC).unwrap();

    status.set_text_content(Some(&format!("Fetching metadata... ({:.1}s)", now() - t0)));
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(asset_hub_rpc)
        .await
        .unwrap();
    let init_time = now() - t0;

    status.set_text_content(Some(&format!("Ready in {:.1}s, syncing...", init_time)));

    let mut blocks = api.stream_best_blocks().await.unwrap();
    let mut first_block_time = None;
    while let Some(Ok(block)) = blocks.next().await {
        let fbt = *first_block_time.get_or_insert_with(|| now() - t0);
        status.set_text_content(Some(&format!(
            "Best: #{} (init {:.1}s, first block {:.1}s)",
            block.number(), init_time, fbt
        )));
        append_block(&list, block.number(), block.hash());
    }
}

async fn run_rpc(url: &str) {
    let status = el("status");
    let list = el("blocks");
    let t0 = now();

    status.set_text_content(Some(&format!("Connecting to {url}...")));
    let api = OnlineClient::<PolkadotConfig>::from_url(url).await.unwrap();
    let init_time = now() - t0;

    status.set_text_content(Some(&format!("Connected in {:.1}s, waiting for blocks...", init_time)));

    let mut blocks = api.stream_best_blocks().await.unwrap();
    let mut first_block_time = None;
    while let Some(Ok(block)) = blocks.next().await {
        let fbt = *first_block_time.get_or_insert_with(|| now() - t0);
        status.set_text_content(Some(&format!(
            "Best: #{} (init {:.1}s, first block {:.1}s)",
            block.number(), init_time, fbt
        )));
        append_block(&list, block.number(), block.hash());
    }
}

#[wasm_bindgen(start)]
pub async fn main() {
    if let Some(url) = query_param("rpc") {
        run_rpc(&url).await;
    } else {
        run_light_client().await;
    }
}
