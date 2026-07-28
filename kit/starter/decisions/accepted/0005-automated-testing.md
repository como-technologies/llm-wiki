# ADR-0005: Automated Testing

> State: Accepted

## Status

Accepted
Created: 2026-06-12

## Stakeholders

_Who owns this decision, and who needs to sign off? List the roles or people involved._

<!-- adroit:seeded-from-assessment -->

<!-- adroit:ai-suggested -->

## Context and Problem Statement
Use of automated testing tools

> Seeded from assessment "Software Engineering Maturity Assessment" — domain "Delivery Pipeline" → practice "Automated Testing".

The assessment flagged:
- Are automated tests run as part of the CI/CD pipeline?
- Is test coverage above 80%?
- Are automated tests run regularly, such as daily or weekly?
- Is there a documented process for creating and maintaining automated tests?
- Are automated tests run on all code changes, including pull requests?
- Is there a clear understanding of what constitutes 'good' test coverage?
- Are automated tests run on all environments, including staging and production?
- Is there a process for identifying and addressing test failures?
- Are automated tests run on code changes that introduce new functionality?
- Is there a clear understanding of what constitutes 'good' test quality?
- Are automated tests run on all types of code changes, including bug fixes and refactors?
- Is there a process for reviewing and approving test results?

## Decision Drivers

- **Why it matters:** Reduced testing time and increased test coverage
- **Risk if unaddressed:** Insufficient test coverage and delayed detection of defects

## Considered Options

We weighed two options:
### Option 1: Implement automated testing tools with limited scope

* Adopt a minimal set of automated tests for critical components, focusing on ensuring high test coverage (>90%) in the short term.
* This approach allows us to establish a baseline for our testing practices and build momentum gradually.

### Negative Consequences
- Limited scope may lead to incomplete testing, potentially overlooking defects or regressions.
- Gradual implementation might not yield immediate results, delaying the benefits of reduced testing time.

### Option 2: Implement automated testing tools with comprehensive coverage

* Develop a comprehensive suite of automated tests covering all code changes and environments, aiming for a target test coverage of >95%.
* This approach enables us to achieve high test quality and catch defects early, but may require significant upfront investment in development time and infrastructure.

### Negative Consequences
- High upfront costs might divert resources from other critical areas.
- Complexity could lead to increased maintenance burdens and testing difficulties over time.

## Decision Outcome
Chosen: **Implement automated testing tools with comprehensive coverage**, because it aligns better with the long-term goal of achieving high test quality, reducing testing time, and ensuring more reliable releases.

### Positive Consequences

* Achieving high test quality through comprehensive coverage will reduce the likelihood of defects slipping through.
* Early detection of issues will enable quicker fixes and minimize delays in release cycles.
* Comprehensive coverage will establish a strong foundation for our testing practices, setting us up for future improvements.

## Negative Consequences
- High upfront costs may divert resources from other critical areas.
- Complexity could lead to increased maintenance burdens and testing difficulties over time.

## Implementation notes
### adroit plan

1. Develop comprehensive automated test suites for all code changes and environments.
2. Establish a documented process for creating, maintaining, and reviewing automated tests.
3. Integrate automated testing tools into our CI/CD pipeline and conduct regular testing cycles.
4. Allocate sufficient resources to support the development and maintenance of automated testing infrastructure.

By implementing comprehensive automated testing, we can establish a robust foundation for high-quality releases while minimizing potential risks and complexities.

## Implementation

<!-- adroit:plan -->

### Automated Testing Implementation Plan
#### Checklist

1. **Develop Comprehensive Automated Test Suites**
	* Components/Files: `tests`, `test-infrastructure`
	* Tasks:
		+ Develop automated tests for all code changes and environments.
		+ Ensure comprehensive coverage (>95%).
		+ Prioritize testing critical components first.
2. **Establish Documented Process for Creating, Maintaining, and Reviewing Automated Tests**
	* Components/Files: `test-process`, `test-docs`
	* Tasks:
		+ Create a standardized process for test creation and maintenance.
		+ Define clear guidelines for reviewing and approving test results.
		+ Ensure all team members understand the testing process.
3. **Integrate Automated Testing Tools into CI/CD Pipeline**
	* Components/Files: `ci-pipeline`, `test-tooling`
	* Tasks:
		+ Integrate automated testing tools with existing CI/CD pipeline.
		+ Conduct regular testing cycles (e.g., daily or weekly).
		+ Ensure high test coverage (>90%) in the short term.
4. **Allocate Sufficient Resources for Automated Testing Infrastructure**
	* Components/Files: `test-infrastructure`, `ci-pipeline`
	* Tasks:
		+ Allocate necessary resources for automated testing infrastructure development.
		+ Ensure sufficient infrastructure to support comprehensive testing.
		+ Monitor and adjust resource allocation as needed.

#### Rollout/Migration

1. **Phased Implementation**: Implement automated testing tools with comprehensive coverage in phases, starting with critical components and environments.
2. **Gradual Transition**: Gradually transition from manual testing to automated testing over time, allowing for a smooth transition and minimizing disruptions.
3. **Monitoring and Feedback**: Continuously monitor test results and gather feedback from the development team to identify areas for improvement.

#### Testing

1. **Unit Tests**: Develop comprehensive unit tests to cover all code changes and environments.
2. **Integration Tests**: Develop integration tests to ensure seamless interactions between components and systems.
3. **End-to-End Tests**: Develop end-to-end tests to simulate real-world user scenarios and validate application functionality.

#### Risks

1. **High Upfront Costs**: High costs associated with implementing comprehensive automated testing infrastructure may divert resources from other critical areas.
2. **Complexity**: Increased complexity in the testing process may lead to increased maintenance burdens and testing difficulties over time.
3. **Resource Allocation**: Insufficient resource allocation for automated testing infrastructure development may result in suboptimal results.

#### Next Steps

1. **Develop Comprehensive Automated Test Suites**:
	* Schedule dedicated time for test development and maintenance.
	* Prioritize critical components and environments first.
2. **Establish Documented Process for Creating, Maintaining, and Reviewing Automated Tests**:
	* Create a standardized process document and share with the team.
	* Ensure all team members understand the testing process.
3. **Integrate Automated Testing Tools into CI/CD Pipeline**:
	* Schedule dedicated time for pipeline integration and testing cycle setup.

By following this implementation plan, we can establish a robust foundation for high-quality releases while minimizing potential risks and complexities.

<!-- /adroit:plan -->
