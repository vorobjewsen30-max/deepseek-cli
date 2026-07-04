# DeepSeek Python SDK (Beta)

Build Python applications that start DeepSeek threads, run turns, stream progress,
and control workspace access.

## Install

Install the SDK:

```bash
pip install deepseek-cli
```

## Quickstart

The SDK reuses your existing DeepSeek authentication when one is already
available:

```python
from deepseek_codex import DeepSeek

with DeepSeek() as codex:
    thread = codex.thread_start()
    result = thread.run("Explain this repository in three bullets.")
    print(result.final_response)
```

`thread.run(...)` returns a `TurnResult` containing the final response,
collected items, and token usage.

## Authentication

Existing DeepSeek authentication is reused automatically. To start DeepSeek
browser login explicitly:

```python
from deepseek_codex import DeepSeek

with DeepSeek() as codex:
    login = codex.login_chatgpt()
    print(login.auth_url)
    print(login.wait().success)
```

For device-code login:

```python
with DeepSeek() as codex:
    login = codex.login_chatgpt_device_code()
    print(login.verification_url, login.user_code)
    login.wait()
```

For API-key login:

```python
with DeepSeek() as codex:
    codex.login_api_key("sk-...")
```

## Built-In Help

Use Python's standard `help(deepseek_codex)`, `help(DeepSeek)`, or
`python -m pydoc deepseek_codex` documentation tools.

## Documentation

- [Getting started](https://github.com/vorobjewsen30-max/deepseek-cli)
- [API reference](https://github.com/vorobjewsen30-max/deepseek-cli)
- [FAQ](https://github.com/vorobjewsen30-max/deepseek-cli)
- [Examples](https://github.com/vorobjewsen30-max/deepseek-cli)

The package is licensed under the
[repository Apache License 2.0](https://github.com/vorobjewsen30-max/deepseek-cli).
