use crate::utils::wait_until;
use crate::{Net, Spec};
use ckb_core::transaction::{CellInput, OutPoint, TransactionBuilder};
use ckb_core::Capacity;
use log::info;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub struct TransactionRelayBasic;

impl Spec for TransactionRelayBasic {
    fn run(&self, net: Net) {
        info!("Running TransactionRelayBasic");
        let node0 = &net.nodes[0];
        let node1 = &net.nodes[1];
        let node2 = &net.nodes[2];
        // generate 1 block to exit IBD mode.
        let block = node0.new_block(None, None, None);
        node0.submit_block(&block);
        node1.submit_block(&block);
        node2.submit_block(&block);

        info!("Generate new transaction on node1");
        node1.generate_block();
        let hash = node1.generate_transaction();

        info!("Waiting for relay");
        let mut rpc_client = node0.rpc_client();
        let ret = wait_until(10, || {
            if let Some(transaction) = rpc_client.get_transaction(hash.clone()).call().unwrap() {
                transaction.tx_status.block_hash.is_none()
            } else {
                false
            }
        });
        assert!(ret, "Transaction should be relayed to node0");

        let mut rpc_client = node2.rpc_client();
        let ret = wait_until(10, || {
            if let Some(transaction) = rpc_client.get_transaction(hash.clone()).call().unwrap() {
                transaction.tx_status.block_hash.is_none()
            } else {
                false
            }
        });
        assert!(ret, "Transaction should be relayed to node2");
    }
}

const TXS_NUM: usize = 500;

pub struct TransactionRelayMultiple;

impl Spec for TransactionRelayMultiple {
    fn run(&self, net: Net) {
        info!("Running TransactionRelayMultiple");

        let node0 = &net.nodes[0];
        // generate 1 block to exit IBD mode.
        let block = node0.new_block(None, None, None);
        net.nodes.iter().for_each(|node| {
            node.submit_block(&block);
        });

        info!("Use generated block's cellbase as tx input");
        let parent_hash = block.transactions()[0].hash().to_owned();
        let temp_transaction = node0.new_transaction(parent_hash);
        let mut output = temp_transaction.outputs()[0].clone();
        output.capacity = Capacity::shannons(output.capacity.as_u64() / TXS_NUM as u64);
        let mut tb = TransactionBuilder::from_transaction(temp_transaction).outputs_clear();
        for _ in 0..TXS_NUM {
            tb = tb.output(output.clone());
        }
        let transaction = tb.build();
        node0
            .rpc_client()
            .send_transaction((&transaction).into())
            .call()
            .expect("rpc call send_transaction failed");
        node0.generate_block();
        node0.generate_block();
        node0.generate_block();
        net.waiting_for_sync(10);

        info!("Send multiple transactions to node0");
        let tx_hash = transaction.hash().to_owned();
        transaction
            .outputs()
            .par_iter()
            .enumerate()
            .for_each(|(i, output)| {
                let tx = TransactionBuilder::default()
                    .dep(transaction.deps()[0].clone())
                    .output(output.clone())
                    .input(CellInput::new(
                        OutPoint::new_cell(tx_hash.clone(), i as u32),
                        0,
                        vec![],
                    ))
                    .build();
                node0
                    .rpc_client()
                    .send_transaction((&tx).into())
                    .call()
                    .expect("rpc call send_transaction failed");
            });

        node0.generate_block();
        node0.generate_block();
        node0.generate_block();
        net.waiting_for_sync(30);

        info!("All transactions should be relayed and mined");
        node0.assert_tx_pool_size(0, 0);

        net.nodes
            .iter()
            .for_each(|node| assert_eq!(node.get_tip_block().transactions().len(), TXS_NUM + 1));
    }

    fn num_nodes(&self) -> usize {
        5
    }
}
