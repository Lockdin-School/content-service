## Conventional Commit Guide

Use [Conventional Commits](https://www.conventionalcommits.org/) to keep the project history readable, consistent, and easy to automate.

### Commit Format

```aiignore
(optional scope):
```

### Common Commit Types

| Type | Use for |
| --- | --- |
| `feat` | New features or user-facing functionality |
| `fix` | Bug fixes |
| `docs` | Documentation-only changes |
| `style` | Formatting, whitespace, or code style changes that do not affect behavior |
| `refactor` | Code changes that neither fix a bug nor add a feature |
| `perf` | Performance improvements |
| `test` | Adding or updating tests |
| `build` | Build system, dependency, or bundler changes |
| `ci` | CI/CD workflow changes |
| `chore` | Maintenance tasks that do not affect source behavior |
| `revert` | Reverting a previous commit |

### Recommended Scopes for This Codebase

Use scopes to identify the area of the project being changed.

| Scope          | Use for                                       |
|----------------|-----------------------------------------------|
| `api`          | API-layer handlers logic                      |
| `infra`        | External components setup for service support |
| `domain`       | domain-specific code changes                  |
| `repository`   | Repository-layer logic                        |
| `service`      | Service-layer logic                           |
| `model`        | Model definition logic                        |
| `config`       | Routes, Cors, State, Server                   |
| `deps`         | Dependency updates                            |
| `docker`       | Dockerfile or compose setup                   |
| `readme`       | README updates                                |
| `contributing` | CONTRIBUTING updates                          |

### Common Commit Message Examples

#### Features

```
feat(app): add main application layout 
feat(pages): add dashboard page 
feat(subjects): add subjects list page 
feat(subjects): add subject detail view 
feat(topics): add topic API module 
feat(components): add reusable video player 
feat(components): add lessons list component 
feat(api): add shared API client 
feat(styles): add responsive page spacing
```

#### Fixes
```
fix(app): correct route rendering 
fix(api): handle empty API responses 
fix(api): use correct base request configuration 
fix(subjects): prevent crash when subject data is missing 
fix(topics): handle missing topic fields 
fix(components): prevent video player from rendering without a source 
fix(styles): correct mobile layout overflow
```
#### Documentation
```
docs(readme): update project setup instructions 
docs(readme): document available npm scripts 
docs(contributing): add conventional commit guide 
docs(api): document API module usage
```

#### Styling
```
style(styles): format global CSS 
style(components): clean up component class names 
style(app): improve layout spacing 
style(subjects): align subject card styling
```

#### Refactors
```
refactor(api): centralize request configuration 
refactor(components): simplify header markup 
refactor(subjects): extract subject list rendering 
refactor(pages): reorganize dashboard page structure 
refactor(app): simplify route composition
```

#### Performance
```text
perf(components): reduce unnecessary renders 
perf(video): optimize video player loading 
perf(styles): remove unused classes
```

#### Tests
```text
test(components): add video player render tests 
test(api): add API client tests 
test(subjects): add subject page tests
```

#### Build, Tooling, and Configuration
```text
build(deps): update React dependencies 
build(deps): update Vite dependency 
build(config): update TypeScript configuration 
build(config): update Vite configuration 
build(docker): add Docker build setup 
build(docker): update compose configuration
```

#### CI and Maintenance
```text
ci: add lint workflow 
ci: add build verification workflow 
chore: clean up unused files 
chore(config): update oxlint rules 
chore(deps): refresh package lockfile 
chore(project): organize source folders
```

#### Reverts
```
revert: revert subject detail page changes 
revert(api): revert API client refactor
```

### Writing Good Commit Descriptions

Keep commit descriptions:

- Short and clear
- Written in the imperative mood
- Lowercase unless using a proper noun
- Focused on what changed, not how difficult it was

Good:
```text
fix(api): handle failed subject requests
```

Avoid:
```text
fixed stuff updated files changes work in progress
```

### Breaking Changes
For breaking changes, add `!` after the type or scope:
```text
feat(api)!: change subject response format
```
You can also include a footer:
```text
BREAKING CHANGE: subject responses now return items instead of data.
```