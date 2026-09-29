# Security

Do not file a vulnerability as an issue or a pull request.

Report a suspected vulnerability in anything in this repository privately, by either route:

- GitHub's private vulnerability reporting ("Report a vulnerability" under the Security tab), or
- email to **info@ioka.io** with "security" in the subject.

Say what you found, where, and how to reproduce it. Do not include live credentials, customer data,
or a proof of concept run against a system you do not operate. You will get an acknowledgement
within two business days, and a fix, or a recorded decision, on the affected path before any related
release. Credit is given if you ask for it.

## Supported versions

Munarium Assure has no release. Until the first tagged release, `main` is the only line and a fix
lands there. Once releases exist, security fixes go to the current minor release and to the previous
one for six months after its successor ships; an older release gets a fix only where the
vulnerability is in a contract it still speaks.

A finding in the design is welcome now, through the same private channel if it has security
consequences and as an ordinary issue otherwise. The threat model this component is built against
is in [README.md](README.md) and, for the platform as a whole, in the hub
([iokaio/munarium-platform](https://github.com/iokaio/munarium-platform)).

## What matters most here

As runtime behavior is implemented, these are the classes of finding taken most seriously and most
quickly:

- **A verifier that passes a package it should not**: an altered manifest, a missing or changed required artifact, an unknown or revoked signing key, a withheld ledger range, or an expired retention window reported as complete.
- **Incomplete evidence presented as proof of completeness.** An authenticated package with a missing receipt must report the gap; integrity verification cannot manufacture completeness.
- **A declared omission that is not declared**, or a control-boundary record that is silent about untested paths.
- **A lawful removal represented as a complete replay**, or a retained accountability record that carries the sensitive payload it should only reference.
- **Restricted standards text copied into the repository**, or a mapping presented as a certification or a legal applicability determination.

## What is deliberate, and is not a defect

- **Assure produces evidence for oversight.** It does not certify an enterprise, guarantee regulatory compliance, or validate model accuracy, and its reports say so. A finding that Assure "does not prove compliance" describes the design.
- **An external checkpoint or anchor strengthens tamper evidence; it does not prove the original events were true or that omitted events never occurred.** The report distinguishes those levels of reconstructability.
- **Assure is open source.** The earlier plan considered enterprise-only packaging for this work; the revised plan does not, and there is no closed edition of it.

When a local development profile exists, its test identity provider, test broker, disposable target
and generated sample credentials are development conveniences confined to that profile. They are
not vulnerabilities in themselves. A path by which they reach a production deployment unnoticed is.

## Findings that cross components

A contract ambiguity that lets two components disagree about authority, a canonicalization
difference between clients, or a gap between what a release advertises and what its evidence
supports is still a security finding. Report it here, or to any other Munarium repository, through
the same private channel; it is routed to the hub and the affected repositories together. Do not
open a public issue for it in the hub.

## Secrets

If you have committed a token or key, treat it as compromised: rotate it first, then report it.
