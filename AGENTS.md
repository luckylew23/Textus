# Agent guide

We use AI coding assistants on Textus ourselves, and so do some of our
contributors. This file is what we hand them: where things are, and the
few rules that have gone wrong before and must not again. The full rules
are in [CONTRIBUTING.md](CONTRIBUTING.md) and the documents it links; if
this file and they ever disagree, they win - and please tell us.

## Where things are

Textus is an Integrated Writing Environment (IWE) for Markdown on macOS,
Windows, Linux, iPhone and iPad,
built with Tauri 2: a React + TypeScript frontend in `src/` (the editor is
Milkdown/ProseMirror, source mode CodeMirror 6) and a Rust backend in
`src-tauri/`.

| Need                                                   | Read                                                             |
| ------------------------------------------------------ | ---------------------------------------------------------------- |
| Setup, the checks to run, code style, writing          | [CONTRIBUTING.md](CONTRIBUTING.md)                               |
| Folders, layers and what ESLint enforces; the backend  | [docs/dev/architecture.md](docs/dev/architecture.md)             |
| How to add a setting, menu item, command, string, test | [docs/dev/recipes.md](docs/dev/recipes.md)                       |
| The one place for each common need: reuse it           | [docs/dev/reuse.md](docs/dev/reuse.md)                           |
| Tabs, opening, saving, drafts, closing                 | [docs/dev/documents.md](docs/dev/documents.md)                   |
| Which test layer, the e2e helpers, CI                  | [docs/dev/testing.md](docs/dev/testing.md)                       |
| Branches, pull requests, commits, releases             | [docs/dev/releasing.md](docs/dev/releasing.md)                   |
| Desktop and iOS: what is shared, what is gated         | [docs/dev/ios-code-map.md](docs/dev/ios-code-map.md)             |
| The interface's design standard                        | [docs/design/design-language.md](docs/design/design-language.md) |
| Every other document                                   | [docs/README.md](docs/README.md)                                 |

## Rules

