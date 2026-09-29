# Munarium Assure implementation architecture

**Proposed design; scaffold only.** Based on section 14 of the
[platform plan, revision 4](https://github.com/iokaio/munarium-platform/blob/main/docs/platform-plan.md), with lifecycle and failure rules in
sections 17–19 and 22. See the hub's
[scaffold decision proposal](https://github.com/iokaio/munarium-platform/blob/main/docs/decisions/0001-scaffold-boundaries.md)
for the distinction between local interfaces and normative contracts.

## Responsibility and current boundary

Portable evidence assembly, independent offline verification, and bounded evidence reports. Assure belongs to the **assurance plane**.
The crate declares interfaces only: no concrete implementations, serialization,
network listeners, persistence, service authentication or target operations exist.

The associated input, output and error types are intentionally unspecified.
These are proposed in-process seams for implementation work, not a released Rust API
or a second definition of the shared wire contract. A trait signature does not enforce
the trust assumptions below. Async runtime, transport and storage choices remain open.

## Module map

| Source | Proposed interface | Responsibility |
|---|---|---|
| [package](../src/package.rs) | `EvidenceAssembler` | Each included artifact needs its authoritative reference and digest; an incomplete interval must remain visibly incomplete. |
| [verification](../src/verification.rs) | `EvidenceVerifier` | Check signatures, references, expected ranges and omissions without a running Console. Integrity is distinct from completeness and truth. |
| [report](../src/report.rs) | `ReportRenderer` | Mappings describe evidence for objectives, not certification, business correctness or legal applicability. |

## Planned flow and state ownership

Select declared boundary, composition and interval → gather pinned records/artifacts → preserve digests, references and omissions → assemble portable package → independently verify using retained trust context → render integrity, coverage and reconstructability findings.

Own export manifests and preserved archive objects with reproducible references. Server remains the authoritative ledger and checkpoint owner. Trust material, retention state and external witness context must travel with or be explicitly required by verification.

## Dependencies and failure behavior

| Dependency | Required input or service | Failure rule |
|---|---|---|
| Server S2 / S5 | Linked action records, signed checkpoints and retained ranges | Missing or changed records are reported as findings. |
| Registry / Council / Sentinel | Inventory, activation, approval and incident references | Record omissions and source coverage; do not infer that absent incidents mean none occurred. |
| Hub composition | Exact component and contract versions plus deployment boundary | A floating branch cannot identify the verified composition. |
| Trust / retention policy | Permitted signing keys, revocation history and lawful removal metadata | Unknown/revoked keys and removed content have explicit verification outcomes. |

No dependency is linked into this scaffold. Supported contract versions are **none**.
Future adapters must consume a reviewed, versioned contract and identify its digest;
a floating hub branch is design context, never deployment authority.

## Threat assumptions

Treat agent code, supplied content and self-reported identity as untrusted.
Host administrators, release roots and required signing authorities remain explicit
trust assumptions of a qualified deployment. Process separation alone does not prove
independent administration.

| Threat | Required control to implement and test |
|---|---|
| Signed but incomplete package | Verify declared expected ranges and omissions separately from signatures. |
| Tampered archive or lost trust context | Digest/link/signature validation with explicit retained key context. |
| Overclaimed mapping or sensitive export | Bounded report language, rights-reviewed references and payload minimization. |

The [validation specification](validation.md) connects these requirements to the hub
invariants. No test evidence is implied by this design.

## Decisions needed before implementation

Agree package format, digest/signature envelope, interval completeness rules, key-time semantics and retention representation in the hub. Choose a bounded local archive format before adding object-lock adapters.

A cross-component semantic change starts in a hub decision record. Keep publication,
activation and component implementation separate. Use expand, migrate, remove for
future breaking contract changes; never duplicate hashing, identity or grant rules.

## Deferred scope

Broad GRC exports, object-lock/WORM adapters, and any automated certification or compliance determination.

The [implementation plan](implementation-plan.md) sequences the first useful increment.
No deployment recipe, service port or live-provider configuration is supplied at this stage.
