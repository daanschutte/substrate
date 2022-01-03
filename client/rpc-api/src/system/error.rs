// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! System RPC module errors.

use crate::system::helpers::Health;
use jsonrpsee::{core::to_json_raw_value, types::error::CallError};

/// System RPC Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// System RPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Provided block range couldn't be resolved to a list of blocks.
	#[error("Node is not fully functional: {}", .0)]
	NotHealthy(Health),
	/// Peer argument is malformatted.
	#[error("{0}")]
	MalformattedPeerArg(String),
}

// Base code for all system errors.
const BASE_ERROR: i32 = 2000;
// Provided block range couldn't be resolved to a list of blocks.
const NOT_HEALTHY_ERROR: i32 = BASE_ERROR + 1;
// Peer argument is malformatted.
const MALFORMATTED_PEER_ARG_ERROR: i32 = BASE_ERROR + 2;

impl From<Error> for CallError {
	fn from(e: Error) -> Self {
		match e {
			Error::NotHealthy(ref h) => Self::Custom {
				code: NOT_HEALTHY_ERROR,
				message: e.to_string(),
				data: to_json_raw_value(&h).ok(),
			},
			Error::MalformattedPeerArg(e) =>
				Self::Custom { code: MALFORMATTED_PEER_ARG_ERROR + 2, message: e, data: None },
		}
	}
}
