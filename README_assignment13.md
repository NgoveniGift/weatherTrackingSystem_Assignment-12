# Assignment 13 – CI/CD Pipeline for Weather Tracking System

This README outlines the CI/CD automation and protection rules implemented for the Weather Tracking System project using **GitHub Actions**.

---

## Local Development

To build and test the project locally:

```bash
cargo build
cargo test
```

---

## CI/CD Pipeline Overview

The CI/CD pipeline is configured in `.github/workflows/ci.yml`. It performs the following:

### Continuous Integration (CI)
- **Trigger**: On `push` and `pull_request` to `main`
- **Steps**:
  - Checkout code
  - Set up stable Rust toolchain
  - Build the project
  - Run unit & integration tests

### Continuous Delivery (CD)
- **Trigger**: On direct `push` to `main` (e.g., merge)
- **Steps**:
  - Archive the full project as `weather-tracking-system.zip`
  - Upload as an artifact using GitHub Actions

Example snippet from `ci.yml`:

```yaml
release-artifact:
  if: github.ref == 'refs/heads/main' && github.event_name == 'push'
  runs-on: ubuntu-latest
  needs: build-and-test
  steps:
    - uses: actions/checkout@v3
    - run: zip -r weather-tracking-system.zip .
    - uses: actions/upload-artifact@v3
      with:
        name: weather-artifact
        path: weather-tracking-system.zip
```
---

## Branch Protection

To ensure high code quality and production stability, the following rules are enforced on the `main` branch:

- Require pull request reviews (at least 1)
- Require status checks to pass (CI must succeed)
- Block direct pushes

More details in [`PROTECTION.md`](./PROTECTION.md)

---

## Key Files

```
.github/workflows/ci.yml        # CI/CD workflow definition
PROTECTION.md                   # Branch protection rationale
openapi.yaml                    # OpenAPI spec for API documentation
```

---

## Example Screenshots (not included in markdown)

- Passing test badge
- Branch protection rule screen
- Pull request blocked by failed test
- Release artifact generated after successful merge

---



