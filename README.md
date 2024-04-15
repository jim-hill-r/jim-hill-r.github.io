[![Netlify Status](https://api.netlify.com/api/v1/badges/e2066d58-8a15-43c9-9e9a-9255733919f4/deploy-status)](https://app.netlify.com/sites/jimhillr/deploys)

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
