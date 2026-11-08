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

The contents of `dist/` are static HTML, CSS, JavaScript, and assets, ready for GitHub Pages or another static host.

## Publish on GitHub Pages

In **Settings → Pages → Build and deployment**, choose **GitHub Actions** as the source. The included workflow builds and publishes `dist/` after each push to `main` or `master`; you can also start it from the **Actions** tab.

Relative asset paths support both a custom domain and the usual `https://<user>.github.io/<repository>/` address.

## Rust CLI

The separate Rust CLI can still be run with `cargo run`. It is not needed to build or host the website.
