// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions to types generated by `wit-bindgen`.
//!
//! Allows converting types used in `linera-execution` to types that can be sent to the guest WASM
//! module.

use super::runtime::{contract, queryable_system, service, writable_system};
use crate::{
    system::Balance, ApplicationId, CallResult, CalleeContext, EffectContext, EffectId,
    OperationContext, QueryContext, SessionId, UserApplicationId,
};
use linera_base::{crypto::HashValue, messages::ChainId};

impl From<OperationContext> for contract::OperationContext {
    fn from(host: OperationContext) -> Self {
        contract::OperationContext {
            chain_id: host.chain_id.into(),
            height: host.height.0,
            index: host
                .index
                .try_into()
                .expect("Operation index should fit in a `u64`"),
        }
    }
}

impl From<EffectContext> for contract::EffectContext {
    fn from(host: EffectContext) -> Self {
        contract::EffectContext {
            chain_id: host.chain_id.into(),
            height: host.height.0,
            effect_id: host.effect_id.into(),
        }
    }
}

impl From<EffectId> for queryable_system::EffectId {
    fn from(host: EffectId) -> Self {
        queryable_system::EffectId {
            chain_id: host.chain_id.into(),
            height: host.height.0,
            index: host
                .index
                .try_into()
                .expect("Effect index should fit in a `u64`"),
        }
    }
}

impl From<EffectId> for writable_system::EffectId {
    fn from(host: EffectId) -> Self {
        writable_system::EffectId {
            chain_id: host.chain_id.into(),
            height: host.height.0,
            index: host
                .index
                .try_into()
                .expect("Effect index should fit in a `u64`"),
        }
    }
}

impl From<EffectId> for contract::EffectId {
    fn from(host: EffectId) -> Self {
        contract::EffectId {
            chain_id: host.chain_id.into(),
            height: host.height.0,
            index: host
                .index
                .try_into()
                .expect("Effect index should fit in a `u64`"),
        }
    }
}

impl From<CalleeContext> for contract::CalleeContext {
    fn from(host: CalleeContext) -> Self {
        contract::CalleeContext {
            chain_id: host.chain_id.into(),
            authenticated_caller_id: host
                .authenticated_caller_id
                .map(contract::ApplicationId::from),
        }
    }
}

impl From<QueryContext> for service::QueryContext {
    fn from(host: QueryContext) -> Self {
        service::QueryContext {
            chain_id: host.chain_id.into(),
        }
    }
}

impl From<SessionId> for contract::SessionId {
    fn from(host: SessionId) -> Self {
        contract::SessionId {
            application_id: host.application_id.into(),
            kind: host.kind,
            index: host.index,
        }
    }
}

impl From<SessionId> for writable_system::SessionId {
    fn from(host: SessionId) -> Self {
        writable_system::SessionId {
            application_id: host.application_id.into(),
            kind: host.kind,
            index: host.index,
        }
    }
}

impl From<ApplicationId> for contract::ApplicationId {
    fn from(host: ApplicationId) -> Self {
        match host {
            ApplicationId::System => {
                unreachable!("Attempt to allow system application access to user application")
            }
            ApplicationId::User(UserApplicationId { bytecode, creation }) => {
                contract::ApplicationId {
                    bytecode: bytecode.0.into(),
                    creation: creation.into(),
                }
            }
        }
    }
}

impl From<ApplicationId> for queryable_system::ApplicationId {
    fn from(host: ApplicationId) -> Self {
        match host {
            ApplicationId::System => {
                unreachable!("Attempt to allow system application access to user application")
            }
            ApplicationId::User(UserApplicationId { bytecode, creation }) => {
                queryable_system::ApplicationId {
                    bytecode: bytecode.0.into(),
                    creation: creation.into(),
                }
            }
        }
    }
}

impl From<ApplicationId> for writable_system::ApplicationId {
    fn from(host: ApplicationId) -> Self {
        match host {
            ApplicationId::System => {
                unreachable!("Attempt to allow system application access to user application")
            }
            ApplicationId::User(UserApplicationId { bytecode, creation }) => {
                writable_system::ApplicationId {
                    bytecode: bytecode.0.into(),
                    creation: creation.into(),
                }
            }
        }
    }
}

impl From<ChainId> for queryable_system::ChainId {
    fn from(chain_id: ChainId) -> Self {
        chain_id.0.into()
    }
}

impl From<ChainId> for writable_system::ChainId {
    fn from(chain_id: ChainId) -> Self {
        chain_id.0.into()
    }
}

impl From<ChainId> for contract::ChainId {
    fn from(chain_id: ChainId) -> Self {
        chain_id.0.into()
    }
}

impl From<ChainId> for service::ChainId {
    fn from(chain_id: ChainId) -> Self {
        chain_id.0.into()
    }
}

