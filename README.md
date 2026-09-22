# The Homeric Command

A Leptos (Rust → WASM) page on **ten leadership lessons** from Homer’s *Iliad* and *Odyssey*. Styled with Tailwind CSS: bronze, wine, and marble.

## Run

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

Open [http://127.0.0.1:8080](http://127.0.0.1:8080).

Release build:

```bash
trunk build --release
```

## GitHub Pages

Push to `master`. The workflow `.github/workflows/github-pages.yml` builds with Trunk and deploys the `dist/` site.

In the repo: **Settings → Pages → Source → GitHub Actions**.

Actions used (latest majors): `actions/checkout@v7`, `actions/setup-node@v7`, `actions/configure-pages@v6`, `actions/upload-pages-artifact@v5`, `actions/deploy-pages@v5`, `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`, `taiki-e/install-action@v2`.
