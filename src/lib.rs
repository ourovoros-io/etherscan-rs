use std::error::Error;

use futures::Future;
use serde::{Deserialize, Serialize};
use web3::types::U256;

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EtherscanModule {
    Unknown,
    Account,
    Contract,
    Transaction,
    Block,
    Stats,
}

impl Default for EtherscanModule {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum EtherscanSort {
    #[serde(rename = "asc")]
    Ascending,

    #[serde(rename = "desc")]
    Descending,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EtherscanAction {
    Unknown,

    Balance,
    BalanceMulti,
    TxList,
    TxListInternal,
    TokenTx,
    TokenNftTx,
    TokenBalance,

    GetABI,
    GetSourceCode,

    GetStatus,
    GetTxReceiptStatus,

    GetBlockReward,
    GetBlockCountdown,
    GetBlockNoByTime,

    TokenSupply,
    EthSupply,
    EthSupply2,
    EthPrice,
    NodeCount,
}

impl Default for EtherscanAction {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EtherscanTag {
    Latest,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EtherscanFormat {
    Raw,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct EtherscanRequest {
    url: Option<String>,
    module_action: Option<(EtherscanModule, EtherscanAction)>,
    contractaddress: Option<U256>,
    address: Option<Vec<U256>>,
    tag: Option<EtherscanTag>,
    startblock: Option<U256>,
    endblock: Option<U256>,
    page: Option<U256>,
    offset: Option<U256>,
    sort: Option<EtherscanSort>,
    txhash: Option<U256>,
    blockno: Option<U256>,
    timestamp: Option<U256>,
    format: Option<EtherscanFormat>,
    apikey: Option<String>,
}

impl Default for EtherscanRequest {
    fn default() -> Self {
        Self {
            url: Default::default(),
            module_action: Default::default(),
            contractaddress: Default::default(),
            address: Default::default(),
            tag: Default::default(),
            startblock: Default::default(),
            endblock: Default::default(),
            page: Default::default(),
            offset: Default::default(),
            sort: Default::default(),
            txhash: Default::default(),
            blockno: Default::default(),
            timestamp: Default::default(),
            format: Default::default(),
            apikey: Default::default(),
        }
    }
}

impl EtherscanRequest {
    #[inline]
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    #[inline]
    pub fn with_apikey(mut self, apikey: String) -> Self {
        self.apikey = Some(apikey);
        self
    }

    #[inline]
    pub fn with_format(mut self, format: EtherscanFormat) -> Self {
        self.format = Some(format);
        self
    }

    #[inline]
    pub fn account_balance(
        address: U256,
        tag: Option<EtherscanTag>,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Account, EtherscanAction::Balance)),
            address: Some(vec![address]),
            tag,
            ..Default::default()
        }
    }

