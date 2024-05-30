use core::fmt;

use crate::{Network, ReceiptResponse};
use linera_alloy_consensus::TxType;
use linera_alloy_eips::eip2718::Eip2718Error;
use linera_alloy_rpc_types::{
    AnyTransactionReceipt, Header, Transaction, TransactionRequest, WithOtherFields,
};

mod builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyTxType(u8);

impl fmt::Display for AnyTxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnyTxType({})", self.0)
    }
}

impl TryFrom<u8> for AnyTxType {
    type Error = Eip2718Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl From<AnyTxType> for u8 {
    fn from(value: AnyTxType) -> Self {
        value.0
    }
}

impl TryFrom<AnyTxType> for TxType {
    type Error = Eip2718Error;

    fn try_from(value: AnyTxType) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl From<TxType> for AnyTxType {
    fn from(value: TxType) -> Self {
        Self(value as u8)
    }
}

/// Types for a catch-all network.
///
/// Essentially just returns the regular Ethereum types + a catch all field.
/// This [`Network`] should be used only when the network is not known at
/// compile time.
#[derive(Clone, Copy, Debug)]
pub struct AnyNetwork {
    _private: (),
}

impl Network for AnyNetwork {
    type TxType = AnyTxType;

    type TxEnvelope = linera_alloy_consensus::TxEnvelope;

    type UnsignedTx = linera_alloy_consensus::TypedTransaction;

    type ReceiptEnvelope = linera_alloy_consensus::AnyReceiptEnvelope;

    type Header = linera_alloy_consensus::Header;

    type TransactionRequest = WithOtherFields<TransactionRequest>;

    type TransactionResponse = WithOtherFields<Transaction>;

    type ReceiptResponse = AnyTransactionReceipt;

    type HeaderResponse = WithOtherFields<Header>;
}

impl ReceiptResponse for AnyTransactionReceipt {
    fn contract_address(&self) -> Option<linera_alloy_primitives::Address> {
        self.contract_address
    }
}
