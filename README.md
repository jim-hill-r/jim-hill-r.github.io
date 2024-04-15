[![Nightly](https://github.com/jim-hill-r/jim-hill-r.github.io/actions/workflows/nightly.yaml/badge.svg)](https://github.com/jim-hill-r/jim-hill-r.github.io/actions/workflows/nightly.yaml)

# jim-hill-r.github.io

Personal GitHub profile page

# Run local (for dev)

- Install rustup: https://rust-lang.github.io/rustup/installation/index.html
- Boostrap dev environment: `source install.sh`
- Serve the website on localhost: `cargo run-script start`

# Create release (for CI)

- Install rustup: https://rust-lang.github.io/rustup/installation/index.html
- Boostrap dev environment: `source install.sh`
- Create build artifacts and save to docs folder: `cargo run-script release`
