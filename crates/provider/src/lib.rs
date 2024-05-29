#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(any(test, feature = "reqwest"))]
/// Type alias for a [`RootProvider`] using the [`Http`] transport and a
/// reqwest client.
///
/// [`Http`]: linera_alloy_transport_http::Http
pub type ReqwestProvider<N = alloy_network::Ethereum> =
    crate::RootProvider<linera_alloy_transport_http::Http<reqwest::Client>, N>;

#[cfg(feature = "hyper")]
/// Type alias for a [`RootProvider`] using the [`Http`] transport and a hyper
/// client.
///
/// [`Http`]: linera_alloy_transport_http::Http
pub type HyperProvider<N = alloy_network::Ethereum> =
    crate::RootProvider<linera_alloy_transport_http::Http<linera_alloy_transport_http::HyperClient>, N>;

#[macro_use]
extern crate tracing;

mod builder;
pub use builder::{Identity, ProviderBuilder, ProviderLayer, Stack};

pub mod ext;

pub mod fillers;
pub mod layers;

mod chain;

mod heart;
pub use heart::{PendingTransaction, PendingTransactionBuilder, PendingTransactionConfig};

mod provider;
pub use provider::{
    EthCall, FilterPollerBuilder, Provider, RootProvider, RpcWithBlock, SendableTx, TraceCallList,
    WalletProvider,
};

pub mod utils;

#[doc(no_inline)]
pub use alloy_network::{self as network, Network};
