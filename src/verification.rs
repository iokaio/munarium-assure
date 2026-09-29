// SPDX-License-Identifier: Apache-2.0
//! Verify a preserved package using an explicit trust context.
//!
//! Check signatures, references, expected ranges and omissions without a running Console. Integrity is distinct from completeness and truth.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: verify a preserved package using an explicit trust context.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait EvidenceVerifier {
    /// Input whose concrete shape and validation rules are still to be specified.
    type EvidencePackage;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type VerificationReport;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Verify a preserved package using an explicit trust context.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn verify(
        &self,
        input: &Self::EvidencePackage,
    ) -> Result<Self::VerificationReport, Self::Error>;
}