    #[inline]
    pub fn account_balance_multi(
        addresses: Vec<U256>,
        tag: Option<EtherscanTag>,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Account, EtherscanAction::BalanceMulti)),
            address: Some(addresses),
            tag,
            ..Default::default()
        }
    }

    #[inline]
    pub fn account_tx_list(
        address: U256,
        startblock: Option<U256>,
        endblock: Option<U256>,
        page: Option<U256>,
        offset: Option<U256>,
        sort: Option<EtherscanSort>,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Account, EtherscanAction::TxList)),
            address: Some(vec![address]),
            startblock,
            endblock,
            page,
            offset,
            sort,
            ..Default::default()
        }
    }

    #[inline]
    pub fn account_tx_list_internal(
        address: U256,
        startblock: Option<U256>,
        endblock: Option<U256>,
        page: Option<U256>,
        offset: Option<U256>,
        sort: Option<EtherscanSort>,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Account, EtherscanAction::TxListInternal)),
            address: Some(vec![address]),
            startblock,
            endblock,
            page,
            offset,
            sort,
            ..Default::default()
        }
    }

    #[inline]
    pub fn account_tx_list_internal_hash(
        txhash: U256,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Account, EtherscanAction::TxListInternal)),
            txhash: Some(txhash),
            ..Default::default()
        }
    }

    #[inline]
    pub fn account_token_tx(
        contract_address: Option<U256>,
        account_address: Option<U256>,
        startblock: Option<U256>,
        endblock: Option<U256>,
        page: Option<U256>,
        offset: Option<U256>,
        sort: Option<EtherscanSort>,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Account, EtherscanAction::TokenTx)),
            contractaddress: contract_address,
            address: account_address.map(|x| vec![x]),
            page,
            offset,
            startblock,
            endblock,
            sort,
            ..Default::default()
        }
    }

    #[inline]
    pub fn account_token_nft_tx(
        contract_address: U256,
        address: U256,
        startblock: Option<U256>,
        endblock: Option<U256>,
        page: Option<U256>,
        offset: Option<U256>,
        sort: Option<EtherscanSort>,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Account, EtherscanAction::TokenNftTx)),
            contractaddress: Some(contract_address),
            address: Some(vec![address]),
            startblock,
            endblock,
            page,
            offset,
            sort,
            ..Default::default()
        }
    }

    #[inline]
    pub fn account_token_balance(
        account_address: U256,
        contract_address: U256,
        tag: Option<EtherscanTag>,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Account, EtherscanAction::TokenBalance)),
            contractaddress: Some(contract_address),
            address: Some(vec![account_address]),
            tag,
            ..Default::default()
        }
    }

    #[inline]
    pub fn contract_get_abi(
        contract_address: U256,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Contract, EtherscanAction::GetABI)),
            address: Some(vec![contract_address]),
            ..Default::default()
        }
    }

    #[inline]
    pub fn contract_get_source_code(
        contract_address: U256,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Contract, EtherscanAction::GetSourceCode)),
            address: Some(vec![contract_address]),
            ..Default::default()
        }
    }

    #[inline]
    pub fn transaction_get_status(
        transaction_hash: U256,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Transaction, EtherscanAction::GetStatus)),
            txhash: Some(transaction_hash),
            ..Default::default()
        }
    }

    #[inline]
    pub fn transaction_get_receipt_status(
        transaction_hash: U256,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Transaction, EtherscanAction::GetTxReceiptStatus)),
            txhash: Some(transaction_hash),
            ..Default::default()
        }
    }

    #[inline]
    pub fn block_get_countdown(
        block_number: U256,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Block, EtherscanAction::GetBlockCountdown)),
            blockno: Some(block_number),
            ..Default::default()
        }
    }

    #[inline]
    pub fn block_get_number_by_timestamp(
        timestamp: U256,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Block, EtherscanAction::GetBlockNoByTime)),
            timestamp: Some(timestamp),
            ..Default::default()
        }
    }

    #[inline]
    pub fn stats_token_supply(
        contract_address: U256,
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Stats, EtherscanAction::TokenSupply)),
            contractaddress: Some(contract_address),
            ..Default::default()
        }
    }

    #[inline]
    pub fn stats_eth_supply(
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Stats, EtherscanAction::EthSupply)),
            ..Default::default()
        }
    }

    #[inline]
    pub fn stats_eth2_supply(
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Stats, EtherscanAction::EthSupply2)),
            ..Default::default()
        }
    }

    #[inline]
    pub fn stats_eth_price(
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Stats, EtherscanAction::EthPrice)),
            ..Default::default()
        }
    }

    #[inline]
    pub fn stats_node_count(
    ) -> Self {
        Self {
            module_action: Some((EtherscanModule::Stats, EtherscanAction::NodeCount)),
            ..Default::default()
        }
    }

    #[inline]
    pub fn build(
        self,
    ) -> Result<
        impl Future<Output = reqwest::Result<reqwest::Response>>,
        Box<dyn Error + Send + Sync>,
    > {
        let Self {
            url,
            module_action,
            contractaddress,
            address,
            tag,
            page,
            offset,
            startblock,
            endblock,
            sort,
            txhash,
            blockno,
            timestamp,
            format,
            apikey,
        } = self;

        Ok(reqwest::get(format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            match url {
                Some(url) => url,
                None => String::new(),
            },
            match module_action {
                Some((module, _)) => format!("?module={}", serde_plain::to_string(&module)?),
                None => String::new(),
            },
            match module_action {
                Some((_, action)) => format!("&action={}", serde_plain::to_string(&action)?),
                None => String::new(),
            },
            match contractaddress {
                Some(contractaddress) => format!("&contractaddress=0x{contractaddress:X}"),
                None => String::new(),
            },
            match address {
                Some(address) => format!(
                    "&address={}",
                    address
                        .iter()
                        .map(|x| format!("0x{x:X}"))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                None => String::new(),
            },
            match tag {
                Some(tag) => format!("&tag={}", serde_plain::to_string(&tag)?),
                None => String::new(),
            },
            match page {
                Some(page) => format!("&page={page}"),
                None => String::new(),
            },
            match offset {
                Some(offset) => format!("&offset={offset}"),
                None => String::new(),
            },
            match startblock {
                Some(startblock) => format!("&startblock={startblock}"),
                None => String::new(),
            },
            match endblock {
                Some(endblock) => format!("&endblock={endblock}"),
                None => String::new(),
            },
            match sort {
                Some(sort) => format!("&sort={}", serde_plain::to_string(&sort)?),
                None => String::new(),
            },
            match txhash {
                Some(txhash) => format!("&txhash=0x{txhash:X}"),
                None => String::new(),
            },
            match blockno {
                Some(blockno) => format!("&blockno={blockno}"),
                None => String::new(),
            },
            match timestamp {
                Some(timestamp) => format!("&timestamp={timestamp}"),
                None => String::new(),
            },
            match format {
                Some(format) => format!("&format={}", serde_plain::to_string(&format)?),
                None => String::new(),
            },
            match apikey {
                Some(apikey) => format!("&apikey={apikey}"),
                None => String::new(),
            }
        )))
    }
}
