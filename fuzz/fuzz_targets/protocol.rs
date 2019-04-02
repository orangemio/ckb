#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate ckb_protocol;

use ckb_protocol::{
    get_root, Block as FbsBlock, Bytes as FbsBytes, CellInput as FbsCellInput,
    CellOutput as FbsCellOutput, CompactBlock, FilteredBlock, GetBlocks as FbsGetBlocks,
    GetHeaders as FbsGetHeaders, Header as FbsHeader, Headers as FbsHeaders,
    OutPoint as FbsOutPoint, ProposalShortId as FbsProposalShortId, RelayMessage, RelayPayload,
    Script as FbsScript, SyncMessage, SyncPayload, Time as FbsTime, TimeMessage,
    Transaction as FbsTransaction, UncleBlock as FbsUncleBlock,
    ValidTransaction as FbsValidTransaction
};

macro_rules! convert {
    ($target:ty, $data:expr) => {
        let _ = get_root::<$target>($data);
    };
}

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    convert!(FbsBlock, data);
});
