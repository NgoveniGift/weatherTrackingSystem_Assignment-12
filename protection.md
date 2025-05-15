# Branch Protection Rules – PROTECTION.md

To ensure a stable and secure mainline for the Weather Tracking System repository, we have enforced GitHub's branch protection rules on the `main` branch.

## Enforced Rules

| Rule                          | Description |
|------------------------------|-------------|
| Require pull request reviews | Every push to `main` must be reviewed by at least one team member. |
| Require status checks to pass | GitHub Actions CI tests must succeed before merging. |
| Block direct pushes         | Prevents accidental overwriting or unreviewed commits to the `main` branch. |

##  Why It Matters

- **Code Quality**: Requiring peer reviews ensures better design decisions and fewer bugs.
- **Test Confidence**: Mandatory CI checks verify that the code builds and tests pass.
- **Safe Collaboration**: Blocking direct pushes avoids accidental breaks to the production-ready `main` branch.
- **Compliance**: These practices align with industry-standard DevOps and software engineering principles.

## Configuration Location
Branch protection rules are configured under:
`GitHub Repository → Settings → Branches → Add Rule → main`

---


