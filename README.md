# Munarium Assure

**Control-framework mapping and evidence packs.** Assure is the assurance-plane component of the
Munarium Governance Platform that turns the platform's records into evidence someone else can
check: a portable, signed package that identifies a deployment boundary, its component composition,
the decisions, approvals, activations, incidents, unresolved outcomes and declared omissions in an
interval, and an offline verifier that checks the package without trusting the interface that
produced it. Assure produces evidence for oversight. It does not certify an enterprise, guarantee
regulatory compliance, or validate model accuracy.

> **Status: Planned — Rust scaffold present.** This checkout contains a dependency-free,
> non-publishable [Cargo library](Cargo.toml) and documented interfaces under [src/](src/lib.rs).
> The interfaces have no implementations: no runtime service, client transport, database,
> provider integration or contract implementation is available. No production path is qualified.
> Build checks validate source structure, not governance capabilities. The
> [capability table](#capability-status) remains the authoritative functional status.

Assure is one of nine components built around the existing Munarium foundation, Munarium Server
and Munarium Matrix. Their shared architecture, normative contracts, decision records, roadmap and
composition evidence live in the public hub,
[iokaio/munarium-platform](https://github.com/iokaio/munarium-platform). This repository will hold
Assure's implementation, its unit and component tests, operational diagnostics, package
definitions, a local development recipe and release evidence. **Assure is open source from the
outset.** The earlier platform plan considered enterprise-only packaging for this work; the revised
plan does not, and there is no closed edition of it.

## Start building

Read the [development index](docs/README.md), then the [architecture](docs/architecture.md),
[implementation plan](docs/implementation-plan.md) and [validation guide](docs/validation.md).
They map the public platform plan to source modules, dependencies, a first bounded work item
and acceptance cases. Runtime capabilities remain planned; supported contract versions are **none**.

## What Assure is for

The platform's guarantee is conditional: within a qualified deployment, the specified consequential
path is mediated. A deployment's control-boundary record must therefore identify the systems,
identities, networks, tools and administrative roles included in qualification, and Assure exports
that boundary along with the positive evidence. **Silence about untested paths would make the
assurance misleading**, so declared omissions are part of the package, not a footnote.

## The design, as planned

### The evidence-package contract

A package identifies the deployment boundary, component composition, time interval, ledger ranges
or pins, policy activations, system inventory, approvals, exceptions, incidents, unresolved outcomes
and declared omissions. **Each item retains a digest and a link** to the authoritative record or
preserved artifact needed to verify it.

The first release supports a compact, machine-readable format and a human-readable report. The
same package must be understandable to an engineer investigating a failure and to an assessor
reviewing a control claim. A broad collection of branded GRC exports is secondary.

### The verifier

The verifier checks the package manifest, signatures, links, expected ranges and referenced
artifacts **without needing access to the running Console**. An authenticated package with a
missing receipt reports the gap. **Integrity verification cannot transform incomplete evidence into
proof of completeness.** Independent verification is meaningful only when an adopter can run it
without trusting the same UI that produced the report.

### Framework mappings

NIST AI RMF, ISO/IEC 42001, NIST SP 800-207 zero trust, the OWASP GenAI risk guidance and the EU AI
Act are organizing references. Assure maps control objectives to evidence types and **states the
limits of the mapping**: an approval record can evidence that a named process step occurred; it
cannot prove the approver exercised good judgment or that the business decision was correct.

Mappings are versioned, attributed and reviewed for applicable rights. **No restricted standards
text is copied into this repository** because the implementation is open. Regulatory applicability,
organizational role and legal interpretation remain the adopter's responsibility.

| Reference area | Evidence the platform intends to supply | What remains outside the claim |
|---|---|---|
| NIST AI RMF: Govern, Map, Measure, Manage | Ownership, inventory, policy decisions, measurement, incidents and response | Complete organizational risk management and business judgment |
| NIST SP 800-207 zero trust | Per-request authorization, verified identity, scoped access, explicit boundaries | Proof that every enterprise system and administrative path follows zero trust |
| ISO/IEC 42001 | Controlled records, reviews, responsibilities, operational evidence | Certification, or satisfaction of the whole management-system standard |
| OWASP GenAI risk guidance | Tests addressing injection-driven effects, excessive agency and disclosure paths | Elimination of all model, application, supply-chain or organizational risks |
| EU AI Act record-keeping, transparency, oversight, robustness | Versioned risk controls, record-keeping, system descriptions, approval and oversight records, monitoring evidence | Applicability determination, adequacy of disclosures, quality of human oversight, model accuracy, deployer obligations |

### Retention and archival

Exports go to an ordinary local archive first, with object-lock or WORM targets added through
adapters. The archive preserves the link structure and identifiers later verification needs. An
external checkpoint or anchor strengthens tamper evidence; **it does not prove that the original
events were true or that omitted events never occurred**.

Retention separates durable accountability metadata from sensitive payloads. Tenant policy
identifies what can be deleted or cryptographically retired, what must be retained, and how a lawful
removal is represented without fabricating a complete replay afterwards. A historical policy
decision can remain attributable when a protected source document is no longer retained; the report
distinguishes those levels of reconstructability.

## First public increment

**A portable evidence package and an offline verifier**, with the control-boundary record and
declared omissions, and one mapping stated with its limits.

Target window: Stage 4 (months 10–12).

## Capability status

The labels are evidence labels, not editions: **Planned**, **Experimental**, **Conformance-tested**,
**Reference-qualified**, **Independently reviewed**. In the hub's component catalog this
repository is at **repository created**.

| Capability | Status | Evidence |
|---|---|---|
| Evidence-package format: boundary, composition, interval, pins, activations, inventory, approvals, exceptions, incidents, unresolved outcomes, declared omissions | Planned | none |
| Per-item digests and links to authoritative records | Planned | none |
| Offline verifier: manifest, signatures, links, ranges, referenced artifacts, gaps | Planned | none |
| Human-readable report from the same package | Planned | none |
| Control-boundary record export | Planned | none |
| Versioned, attributed framework mappings with stated limits | Planned | none |
| Local archive export preserving link structure | Planned | none |
| Object-lock and WORM archive adapters | Planned, later | none |
| External checkpoint or anchor support | Planned, later | none |
| Branded GRC exports | Deferred | none |

Supported contract versions: **none**. Supported archive targets: **none**. Operations available
today: **none**.

## Acceptance evidence for the first release

| Test | Required outcome |
|---|---|
| Corrupted archive | Verifier reports the corruption; nothing passes |
| Missing artifact or receipt | Reported as a gap; the package is not complete |
| Altered manifest | Signature or digest mismatch reported |
| Unknown signing key | Refused |
| Revoked signing key | Refused for packages signed after revocation; historical verification behavior stated |
| Withheld ledger range | Reported against the expected range |
| Expired retention window | Reported; the lower level of reconstructability is stated |
| Verification without Console | Runs from the package and the referenced artifacts alone |
| Report versus package | The human-readable report contains no claim absent from the machine-readable package |

A blank evidence field means unverified, not passed.

## Invariants

| ID | Required property | Owner and first gate |
|---|---|---|
| INV-17 | Missing evidence or telemetry is reported as a gap, not a successful interval | Sentinel and Assure; stages 3–4 |
| INV-18 | Evidence-package verification detects changed or missing required artifacts | Assure and Server; stage 4 |
| INV-22 | A release advertises only the profiles and capabilities supported by its evidence | every component; every stage |

## Contracts, dependencies and neighbors

- **Contracts.** The hub's contracts directory is normative for the evidence-package format and
  the composition manifest it references. Supported contract versions: none yet.
- **Foundation.** Munarium Server 1.3.0's ledger is the authoritative record; the hub's S5 (sign
  ledger checkpoints and support external witnesses, documenting who controls the signing key and
  what tampering before or after a checkpoint can still evade detection) is the Server change
  Assure depends on.
- **Sentinel** and Assure read the same records and report gaps the same way; **Council**,
  **Registry**, **Gate**, **Warden** and **Gateway** produce the approvals, activations, decisions,
  grants and invocation records a package links; **Console** is the interface the verifier must
  not need.
- **External dependencies.** None chosen.

## Not in scope

- Certifying an enterprise, guaranteeing compliance, or validating model accuracy.
- Copying restricted standards or regulatory text into the tree.
- Presenting a mapping as a legal applicability determination.
- Turning integrity verification into proof of completeness, or an external anchor into proof that
  the original events were true.
- Retaining sensitive payloads in accountability metadata.

## Roadmap position

| Stage | Assure's part |
|---|---|
| 0 · month 1 | This repository; the evidence-package contract drafted in the hub |
| 1–3 · months 2–9 | Consumes the action-record shapes and control-boundary conventions as they land; no separate release |
| 4 · months 10–12 | Package format, offline verifier, control-boundary export, first mapping; verified evidence export is exit evidence for the reference composition |
| 5 · months 13+ | Archive adapters, anchors, further mappings, demand-led |

## Repository layout

| Path | What exists |
|---|---|
| [Cargo.toml](Cargo.toml), [Cargo.lock](Cargo.lock) | Independent library, version 0.1.0-dev, publishing disabled, no external crate dependencies |
| [src/lib.rs](src/lib.rs) | Documented proposed module interfaces; no runtime implementations |
| [docs/](docs/README.md) | Architecture, implementation sequence and acceptance specifications |
| [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md) | Contribution process and aligned development guidance |
| [.github/workflows/](.github/workflows/) | Automatic Rust, repository-hygiene and DCO checks |
| [scripts/](scripts/), [check_license.py](check_license.py) | Existing documentation, private-material and license checks |
| [LICENSE](LICENSE), [NOTICE](NOTICE), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) | Licensing and dependency notices |

Subsystem modules: [package](src/package.rs), [verification](src/verification.rs), [report](src/report.rs).
Tests, fixtures, migrations, binaries and deployment assets arrive with the implementation that
uses them. The scaffold defines no shared wire types and depends on no sibling checkout.

## Development

Use Rust 1.98.1 with rustfmt, Clippy and the platform's native linker. From this repository root:

```console
cargo fmt --all --check
cargo build --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
cargo test --offline --locked
cargo doc --offline --locked --no-deps
```

The crate currently has **zero runtime or conformance tests**. A successful test command checks
the scaffold only. The [validation guide](docs/validation.md) gives the required behavioral
test specifications and explains how to retain evidence when they are implemented.

Also run the existing hygiene gates:

```console
py check_license.py
py scripts/private_material_scan.py
py scripts/docs_linkcheck.py
gitleaks dir . --config .gitleaks.toml --no-banner --redact --exit-code 1
git diff --check
```

Use `python` or `python3` where `py` is unavailable. The new
[Rust workflow](.github/workflows/rust.yml) runs on main pushes and pull requests alongside
the existing [repository hygiene](.github/workflows/repo-hygiene.yml) and
[DCO](.github/workflows/dco.yml) workflows. They provide build and repository checks, not a
qualified runtime. No package is published or service deployed by these workflows.
Local checks do not imply hosted CI success. See [CONTRIBUTING.md](CONTRIBUTING.md).

## The platform

| Repository | Plane | Role |
|---|---|---|
| [iokaio/munarium-platform](https://github.com/iokaio/munarium-platform) | hub | Architecture, normative contracts, decision records, roadmap and composition evidence for the whole platform |
| [iokaio/munarium](https://github.com/iokaio/munarium) | foundation (mediation) | Munarium Server: governed memory, the append-only ledger, and the Server client libraries |
| [iokaio/munarium-matrix](https://github.com/iokaio/munarium-matrix) | foundation (mediation) | Munarium Matrix: governed, read-only structured evidence from enterprise data sources |
| [iokaio/munarium-registry](https://github.com/iokaio/munarium-registry) | authority | Inventory of agents, tools, manifests, and policy bundles |
| [iokaio/munarium-harness](https://github.com/iokaio/munarium-harness) | agent | SDKs that make the governed path easy for honest agents |
| [iokaio/munarium-warden](https://github.com/iokaio/munarium-warden) | authority | Workload identity, delegation, just-in-time credentials, kill switches |
| [iokaio/munarium-gate](https://github.com/iokaio/munarium-gate) | mediation | Policy decision and enforcement point for every tool call |
| [iokaio/munarium-gateway](https://github.com/iokaio/munarium-gateway) | mediation | Model-call mediation: routing, BYOK, budgets, screening |
| [iokaio/munarium-council](https://github.com/iokaio/munarium-council) | authority | Approvals, policy lifecycle, ratified governance transitions |
| [iokaio/munarium-sentinel](https://github.com/iokaio/munarium-sentinel) | assurance | Telemetry, anomaly detection, circuit breakers, incident replay |
| [iokaio/munarium-assure](https://github.com/iokaio/munarium-assure) | assurance | Control-framework mapping and evidence packs |
| [iokaio/munarium-console](https://github.com/iokaio/munarium-console) | assurance | One interface for approvers, operators, and auditors |
| [iokaio/munarium-clients-publish](https://github.com/iokaio/munarium-clients-publish) | tooling | The one place Munarium client packages are built for release and published from |
| [iokaio/munarium-demo](https://github.com/iokaio/munarium-demo) | examples | Munarium Demo: working applications and bundled datasets for evaluating the foundation |

The development tool VCP ([iokaio/vcp](https://github.com/iokaio/vcp)) is separate: not one of the
nine components and not a runtime dependency for adopters. Ioka's private repositories hold
planning material awaiting publication review and the proprietary Matrix analytics adapters;
nothing from them is copied into a public repository without that review.

## Licensing

Apache-2.0 ([LICENSE](LICENSE), [NOTICE](NOTICE)). The names are not part of that grant:
[TRADEMARK.md](TRADEMARK.md) says what you may do without asking, which is most things. There is
no proprietary edition of this component and none is planned; a capability that arrives later is
deferred roadmap work, not a commercial restriction.

## Contributing, support, security

Signed-off pull requests, no CLA ([CONTRIBUTING.md](CONTRIBUTING.md)). Questions go to Discussions,
defects and design findings to Issues, and suspected vulnerabilities to the private channel
[SECURITY.md](SECURITY.md) names, never a public issue. What is and is not supported:
[SUPPORT.md](SUPPORT.md). Conduct: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Release history,
such as it is: [CHANGELOG.md](CHANGELOG.md).
