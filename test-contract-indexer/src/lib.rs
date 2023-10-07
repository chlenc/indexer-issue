extern crate alloc;
use fuel_indexer_utils::prelude::*;

#[indexer(manifest = "test_contract_indexer.manifest.yaml")]
pub mod test_contract_indexer_index_mod {

    fn handle_block(block: BlockData) {
        let txs = block.transactions.len();
        info!("🧱 Block height: {} | transacrions: {txs}", block.height);
    }


    fn handle_test_event(event: TestEvent) {
        info!("✨ Test event {:#?}", event);
        let event = TestEventEntity::new(event.n);
        event.save();
    }


}
