use ain_evm::{bytes::Bytes, log::LogIndex};
use ethereum_types::{H160, H256, U256};
use serde_with::{serde_as, OneOrMany};

use crate::block::BlockNumber;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LogResult {
    pub address: H160,
    pub block_hash: H256,
    pub block_number: U256,
    pub data: Bytes,
    pub log_index: U256,
    pub removed: bool,
    pub topics: Vec<H256>,
    pub transaction_hash: H256,
    pub transaction_index: U256,
}

impl From<LogIndex> for LogResult {
    fn from(log: LogIndex) -> Self {
        Self {
            address: log.address,
            block_hash: log.block_hash,
            block_number: log.block_number,
            data: Bytes::from(log.data),
            log_index: log.log_index,
            removed: log.removed,
            topics: log.topics,
            transaction_hash: log.transaction_hash,
            transaction_index: log.transaction_index,
        }
    }
}

/// Call request
#[serde_as]
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsRequest {
    #[serde_as(as = "Option<OneOrMany<_>>")]
    pub address: Option<Vec<H160>>,
    pub block_hash: Option<H256>,
    pub from_block: Option<BlockNumber>,
    pub to_block: Option<BlockNumber>,
    pub topics: Option<Vec<Option<H256>>>,
}
