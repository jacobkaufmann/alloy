//! Contains types related to the Prague hardfork that will be used by RPC to communicate with the
//! beacon consensus engine.

use alloc::vec::Vec;

use alloy_eips::eip7685::Requests;

/// Fields introduced in `engine_newPayloadV4` that are not present in the `ExecutionPayload` RPC
/// object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PraguePayloadFields {
    /// The execution requests.
    pub requests: Requests,

    /// The IL.
    pub il: Vec<Vec<u8>>,
}

/// A container type for [PraguePayloadFields] that may or may not be present.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaybePraguePayloadFields {
    fields: Option<PraguePayloadFields>,
}

impl MaybePraguePayloadFields {
    /// Returns a new `MaybePraguePayloadFields` with no prague fields.
    pub const fn none() -> Self {
        Self { fields: None }
    }

    /// Returns a new `MaybePraguePayloadFields` with the given prague fields.
    pub fn into_inner(self) -> Option<PraguePayloadFields> {
        self.fields
    }

    /// Returns the execution requests, if any.
    pub fn requests(&self) -> Option<&Requests> {
        self.fields.as_ref().map(|fields| &fields.requests)
    }

    /// Returns the IL, if any.
    pub fn il(&self) -> Option<&Vec<Vec<u8>>> {
        self.fields.as_ref().map(|fields| &fields.il)
    }

    /// Returns a reference to the inner fields.
    pub const fn as_ref(&self) -> Option<&PraguePayloadFields> {
        self.fields.as_ref()
    }
}

impl From<PraguePayloadFields> for MaybePraguePayloadFields {
    #[inline]
    fn from(fields: PraguePayloadFields) -> Self {
        Self { fields: Some(fields) }
    }
}

impl From<Option<PraguePayloadFields>> for MaybePraguePayloadFields {
    #[inline]
    fn from(fields: Option<PraguePayloadFields>) -> Self {
        Self { fields }
    }
}
