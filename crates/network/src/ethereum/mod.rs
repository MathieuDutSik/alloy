use crate::{Network, ReceiptResponse};

mod builder;

mod signer;
pub use signer::EthereumSigner;

/// Types for a mainnet-like Ethereum network.
#[derive(Clone, Copy, Debug)]
pub struct Ethereum {
    _private: (),
}

impl Network for Ethereum {
    type TxType = linera_alloy_consensus::TxType;

    type TxEnvelope = linera_alloy_consensus::TxEnvelope;

    type UnsignedTx = linera_alloy_consensus::TypedTransaction;

    type ReceiptEnvelope = linera_alloy_consensus::ReceiptEnvelope;

    type Header = linera_alloy_consensus::Header;

    type TransactionRequest = linera_alloy_rpc_types::transaction::TransactionRequest;

    type TransactionResponse = linera_alloy_rpc_types::Transaction;

    type ReceiptResponse = linera_alloy_rpc_types::TransactionReceipt;

    type HeaderResponse = linera_alloy_rpc_types::Header;
}

impl ReceiptResponse for linera_alloy_rpc_types::TransactionReceipt {
    fn contract_address(&self) -> Option<linera_alloy_primitives::Address> {
        self.contract_address
    }
}
