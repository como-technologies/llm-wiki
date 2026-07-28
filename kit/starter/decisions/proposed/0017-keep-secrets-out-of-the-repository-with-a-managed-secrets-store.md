# ADR-0017: Keep secrets out of the repository with a managed secrets store

> State: Proposed

## Status

Proposed
Created: 2026-06-12

## Stakeholders

Tech lead (owner), all engineers (handle credentials daily), whoever
operates the deployment pipeline (consumes secrets at deploy time).

## Context and Problem Statement

Credentials — API tokens, database passwords, signing keys, webhook
secrets — gravitate toward wherever they are easiest to use, and the
easiest place is always the repository: a config file, a CI script, a
"temporary" test fixture. Once a secret is committed, it is in the history
forever; rotating it means both changing the credential *and* remembering
that the old value is still readable by anyone with a clone. Forks,
backups, and CI caches multiply the copies. Most leaked credentials are
not stolen from vaults — they are read out of version control.

Today the team has no recorded rule, which in practice means each engineer
improvises. We need one explicit storage convention for secrets, plus a
mechanical backstop for the inevitable mistake.

## Decision Drivers

- A committed secret is a permanent artifact: prevention is categorically
  cheaper than rotation-plus-history-scrubbing
- Engineers need a *sanctioned* easy path — a rule with no convenient
  alternative gets bypassed under deadline
- Rotation must be possible without code changes or redeploys of
  everything that reads the value
- Bring-your-own-stack: the convention must name a role (managed secrets
  store), not a vendor — your cloud's secret manager, a self-hosted vault
  service, or your forge's encrypted CI secrets all qualify
- The mistake case must be caught mechanically, before a push, not in a
  quarterly audit

## Considered Options

1. **Managed secrets store + local env files + a pre-commit/CI scanner** —
   runtime and CI secrets live in a managed store and reach processes as
   environment variables or mounted files; local development uses
   git-ignored env files seeded from a committed `*.example` file with
   placeholder values; a secret scanner runs pre-commit and in CI as the
   backstop.
2. **Encrypted secrets committed in-repo** (e.g. an encrypted YAML/age/
   sops-style workflow) — keeps secrets versioned next to code and works
   offline, but key distribution becomes the new secret-management
   problem, and one mis-encrypted file is a plaintext commit.
3. **Status quo: convention by folklore** — no rule, no scanner. Free
   today; this is the option that ends in an incident report.

## Decision Outcome

Chosen: **managed secrets store + local env files + a scanner**, because
it gives every secret a home that is *easier* than committing it, keeps
rotation a store-side operation, and pairs the human rule with a
mechanical catch for the day the rule is forgotten.

Concretely:

- Runtime/CI secrets live only in the managed store; applications read
  them from the environment or a mounted path — never from tracked files.
- Every needed variable is documented by a committed example file with
  placeholder values; the real local file is git-ignored.
- A secret scanner runs as a pre-commit hook and again in CI; a hit blocks
  the merge.
- A committed secret is treated as compromised the moment it lands:
  rotate first, then clean up — history rewriting is cosmetic, not
  remediation.

### Positive Consequences

- Rotation becomes routine (change it in the store) instead of a code
  change with an audit of every clone
- Onboarding is documented by construction: the example file *is* the
  list of required configuration
- The scanner converts the team's worst credential day from an incident
  into a blocked commit

### Negative Consequences

- A managed store is new infrastructure with its own availability,
  access-control, and audit story — the team must actually operate it
- Local development gains one indirection (seed the env file first);
  the example file will drift unless reviews hold it to the same bar as
  code
- Scanners produce false positives (high-entropy strings, test keys) and
  false negatives (structured secrets they don't recognize); the rule,
  not the tool, is the actual control

## Rollout

1. Pick the store that matches the existing platform (cloud secret
   manager, self-hosted vault service, or forge CI secrets) and define
   who can read/write which scopes.
2. Inventory current secrets: move them to the store, rotate anything
   that has ever been committed.
3. Add the example-file convention and git-ignore rules; wire the scanner
   into pre-commit and CI.
4. Document the "I committed a secret" procedure (rotate, then clean) in
   the team's runbook material.
