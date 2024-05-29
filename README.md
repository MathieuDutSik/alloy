# Alloy

Alloy connects applications to blockchains.

Alloy is a rewrite of [`ethers-rs`] from the ground up, with exciting new
features, high performance, and excellent [docs](https://alloy-rs.github.io/alloy/).

[`ethers-rs`] will continue to be maintained until we have achieved
feature-parity in Alloy. No action is currently needed from devs.

[![Telegram chat][telegram-badge]][telegram-url]

[`ethers-rs`]: https://github.com/gakonst/ethers-rs
[telegram-badge]: https://img.shields.io/endpoint?color=neon&style=for-the-badge&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Fethers_rs
[telegram-url]: https://t.me/ethers_rs

## Installation

Currently, Alloy is not hosted on [crates.io](https://crates.io), the Rust package registry.

To incorporate Alloy into your project, you will need to specify the GitHub repository as the source. This can be achieved by executing the following command in your terminal:

```sh
cargo add alloy --git https://github.com/alloy-rs/alloy
```

After incorporating Alloy, you may wish to utilize specific features of the crate. These features can be enabled through modifications in your project's `Cargo.toml` file. A comprehensive list of available features can be found at [this GitHub link](https://github.com/alloy-rs/alloy/blob/35cbf35164f31d2de1b84b2a8a9986e5b9b1560f/crates/alloy/Cargo.toml#L89). 

## Overview

This repository contains the following crates:

- [`alloy`]: Meta-crate for the entire project, including [`alloy-core`]
- [`linera-alloy-consensus`] - Ethereum consensus interface
- [`linera-alloy-contract`] - Interact with on-chain contracts
- [`linera-alloy-eips`] - Ethereum Improvement Proposal (EIP) implementations
- [`linera-alloy-genesis`] - Ethereum genesis file definitions
- [`linera-alloy-json-rpc`] - Core data types for JSON-RPC 2.0 clients
- [`alloy-network`] - Network abstraction for RPC types
- [`alloy-node-bindings`] - Ethereum execution-layer client bindings
- [`linera-alloy-provider`] - Interface with an Ethereum blockchain
- [`linera-alloy-pubsub`] - Ethereum JSON-RPC [publish-subscribe] tower service and type definitions
- [`linera-alloy-rpc-client`] - Low-level Ethereum JSON-RPC client implementation
- [`linera-alloy-rpc-types`] - Ethereum JSON-RPC types
  - [`linera-alloy-rpc-types-engine`] - Ethereum execution-consensus layer (engine) API RPC types
  - [`linera-alloy-rpc-types-trace`] - Ethereum RPC trace types
- [`alloy-signer`] - Ethereum signer abstraction
  - [`alloy-signer-aws`] - [AWS KMS] signer implementation
  - [`alloy-signer-gcp`] - [GCP KMS] signer implementation
  - [`alloy-signer-ledger`] - [Ledger] signer implementation
  - [`alloy-signer-trezor`] - [Trezor] signer implementation
  - [`alloy-signer-wallet`] - Local wallet (Keystore/Mnemonic/Yubihsm) signer implementation
- [`linera-alloy-transport`] - Low-level Ethereum JSON-RPC transport abstraction
  - [`linera-alloy-transport-http`] - HTTP transport implementation
  - [`linera-alloy-transport-ipc`] - IPC transport implementation
  - [`linera-alloy-transport-ws`] - WS transport implementation

[`alloy`]: https://github.com/alloy-rs/alloy/tree/main/crates/alloy
[`linera-alloy-consensus`]: https://github.com/alloy-rs/alloy/tree/main/crates/consensus
[`linera-alloy-contract`]: https://github.com/alloy-rs/alloy/tree/main/crates/contract
[`linera-alloy-eips`]: https://github.com/alloy-rs/alloy/tree/main/crates/eips
[`linera-alloy-genesis`]: https://github.com/alloy-rs/alloy/tree/main/crates/genesis
[`linera-alloy-json-rpc`]: https://github.com/alloy-rs/alloy/tree/main/crates/json-rpc
[`alloy-network`]: https://github.com/alloy-rs/alloy/tree/main/crates/network
[`alloy-node-bindings`]: https://github.com/alloy-rs/alloy/tree/main/crates/node-bindings
[`linera-alloy-provider`]: https://github.com/alloy-rs/alloy/tree/main/crates/provider
[`linera-alloy-pubsub`]: https://github.com/alloy-rs/alloy/tree/main/crates/pubsub
[`linera-alloy-rpc-client`]: https://github.com/alloy-rs/alloy/tree/main/crates/rpc-client
[`linera-alloy-rpc-types-engine`]: https://github.com/alloy-rs/alloy/tree/main/crates/rpc-types-engine
[`linera-alloy-rpc-types-trace`]: https://github.com/alloy-rs/alloy/tree/main/crates/rpc-types-trace
[`linera-alloy-rpc-types`]: https://github.com/alloy-rs/alloy/tree/main/crates/rpc-types
[`alloy-signer`]: https://github.com/alloy-rs/alloy/tree/main/crates/signer
[`alloy-signer-aws`]: https://github.com/alloy-rs/alloy/tree/main/crates/signer-aws
[`alloy-signer-gcp`]: https://github.com/alloy-rs/alloy/tree/main/crates/signer-gcp
[`alloy-signer-ledger`]: https://github.com/alloy-rs/alloy/tree/main/crates/signer-ledger
[`alloy-signer-trezor`]: https://github.com/alloy-rs/alloy/tree/main/crates/signer-trezor
[`alloy-signer-wallet`]: https://github.com/alloy-rs/alloy/tree/main/crates/signer-wallet
[`linera-alloy-transport`]: https://github.com/alloy-rs/alloy/tree/main/crates/transport
[`linera-alloy-transport-http`]: https://github.com/alloy-rs/alloy/tree/main/crates/transport-http
[`linera-alloy-transport-ipc`]: https://github.com/alloy-rs/alloy/tree/main/crates/transport-ipc
[`linera-alloy-transport-ws`]: https://github.com/alloy-rs/alloy/tree/main/crates/transport-ws
[`alloy-core`]: https://docs.rs/alloy-core
[publish-subscribe]: https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern
[AWS KMS]: https://aws.amazon.com/kms
[GCP KMS]: https://cloud.google.com/kms
[Ledger]: https://www.ledger.com
[Trezor]: https://trezor.io

## Supported Rust Versions

<!--
When updating this, also update:
- .clippy.toml
- Cargo.toml
- .github/workflows/ci.yml
-->

Alloy will keep a rolling MSRV (minimum supported rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.76.

Note that the MSRV is not increased automatically, and only as part of a minor
release.

## Contributing

Thanks for your help improving the project! We are so happy to have you! We have
[a contributing guide](./CONTRIBUTING.md) to help you get involved in the
Alloy project.

Pull requests will not be merged unless CI passes, so please ensure that your
contribution follows the linting rules and passes clippy.

## Note on `no_std`

Because these crates are primarily network-focused, we do not intend to support
`no_std` for most of them at this time.

The following crates support `no_std`:

- linera-alloy-eips
- linera-alloy-genesis
- linera-alloy-serde
- linera-alloy-consensus

If you would like to add `no_std` support to a crate, please make sure to update
`scripts/check_no_std.sh` as well.

## Credits

None of these crates would have been possible without the great work done in:

- [`ethers.js`](https://github.com/ethers-io/ethers.js/)
- [`rust-web3`](https://github.com/tomusdrw/rust-web3/)
- [`ruint`](https://github.com/recmo/uint)
- [`ethabi`](https://github.com/rust-ethereum/ethabi)
- [`ethcontract-rs`](https://github.com/gnosis/ethcontract-rs/)
- [`guac_rs`](https://github.com/althea-net/guac_rs/)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
