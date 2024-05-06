[![Netlify Status](https://api.netlify.com/api/v1/badges/e2066d58-8a15-43c9-9e9a-9255733919f4/deploy-status)](https://app.netlify.com/sites/jimhillr/deploys)

# jim-hill-r.github.io

Personal GitHub profile page

# The website can be published in three ways:

- MPA (Mulit page app): This is a client side app in which each route is statically rendered and dynamic content is contained in island WASMs
- SSH (Server Side Hydrated): This is a server side app in which static content is generated and dynamic content is hydrated as needed.
- SPA (Single page app): This is a client side app fully contained within one WASM binary.
- SSR (Server Side Rendered): This is a server side app in which all routes are served/rendered by the server.

MPA is the most preferable method as it has the fastest load times, best SEO, and doesn't require a server. Ideal for marketing websites. However, this is the least stable leptos functionality and has longer build times.
SSH is likely the best for first page load and other considerations, but requires the cost of a server.
SPA is the next best if you don't have a server.
SSR is the least desirable as it doesn't hydrate and requires a server, but most stable version of this code base.

# Run local (for dev)

- Install rustup: https://rust-lang.github.io/rustup/installation/index.html
- Boostrap dev environment: `source install.sh`
- Serve the website on localhost: `cargo do dev-web`

# Create release (for CI)

- Install rustup: https://rust-lang.github.io/rustup/installation/index.html
- Boostrap dev environment: `source install.sh`
- Create build artifacts and save to docs folder: `cargo do build-web`
