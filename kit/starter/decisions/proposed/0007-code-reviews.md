# ADR-0007: Code Reviews

> State: Proposed

## Status

Proposed
Created: 2026-06-12

## Stakeholders

_Who owns this decision, and who needs to sign off? List the roles or people involved._

<!-- adroit:seeded-from-assessment -->

<!-- adroit:ai-suggested -->

## Context and Problem Statement

Regular review of code changes by peers
> Seeded from assessment "Software Engineering Maturity Assessment" — domain "Code Quality" → practice "Code Reviews".

The assessment highlighted the importance of regular code reviews to ensure high-quality code, reduce bugs, and improve knowledge sharing among team members.

## Decision Drivers

- **Why it matters:** Improved code quality, reduced bugs, and increased knowledge sharing
- **Risk if unaddressed:** Inadequate code reviews leading to technical debt

## Considered Options

### Option 1: Mandatory Code Reviews for All Changes
* Require all changes to be reviewed by at least two team members within 24 hours of check-in.
* Automated tests will be run on the code changes before review.

Pros:
* Ensures high-quality code and reduces bugs.
* Fosters a culture of knowledge sharing among team members.
Cons:
* Increases the time spent on code reviews, potentially slowing down development.
* May lead to burnout if reviewers are overwhelmed with work.

### Option 2: Mandatory Code Reviews for High-Risk Changes
* Require only high-risk changes (e.g., new features, API updates) to be reviewed by at least two team members within 24 hours of check-in.
* Automated tests will be run on all code changes before review.

Pros:
* Still ensures high-quality code and reduces bugs on critical changes.
Cons:
* May allow low-risk changes to bypass thorough review, potentially introducing issues later.
* Requires more judgment from reviewers to decide which changes are high-risk.

### Option 3: Code Review as a Best Practice
* Encourage but do not require code reviews for all changes.
* Automated tests will still be run on all code changes before deployment.

Pros:
* Allows for flexibility and choice in how team members want to review their own work.
Cons:
* May lead to inconsistent quality of code if reviewers are not diligent.
* Does not address the risk of inadequate code reviews leading to technical debt.

## Decision Outcome

Chosen: **Option 1**, because it ensures high-quality code and reduces bugs while fostering a culture of knowledge sharing among team members.

### Positive Consequences

* Improved code quality through regular review and feedback.
* Reduced bugs and errors due to thorough testing.
* Increased knowledge sharing among team members, leading to improved collaboration and innovation.

### Negative Consequences

* Potential increase in development time due to mandatory reviews.
* Risk of burnout if reviewers are overwhelmed with work.

## Implementation notes
* The first step will be to implement automated tests for all code changes, as required by Option 1.
* A review process will need to be established to ensure that all team members understand their roles and responsibilities in the code review process.
* The engineering lead will need to communicate the importance of code reviews to the entire team and provide support as needed.
