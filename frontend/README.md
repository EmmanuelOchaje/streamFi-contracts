# frontend

This directory is a **components-only** library, not a standalone app.

## Build tooling

There is intentionally no `vite.config.ts`, `next.config.js`, or other
bundler config here. `css-modules.d.ts` only teaches TypeScript how to type
`*.module.css` imports (`import styles from './Foo.module.css'`) — it does
not imply this directory produces its own build output.

These components (and their co-located `*.module.css` files) are expected
to be imported directly into a host application's existing build (e.g. a
Next.js or Vite app elsewhere in the Conduit stack), which supplies the
actual CSS Modules loader/bundler. If that assumption changes and this
directory needs to ship its own build, add the bundler config then —
until a host app exists, adding one here would be unused surface area to
maintain.

## Scripts

```bash
npm run typecheck   # tsc --noEmit
npm run lint        # eslint .
npm test            # vitest run
```

These are also run in CI on every push/PR that touches `frontend/` — see
`.github/workflows/ci.yml`.
