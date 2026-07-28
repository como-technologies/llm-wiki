# ADR-0009: Unit Testing

> State: Proposed

## Status

Proposed
Created: 2026-06-12

## Stakeholders

_Who owns this decision, and who needs to sign off? List the roles or people involved._

<!-- adroit:seeded-from-assessment -->

<!-- adroit:ai-suggested -->

## Context and Problem Statement

Use of unit tests to verify individual components. The assessment flagged:
- Are unit tests written for all new code changes?
- Do you have a continuous integration/continuous deployment (CI/CD) pipeline that runs unit tests?
- Are there any known defects in the codebase that were not caught by unit testing?
- Do you have a process for reviewing and approving unit test results?
- Are there any automated tests that are not currently running as part of the CI/CD pipeline?
- Do you have a code coverage target for unit tests?
- Are there any known issues with the testing framework or tools used for unit testing?
- Do you have a process for refactoring code that affects unit tests?
- Are there any unit tests that are not currently passing?
- Do you have a code review process that includes reviewing unit tests?
- Are there any known security vulnerabilities in the codebase that were not caught by unit testing?
- Do you have a process for maintaining and updating unit tests over time?

## Decision Drivers

- **Why it matters:** Reduced bugs, improved code quality, and increased confidence in code changes
- **Risk if unaddressed:** Insufficient unit testing leading to undetected defects

## Considered Options

We weighed three options:
1. **Implement a linter that reports missing unit tests** for new code changes, but does not enforce it.
	* This would address the first two assessment flags and provide some visibility into missing unit tests, but would not actively encourage or ensure their presence in the codebase.
2. **Introduce a continuous integration step that runs unit tests**, which is currently running some automated tests as part of the CI/CD pipeline.
	* This would increase the coverage of automated testing, addressing the third and eighth assessment flags, but might not address all other issues.
3. **Implement a code review process that includes reviewing unit tests** for new code changes.
	* This option would actively encourage and ensure the presence of unit tests in the codebase, addressing multiple assessment flags (4, 5, 8, 10), while also improving code quality.

## Decision Outcome

Chosen: **Implement a continuous integration step that runs unit tests**, because it addresses key assessment flags while building on existing infrastructure.

### Positive Consequences

- Improved coverage of automated testing
- Enhanced confidence in code changes
- Reduced risk of undetected defects

### Negative Consequences

- May not address all issues with the testing framework or tools used for unit testing
- Does not directly enforce the presence of missing unit tests, only reports them

## Implementation Notes

1. We will integrate a new step into our CI/CD pipeline that runs unit tests as part of the automated tests.
2. The chosen implementation addresses key assessment flags and builds on existing infrastructure, minimizing additional complexity and overhead.
