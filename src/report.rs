// SPDX-License-Identifier: Apache-2.0
//! Render verification findings without broadening their claim.
//!
//! Mappings describe evidence for objectives, not certification, business correctness or legal applicability.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: render verification findings without broadening their claim.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait ReportRenderer {
    /// Input whose concrete shape and validation rules are still to be specified.
    type VerificationReport;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type Report;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Render verification findings without broadening their claim.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn render(&self, input: &Self::VerificationReport) -> Result<Self::Report, Self::Error>;
}