- **Branches and pull requests** follow
  [releasing.md](docs/dev/releasing.md#pull-requests-and-commits): work goes
  to `main` from a branch, never pushed directly; plain English commit
  messages; don't merge a pull request yourself unless asked.
- **Run the checks** before you say a change is done: `npm run check:quick`
  while you work (only your files), and `npm run check` before a pull
  request ([CONTRIBUTING.md](CONTRIBUTING.md#before-you-push)).
- **Don't change CI or the test setup to make a check pass**: `.github/`,
  `playwright*.config.ts`, `vite*.config.ts`, `eslint.config.js`,
  `tsconfig*.json`, `scripts/check-*.mjs`, `e2e/helpers.ts`, or a test's
  expectation. Fix the code; if a check is truly wrong, change it in a pull
  request of its own that says why.
- **Reproduce a bug before fixing it**, in the engine where it was seen,
  and add a test in the lowest layer that catches it; check that it fails
  without the fix ([testing.md](docs/dev/testing.md)).
- **Check native behaviour in the built app** (`npm run tauri build -- --debug`):
  the window bar, title bar and traffic lights, drag regions, full screen,
  the side panels' layout and motion, `Cmd` shortcuts that have a menu
  item, file dialogs, input methods. A browser can't show these. Say in
  the pull request what you ran.
- **Generated files are never edited by hand**: `src/platform/bindings/`
  (`npm run bindings`), `docs/dev/plugins/reference.md`
  (`node scripts/plugin-reference.mjs`). The checks fail while they are
  stale.
- **Drag with pointer events** (`src/engine/drag/pointer-drag.ts`), never HTML drag
  and drop: the window's file drop swallows it.
- **The e2e plugin flows need a textus-plugins checkout** beside the
  repository or at `TEXTUS_PLUGINS_DIR`; without one the catalog is empty
  ([testing.md](docs/dev/testing.md#the-official-plugins)).
- **Keep the code shape** in [code-style.md](docs/dev/code-style.md)
  (file size, hooks a component, pure logic apart from side effects,
  provider differences as data). Make the smallest change that fixes the cause, match the
  surrounding code, and write comments that say why, not what.
- **Startup**: anything first used after a click or a keystroke loads
  through `lazyModule` (`src/lazy/lazy-module.ts`); the startup layer
  imports none of it. A pull request that grows the entry chunk says by how
  much, and why it can't wait.
- **Every user-visible string goes into all ten locale files** in
  `src/i18n/locales/` ([recipes.md](docs/dev/recipes.md#add-a-ui-string)).
- **Shapes shared by Rust and TypeScript** carry a comment on both sides
  naming the other; change both.
- **Interface work** follows [design-language.md](docs/design/design-language.md):
  tokens only (no raw colour, radius or font size in a component), and its
  review checklist (section 17) before a pull request that changes how
  something looks.
- **iOS is the same app in a narrower layout**: gate only what the platform
  can't do, with `#[cfg(desktop)]` and the flags in
  `src/platform/capabilities.ts`, never to hide a feature
  ([ios-code-map.md](docs/dev/ios-code-map.md)).
- **WebView2 can be as old as Chromium 105**: newer CSS goes behind
  `@supports` ([webview-support.md](docs/dev/webview-support.md)).
- **A release note** is a new file under `docs/releases/next/`
  ([its README](docs/releases/next/README.md)), never an edit to
  `docs/releases/X.Y.Z.md`. Documentation and notes are English.

## Safety

- Never commit or print secrets: anything named `*.local`, API keys, tokens,
  `AuthKey_*.p8`, signing keys.
- Never run an installed Textus (`/usr/bin/textus`, `/Applications/Textus.app`)
  for testing, and never point a test build at a real user's data. Run
  `npm run tauri dev` with throwaway data folders (on Linux set
  `XDG_DATA_HOME`, `XDG_CONFIG_HOME`, `XDG_CACHE_HOME` and `XDG_STATE_HOME`
  to a temporary directory; [storage.md](docs/dev/storage.md) lists the
  folders on each platform).
- Never `rm -rf` a path built from a variable; move files to the trash or a
  scratch folder instead. Don't kill processes you didn't start.
- Don't drive the user's desktop (synthetic key presses, focusing windows)
  without being asked.
- The updater: Textus publishes `textus-latest.json`, never `latest.json`
  (that name belongs to old Levis installs). `npm run check` guards this;
  don't work around it.
- Don't change signing, notarization, release or store workflows unless the
  task is about them.

## Fork releases (luckylew23/Textus)

This fork releases from `.github/workflows/release-fork.yml`, modeled on
the Tydora fork's pipeline: pushing a `v*.*.*` tag builds and publishes
directly, or
`gh workflow run -R luckylew23/Textus release-fork.yml
-f tag=<tag> --ref fork`. One build job per platform
feeds one publish job, so no two builds race for the release. What the
first fork release (v1.4.1-rc.3, 2026-11-13) established; drop this
section when syncing from upstream:

- **Version**: follow upstream's version and raise only the rc suffix
  (`1.4.1-rc.N`). Every file `scripts/check-versions.mjs` lists moves
  together (package.json, package-lock.json, src-tauri/tauri.conf.json,
  src-tauri/Cargo.toml, Cargo.lock); the publish job re-checks the tag.
  The tag must exist on the remote before dispatching.
- **Updater signing**: the fork has its own `TAURI_SIGNING_PRIVATE_KEY`
  (created 2026-11-13, empty password) in the Actions secrets; fork
  releases never publish latest.json, so the key is not the upstream one.
- **macOS unsigned**: no Apple Developer account, so release-fork.yml
  sets none of the APPLE secrets and tauri build ships unsigned DMGs
  (Gatekeeper: right-click - Open on first launch).
- **Platforms**: only macOS Apple-silicon DMG and Windows x64 (NSIS +
  portable zip). The windows job adds a portable zip built with
  `TEXTUS_DISTRIBUTION=portable` (`Textus/` plus `TextusData/` at the
  zip's root).
- **Upstream's release.yml is disabled on this fork** (Actions settings)
  and carries no fork edits - re-enable it and delete release-fork.yml
  when returning to the upstream flow. Only this AGENTS.md section and
  release-fork.yml are fork files, so upstream sync never conflicts.
- **Pushing workflow files needs SSH**: the gh OAuth token has no
  `workflow` scope, so an https push that adds or changes anything under
  `.github/workflows/` is rejected ("refusing to allow an OAuth App...")
  - long after the real error surfaced as connect timeouts. The origin
    points at `git@github.com:...` (SSH, port 22 works here) and pushes
    workflow commits fine.
- **A workflow_dispatch needs the workflow on the default branch**;
  release-fork.yml is only on `fork`. Releasing from the
  branch therefore means: move the tag onto a commit that carries
  release-fork.yml and push it (`git tag -f v... <commit> && git push -f
origin v...`) - the tag push event runs the workflow from the commit
  the tag points at.
- **Network**: pushes and Actions checks fail when GitHub is unreachable
  from the intranet. A failed or cancelled run is no loss: dispatch the
  same tag again after the network is back.

## Fork work log (luckylew23/Textus, 2026-11-13)

State and lessons from the first fork work session, so a later agent can
continue without rediscovering them. Branch `fork` (HEAD
`58fc240d`); first fork release `v1.4.1-rc.3` published (see Fork releases
above). Local checkout: `~/Work/Textus`.

### What the fork adds so far

State after the 2026-11-19 rebase onto upstream `main`
(`51f4e3b9`, one CI commit past `1.4.6-rc.1`). Upstream had by then shipped, in its own better forms, the
proxy credentials and direct loopback (1.4.1) and the whole
system-proxy-following route design (`aicompat::http::Route`,
hyper-util's matcher, `proxy_url()` resolving the system's proxy -
1.4.5), so the fork's versions of those were dropped again. What is
still fork-only after this rebase:

- **Fork release pipeline** (`49f80fd9`): `release-fork.yml`, see
  Fork releases above. Upstream's `release.yml` is untouched and
  disabled on the fork.
- **SOCKS5 really connects** (`6df7d571`): reqwest gains the `socks`
  feature (upstream's reqwest has neither `socks` nor `system-proxy`;
  it reads the system proxy itself through hyper-util). Without `socks`
  a SOCKS5 setting was accepted but every request failed before it left
  the computer. Selecting SOCKS5 builds a `socks5h://` proxy URL, so
  names are looked up at the proxy, which this computer's own DNS
  cannot do. A regression test talks to a local SOCKS server and checks
  the greeting. `reqwest13` is a direct, unused dependency purely for
  feature unification: tauri-plugin-updater's own reqwest 0.13 client
  needs `socks` for a SOCKS5 proxy during update checks and installs.
- **The portable zip builds like upstream's** (`80b40997`): a second
  Windows build with `TEXTUS_DISTRIBUTION=portable` (`TextusData`
  beside the exe), staged for the publish job - the earlier 7z path
  packaging buried the app two folders down.
- **Windows "follow the system" reads ProxyServer ourselves**
  (2026-11-19, rc.3): hyper-util's registry reader hands the raw
  `ProxyServer` value to a URI parser, which drops the per-protocol
  form (`http=...;https=...`) - requests then went direct and failed
  with a name error on an intranet, while the browser read the same
  setting and worked. aicompat now reads the registry first
  (`windows-registry`, already in the tree) and normalizes both shapes
  (`parse_windows_proxy_server`, tested everywhere; the registry read
  is `#[cfg(windows)]`, so it only type-checks on a Windows build -
  there is no windows target std installed on this machine, rustup
  cannot download it). A catalog failure also reports the proxy in
  effect (`aicompat::http::proxy_display`, credentials masked).
- **Mermaid diagrams draw when a document opens, and wide ones fit**
  (2026-11-24): two editor bugs behind "cannot render mermaid erDiagram,
  11.17.2" - which was never a mermaid bug (11.17.2 and 12.3.0 render ER
  identically; upgrading would not have helped). One: a document whose
  first block is a mermaid diagram showed source, no drawing, because the
  opening caret sits in that block and reveal treated it as being edited;
  a drawing now remembers the selection it was drawn for
  (`reveal-state.ts` `drawnAt`), a just-opened document draws for "no
  one" (`pointer-hold.ts` exports `NOWHERE`), and clicking a drawing
  reveals its source through a `"reveal"` transaction meta
  (`mermaidKey`, moved to `mermaid-blocks.ts`). The math/html/embeds
  reveal plugins share the opening behavior; only mermaid was changed.
  Two: 显示不全 - wide ER diagrams were cut at the column edge by the
  65% min-width hold meant for strips; `keepReadable` now holds only
  gantt/timeline sources (their SVG carries no class to tell them by),
  everything else fits the column. Lives on
  `fix/diagram-first-block` (from upstream `main` `8b911c18`, commit
  `44df7d90`) for the upstream PR, cherry-picked onto `fork`
  (`f8f2a50d`) for the next fork release; neither pushed at the time of
  writing. Tests: `src/editor/mermaid-blocks.test.ts` (new) and
  `reveal-state.test.ts` additions; `npm run check:quick` green and
  playground-verified on both bases. Release note:
  `docs/releases/next/diagram-first-block-and-fit.md`. Still open: a
  diagram that fails to parse shows no error anywhere
  (`mermaid-plugin.ts` catch).

### On GitHub (state as of 2026-11-13)

- **Fork**: `luckylew23/Textus` (parent/upstream:
  `CatVinci-Studio/Textus`). Working model, since 2026-11-19: fork
  `main` mirrors upstream only (fast-forward on every sync, never
  conflicts); all fork work lives on `fork`, rebased
  onto upstream's `main` when syncing; contributions to upstream go
  from a clean `fix/*` branch derived from upstream's `main` (see
  `fix/intranet-proxy`), which carries no version bump and no
  fork-only files. One branch per contribution, deleted after the PR
  merges; a fork commit whose change has landed upstream is then
  skipped automatically at the next rebase (it rebases to empty).
- **Tag** `v1.4.1-rc.3` points at `6ce7e11a` (it was force-moved there so
  the tag carries release-fork.yml; re-pointing a tag like this is normal
  here - no published release depends on the old commit).
- **Release** `v1.4.1-rc.3` is published as a prerelease with six assets:
  `Textus_1.4.1-rc.3_aarch64.dmg` (unsigned), `..._x64-setup.exe`,
  `..._x64_portable.zip` (a `Textus/` folder at the zip's root), plus the
  updater artifacts (`Textus.app.tar.gz(.sig)`, `..._x64-setup.exe.sig`).
- **Actions**: upstream `release.yml` is DISABLED on the fork;
  `release-fork.yml` is the only release workflow (tag push or dispatch).
  Secrets: `TAURI_SIGNING_PRIVATE_KEY` (fork's own key, 2026-11-13, empty
  password) and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (empty) are set;
  there are no APPLE_* secrets and none are needed.
- **Updater chain**: fork releases never publish latest.json, and the
  fork-built app's updater endpoint stays upstream's - publishing on the
  fork cannot affect upstream users, and fork builds won't be offered
  upstream updates until upstream ships a stable release.

### This machine's environment

- **cargo**: direct crates.io hangs. Use a throwaway CARGO_HOME with the
  rsproxy mirror:
  `mkdir -p /tmp/textus-cargo-home && printf '[source.crates-io]\nreplace-with = "rsproxy-sparse"\n\n[source.rsproxy-sparse]\nregistry = "sparse+https://rsproxy.cn/index/"\n' > /tmp/textus-cargo-home/config.toml`
  then `CARGO_HOME=/tmp/textus-cargo-home cargo ...` (deps for this repo
  are already cached there).
- **npm**: the default npm cache has root-owned files (EPERM errors).
  Install with `npm install --cache /tmp/npm-cache-textus`.
- **Pre-existing test failures**: on a clean `main`, 28 vitest tests fail
  with `localStorage` being undefined (6 files, e.g.
  `src/ai/chat/conversation/conversation-model.test.ts`) even though jsdom
  is installed - a machine-side issue, not the repo's. Before blaming your
  change, run the failing file on a clean tree.
- **macOS loopback binding**: only `127.0.0.1` can be bound (no
  `127.0.0.2+` aliases), so loopback-family tests fall back to
  `127.0.0.1` locally; Linux CI binds the whole /8 and covers the rest.

### Lessons

- **An https push that touches `.github/workflows/` is rejected** ("OAuth
  App ... without `workflow` scope") - the gh token lacks that scope, and
  the rejection sometimes surfaces as a connect timeout, which reads like
  a network outage for an hour. Pushing over SSH works: origin is
  `git@github.com:luckylew23/Textus.git`.
- **A workflow_dispatch needs the workflow file on the default branch.**
  From a branch, move the tag onto a commit that carries the workflow and
  push it (`git tag -f v... <commit> && git push -f origin v...`); the
  tag push event runs the workflow from the commit the tag points at.
- **Check what actually failed before assuming the network**: `git
ls-remote` and `gh api` succeeding while `git push` fails means the
  token scope, not the intranet.
- **TDD against a shared-global Rust module**: tests that mutate a
  process-global (e.g. the proxy URL static) must live in ONE test
  function - parallel tests otherwise leak state into each other.
