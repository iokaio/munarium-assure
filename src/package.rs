// SPDX-License-Identifier: Apache-2.0
//! Assemble evidence with explicit boundary and omissions.
//!
//! Each included artifact needs its authoritative reference and digest; an incomplete interval must remain visibly incomplete.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: assemble evidence with explicit boundary and omissions.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait EvidenceAssembler {
    /// Input whose concrete shape and validation rules are still to be specified.
    type ExportRequest;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type EvidencePackage;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Assemble evidence with explicit boundary and omissions.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn assemble(&self, input: &Self::ExportRequest) -> Result<Self::EvidencePackage, Self::Error>;
}
