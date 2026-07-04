# DeepSeek CLI Runtime for Python SDK

Platform-specific runtime package consumed by the published `deepseek-cli`.

This package is staged during release so the SDK can pin an exact DeepSeek CLI
version without checking platform binaries into the repo.

`deepseek-cli-cli-bin` is intentionally wheel-only. Do not build or publish an
sdist for this package.
