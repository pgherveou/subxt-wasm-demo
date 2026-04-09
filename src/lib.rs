use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443")
        .await
        .expect("Failed to connect");

    let doc = web_sys::window().unwrap().document().unwrap();
    doc.get_element_by_id("status")
        .unwrap()
        .set_text_content(Some("Connected. Waiting for finalized blocks..."));
    let list = doc.get_element_by_id("blocks").unwrap();

    let mut blocks = api.stream_blocks().await.unwrap();

    while let Some(Ok(block)) = blocks.next().await {
        let number = block.number();
        let hash = block.hash();

        let li = doc.create_element("li").unwrap();
        li.set_text_content(Some(&format!("#{number} - {hash}")));
        list.prepend_with_node_1(&li).unwrap();
    }
}
