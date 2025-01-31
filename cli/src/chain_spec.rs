// Copyright 2017 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Predefined chains.

use service;

/// The chain specification (this should eventually be replaced by a more general JSON-based chain
/// specification).
#[derive(Clone, Debug)]
pub enum ChainSpec {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
	/// The Kusama network.
	Kusama,
	/// Whatever the current runtime is with the "global testnet" defaults.
	StagingTestnet,
	/// Parus network
	Parus,
	/// Yuhu network
	Yuhu,
}

impl Default for ChainSpec {
	fn default() -> Self {
		ChainSpec::Parus
	}
}

/// Get a chain config from a spec setting.
impl ChainSpec {
	pub(crate) fn load(self) -> Result<service::ChainSpec, String> {
		Ok(match self {
			ChainSpec::Yuhu => service::chain_spec::yuhu_testnet_config(),
			ChainSpec::Parus => service::chain_spec::parus_testnet_config(),
			ChainSpec::Kusama => service::chain_spec::kusama_config()?,
			ChainSpec::Development => service::chain_spec::development_config(),
			ChainSpec::LocalTestnet => service::chain_spec::local_testnet_config(),
			ChainSpec::StagingTestnet => service::chain_spec::staging_testnet_config(),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(ChainSpec::Development),
			"local" => Some(ChainSpec::LocalTestnet),
			"kusama" => Some(ChainSpec::Kusama),
			"staging" => Some(ChainSpec::StagingTestnet),
			"parus" => Some(ChainSpec::Parus),
			"yuhu" => Some(ChainSpec::Yuhu),
			"" => Some(ChainSpec::default()),
			_ => None,
		}
	}
}

