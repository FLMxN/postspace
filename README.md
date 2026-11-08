# Postspace

Postspace is a browser-only political profile quiz with 84 questions, seven axes, ideological matches, and nearby public figures. Answers and calculations stay in the browser; the site does not need a server or API.

## Run locally

```sh
npm ci
npm run dev
```

Create the static production site in `dist/` with:

```sh
npm run build
```

The contents of `dist/` are plain static HTML, CSS, JavaScript, and assets. They can be served by GitHub Pages or any static file host.

## Publish on GitHub Pages

1. Push this repository to GitHub on the `main` branch.
2. In **Settings → Pages → Build and deployment**, choose **GitHub Actions** as the source.
3. The included workflow builds the site and publishes `dist/` after each push to `main`. You can also start it from the **Actions** tab with **Run workflow**.

The Vite build uses relative asset paths, so it works both at a custom domain and at the usual `https://<user>.github.io/<repository>/` address.

## Rust CLI

The separate Rust CLI can still be run with `cargo run`. It is not needed to build or host the website.
