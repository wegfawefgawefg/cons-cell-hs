# cons-cell-hs

Educational toy Haskell project for experimenting with custom cons cells,
laziness, and infinite structures.

Worked on: 2024-02-08

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

## Notes

- Build artifacts are created under `dist-newstyle/`.
- This project currently uses `base >=4.15 && <4.20` in `cons-cell-hs.cabal`.
