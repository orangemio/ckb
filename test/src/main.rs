use ckb_test::specs::*;
use logger::{self, Config};
use std::env;

fn main() {
    let log_config = Config {
        filter: Some("info".to_owned()),
        ..Default::default()
    };
    let _logger_guard = logger::init(log_config).expect("init Logger");

    let binary = env::args()
        .nth(1)
        .unwrap_or_else(|| "../target/release/ckb".to_string());
    let start_port = env::args()
        .nth(2)
        .unwrap_or_else(|| "9000".to_string())
        .parse()
        .expect("invalid port number");
    if let Some(spec_name) = env::args().nth(3) {
        let spec: Box<Spec> = match &spec_name[..] {
            "block_relay_basic" => Box::new(BlockRelayBasic),
            "block_sync_basic" => Box::new(BlockSyncBasic),
            "chain_fork_1" => Box::new(ChainFork1),
            "chain_fork_2" => Box::new(ChainFork2),
            "mining_basic" => Box::new(MiningBasic),
            "mining_template_size_limit" => Box::new(TemplateSizeLimit),
            "pool_reconcile" => Box::new(PoolReconcile),
            "pool_resurrect" => Box::new(PoolResurrect),
            "transaction_relay_basic" => Box::new(TransactionRelayBasic),
            "transaction_relay_multiple" => Box::new(TransactionRelayMultiple),
            "discovery" => Box::new(Discovery),
            "disconnect" => Box::new(Disconnect),
            "malformed_message" => Box::new(MalformedMessage),
            "depent_tx_in_same_block" => Box::new(DepentTxInSameBlock),
            // TODO enable these after staging/pending pool tip verfiry logic changing
            // "cellbase_maturity" => Box::new(CellbaseMaturity),
            // "valid_since" => Box::new(ValidSince),
            "different_txs_with_same_input" => Box::new(DifferentTxsWithSameInput),
            "compact_block_basic" => Box::new(CompactBlockBasic),
            "invalid_locator_size" => Box::new(InvalidLocatorSize),
            _ => panic!("invalid spec"),
        };
        let net = spec.setup_net(&binary, start_port);
        spec.run(net);
    } else {
        let specs: Vec<Box<Spec>> = vec![
            Box::new(BlockRelayBasic),
            Box::new(BlockSyncBasic),
            Box::new(ChainFork1),
            Box::new(ChainFork2),
            Box::new(MiningBasic),
            Box::new(TemplateSizeLimit),
            Box::new(PoolReconcile),
            Box::new(PoolResurrect),
            Box::new(TransactionRelayBasic),
            Box::new(TransactionRelayMultiple),
            Box::new(Discovery),
            Box::new(Disconnect),
            Box::new(MalformedMessage),
            Box::new(DepentTxInSameBlock),
            // TODO enable these after staging/pending pool tip verfiry logic changing
            // Box::new(CellbaseMaturity),
            // Box::new(ValidSince),
            Box::new(DifferentTxsWithSameInput),
            Box::new(CompactBlockBasic),
            Box::new(InvalidLocatorSize),
        ];

        specs.iter().for_each(|spec| {
            let net = spec.setup_net(&binary, start_port);
            spec.run(net);
        })
    }
}
