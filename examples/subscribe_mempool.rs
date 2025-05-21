use std::sync::Arc;

use alloy::providers::ProviderBuilder;
use alloy::providers::WsConnect;
use burberry::{collector::MempoolCollector, Collector};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let ws = WsConnect::new("wss://eth.merkle.io");
    let provider = ProviderBuilder::new()
        .connect_ws(ws)
        .await
        .expect("fail to create ws provider");

    let collector = MempoolCollector::new(Arc::new(provider));
    let mut stream = collector
        .get_event_stream()
        .await
        .expect("fail to get event stream");

    println!("start to receive tx from mempool");

    while let Some(tx) = stream.next().await {
        println!("received tx: {:?}", tx);
    }
}
