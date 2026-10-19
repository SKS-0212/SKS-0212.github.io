Rustfolio
A Rust-powered static portfolio generator that turns project metadata and GitHub repositories into a personal website.
Rustfolio builds a static portfolio from a TOML configuration, local Rust projects, and public Rust repositories discovered through the GitHub API. It generates a homepage, individual project pages, and copies frontend assets into a deployable `dist/` directory.
Live portfolio (after deployment): https://sks-0212.github.io  
Source repository: https://github.com/SKS-0212/SKS-0212.github.io
Features
GitHub discovery: Finds public, non-fork Rust repositories from a configured GitHub account.
Local project scanning: Reads `Cargo.toml` and `README.md` from configured project directories.
README support: Accepts regular Markdown READMEs or optional YAML frontmatter for extra metadata.
Project merging: Combines local and GitHub projects, with local projects taking priority when names normalize to the same slug.
Static site generation: Produces a portfolio homepage and dedicated project pages.
Customizable profile: Configures name, headline, biography, skills, experience, and links using TOML.
Responsive frontend: Ships static CSS and JavaScript assets without requiring a JavaScript build system.
Safer publishing: Stages output before replacing the previous build, with a backup and rollback attempt if publication fails.
Automated deployment: Supports publishing `dist/` through GitHub Actions and GitHub Pages.
Tech stack
Language: Rust
HTTP client: `reqwest` (blocking client)
Configuration and metadata: `toml`, `serde`, `serde_yaml_ng`
GitHub content decoding: `base64`
Output: Static HTML, CSS, and JavaScript
Hosting: GitHub Pages
Getting started
Prerequisites
A current stable Rust toolchain (`rustc` and `cargo`)
Git
Internet access when GitHub discovery is enabled
Clone and run
```bash
git clone https://github.com/SKS-0212/SKS-0212.github.io.git
cd SKS-0212.github.io
cargo test
cargo run --release
```
The generated site is written to `dist/`. Open `dist/index.html` in a browser to inspect the output. For a more representative local preview, serve `dist/` with a simple static HTTP server.
Configuration
Edit `rustfolio.toml` in the repository root:
```toml
# Local projects are optional. Use paths available on the build machine.
projects = []

[profile]
name = "Sachin Kumar Singh"
headline = "Software Engineer | Rust & Systems Programming"
bio = "Building reliable software and exploring systems programming with Rust."
location = "India"
skills = ["Rust", "Flutter", "Java", "Spring Boot", "PostgreSQL"]

[github]
username = "SKS-0212"

[[profile.links]]
label = "GitHub"
url = "https://github.com/SKS-0212"

[[profile.links]]
label = "LinkedIn"
url = "https://www.linkedin.com/in/sachin-kumar-singh-371710230/"
```
To include a local project, add its directory to `projects`:
```toml
projects = ["../hexview", "../rsh"]
```
> **CI note:** Relative local paths must exist on the GitHub Actions runner. If those directories are not checked out as part of the workflow, use `projects = []` and GitHub discovery instead.
Optional README metadata
A project README can contain YAML frontmatter:
```markdown
---
category: Systems Programming
featured: true
technologies:
  - Rust
  - CLI
---

# Hexview

A streaming hexadecimal file viewer written in Rust.
```
Ordinary Markdown READMEs without frontmatter are also supported. When frontmatter is absent, category and technologies default to empty values and `featured` defaults to `false`.
Project structure
```text
rustfolio/
├── .github/workflows/deploy.yml  # GitHub Pages workflow
├── assets/                       # CSS and JavaScript
├── src/
│   ├── config/                   # TOML configuration models and loading
│   ├── model/                    # Portfolio domain models
│   ├── scanner/                  # Cargo and README parsing
│   ├── github/                   # GitHub API client and decoding
│   ├── service/                  # Project loading, discovery, merging
│   ├── generator/                # HTML rendering, paths, writing
│   │   └── html/                 # Page sections and templates
│   └── main.rs                   # Application entry point
├── rustfolio.toml                # Portfolio configuration
├── Cargo.toml
└── Cargo.lock
```
`dist/` and `target/` are generated directories and should not be committed.
How it works
```text
rustfolio.toml ───────────────┐
                             │
Local Cargo.toml + README ───┼──> Project loading and merging
                             │               │
GitHub API + repository files┘               v
                                       Portfolio model
                                             │
                                             v
                                       HTML generation
                                             │
                                             v
                                      Staging directory
                                             │
                                             v
                                      dist/ + assets
                                             │
                                             v
                                        GitHub Pages
```
Rustfolio normalizes project names into URL-safe slugs, creates an HTML page for each project, and generates links from the homepage. The writer prepares the new site in a staging directory before replacing the previous output. This improves recoverability but is not a fully atomic or crash-proof deployment mechanism.
Testing
Run the full offline test suite:
```bash
cargo test
```
Check formatting:
```bash
cargo fmt --check
```
Some live GitHub API tests are marked `#[ignore]` because they require network access. To run ignored tests explicitly:
```bash
cargo test -- --ignored
```
The project uses test-driven development for parser behavior, project discovery, HTML generation, and output writing.
Deploying to GitHub Pages
The repository includes `.github/workflows/deploy.yml` for GitHub Actions deployment.
Push the project to the `main` branch of `SKS-0212/SKS-0212.github.io`.
In repository Settings → Pages, set Build and deployment → Source to GitHub Actions.
The workflow should run tests, generate the site using `cargo run --locked --release`, upload `dist/`, and deploy it to Pages.
Check the Actions tab for deployment status.
After a successful deployment, visit https://sks-0212.github.io.
> The site URL is a deployment target, not proof that the site is currently live. If a workflow fails, review its logs and verify that `rustfolio.toml` does not reference unavailable local directories.
Current scope and limitations
GitHub discovery targets public, non-fork repositories identified as Rust projects by GitHub.
GitHub API availability and rate limits can affect repository discovery.
The current command-line entry point builds the site; it is not a full-featured CLI with subcommands.
Build backup/rollback reduces risk but does not guarantee uninterrupted availability or automatic crash recovery.
Author
Sachin Kumar Singh  
GitHub · LinkedIn