# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- (Work in progress)

## [0.1.0] - 2025-11-01
### Added
- Initial public release (first version).
- Basic interactive shell loop that executes external commands and supports simple pipelines.
- Colored ASCII banner and `username$ /path` prompt (shortens `$HOME` to `~` and compresses long paths).
- History saved to a cross-platform path (`$HOME/.hcshell_history` on Unix-like systems, `%USERPROFILE%\\.hcshell_history` on Windows; falls back to the OS temp directory).
- Uses `rustyline` for line editing and command history.

### Notes
- Build with `cargo build` (debug) or `cargo build --release` (optimized).
- See `README.md` for run/install instructions and platform notes.

---

> This changelog follows the "Keep a Changelog" style. Update the `Unreleased` section for future changes and move entries into versioned sections when releasing.