impl From<HashValue> for contract::HashValue {
    fn from(hash_value: HashValue) -> Self {
        let bytes = hash_value.as_bytes();

        contract::HashValue {
            part1: u64::from_le_bytes(bytes[0..8].try_into().expect("incorrect indices")),
            part2: u64::from_le_bytes(bytes[8..16].try_into().expect("incorrect indices")),
            part3: u64::from_le_bytes(bytes[16..24].try_into().expect("incorrect indices")),
            part4: u64::from_le_bytes(bytes[24..32].try_into().expect("incorrect indices")),
            part5: u64::from_le_bytes(bytes[32..40].try_into().expect("incorrect indices")),
            part6: u64::from_le_bytes(bytes[40..48].try_into().expect("incorrect indices")),
            part7: u64::from_le_bytes(bytes[48..56].try_into().expect("incorrect indices")),
            part8: u64::from_le_bytes(bytes[56..64].try_into().expect("incorrect indices")),
        }
    }
}

impl From<HashValue> for service::HashValue {
    fn from(hash_value: HashValue) -> Self {
        let bytes = hash_value.as_bytes();

        service::HashValue {
            part1: u64::from_le_bytes(bytes[0..8].try_into().expect("incorrect indices")),
            part2: u64::from_le_bytes(bytes[8..16].try_into().expect("incorrect indices")),
            part3: u64::from_le_bytes(bytes[16..24].try_into().expect("incorrect indices")),
            part4: u64::from_le_bytes(bytes[24..32].try_into().expect("incorrect indices")),
            part5: u64::from_le_bytes(bytes[32..40].try_into().expect("incorrect indices")),
            part6: u64::from_le_bytes(bytes[40..48].try_into().expect("incorrect indices")),
            part7: u64::from_le_bytes(bytes[48..56].try_into().expect("incorrect indices")),
            part8: u64::from_le_bytes(bytes[56..64].try_into().expect("incorrect indices")),
        }
    }
}

impl From<HashValue> for queryable_system::HashValue {
    fn from(hash_value: HashValue) -> Self {
        let bytes = hash_value.as_bytes();

        queryable_system::HashValue {
            part1: u64::from_le_bytes(bytes[0..8].try_into().expect("incorrect indices")),
            part2: u64::from_le_bytes(bytes[8..16].try_into().expect("incorrect indices")),
            part3: u64::from_le_bytes(bytes[16..24].try_into().expect("incorrect indices")),
            part4: u64::from_le_bytes(bytes[24..32].try_into().expect("incorrect indices")),
            part5: u64::from_le_bytes(bytes[32..40].try_into().expect("incorrect indices")),
            part6: u64::from_le_bytes(bytes[40..48].try_into().expect("incorrect indices")),
            part7: u64::from_le_bytes(bytes[48..56].try_into().expect("incorrect indices")),
            part8: u64::from_le_bytes(bytes[56..64].try_into().expect("incorrect indices")),
        }
    }
}

impl From<HashValue> for writable_system::HashValue {
    fn from(hash_value: HashValue) -> Self {
        let bytes = hash_value.as_bytes();

        writable_system::HashValue {
            part1: u64::from_le_bytes(bytes[0..8].try_into().expect("incorrect indices")),
            part2: u64::from_le_bytes(bytes[8..16].try_into().expect("incorrect indices")),
            part3: u64::from_le_bytes(bytes[16..24].try_into().expect("incorrect indices")),
            part4: u64::from_le_bytes(bytes[24..32].try_into().expect("incorrect indices")),
            part5: u64::from_le_bytes(bytes[32..40].try_into().expect("incorrect indices")),
            part6: u64::from_le_bytes(bytes[40..48].try_into().expect("incorrect indices")),
            part7: u64::from_le_bytes(bytes[48..56].try_into().expect("incorrect indices")),
            part8: u64::from_le_bytes(bytes[56..64].try_into().expect("incorrect indices")),
        }
    }
}

impl From<CallResult> for writable_system::CallResult {
    fn from(host: CallResult) -> Self {
        writable_system::CallResult {
            value: host.value,
            sessions: host
                .sessions
                .into_iter()
                .map(writable_system::SessionId::from)
                .collect(),
        }
    }
}

impl Balance {
    /// Helper function to obtain the 64 most significant bits of the balance.
    fn upper_half(self) -> u64 {
        (u128::from(self) >> 64)
            .try_into()
            .expect("Insufficient shift right for u128 -> u64 conversion")
    }

    /// Helper function to obtain the 64 least significant bits of the balance.
    fn lower_half(self) -> u64 {
        (u128::from(self) & 0xFFFF_FFFF_FFFF_FFFF)
            .try_into()
            .expect("Incorrect mask for u128 -> u64 conversion")
    }
}

impl From<Balance> for queryable_system::SystemBalance {
    fn from(host: Balance) -> Self {
        queryable_system::SystemBalance {
            lower_half: host.lower_half(),
            upper_half: host.upper_half(),
        }
    }
}

impl From<Balance> for writable_system::SystemBalance {
    fn from(host: Balance) -> Self {
        writable_system::SystemBalance {
            lower_half: host.lower_half(),
            upper_half: host.upper_half(),
        }
    }
}
