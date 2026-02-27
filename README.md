# more-haskell-introduction

Educational toy project for experimenting with custom cons cells, recursion,
laziness, and infinite-style structures.

## Rough timeline

Based on git history and file timestamps, this project appears to have been worked on mostly on **February 7, 2024** (local machine time), roughly between **~6:00 AM and ~9:10 AM**.

- `git` initial commit: `2024-02-07 05:57:58 -0600`
- `app/Main.hs` modified around: `2024-02-07 08:51 -0600`
- `wtf/src/main.rs` modified around: `2024-02-07 09:07 -0600`

## Requirements

- GHC (recommended: 9.0+)
- cabal-install

## Install (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y ghc cabal-install
```

## Run

From the repository root:

```bash
cabal run
```

## Optional setup

Refresh package index if Cabal warns it is old:

```bash
cabal update
```

## What you were doing (likely)

You were doing language-learning exercises around custom linked lists and recursion, then pushing into higher-order/lazy-style patterns.

- In Haskell (`app/Main.hs`): defining a custom `List`, implementing list operations, doubling elements, and testing repeated function composition with finite prefixes.
- In Rust (`wtf/src/main.rs`): recreating a similar custom `List`, adding iterator-style traversal, and mirroring the same `take/range/double` style experiments as a cross-language comparison.

## Notes

- Build artifacts are created under `dist-newstyle/`.
