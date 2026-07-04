# DeepSeek CLI — агент для командной строки на базе DeepSeek

**DeepSeek CLI** — это open-source агент для написания кода на Rust, работающий локально на вашем компьютере через API DeepSeek.

Интерфейс полностью на русском языке.

## Быстрый старт

### Сборка из исходников

```shell
git clone https://github.com/vorobjewsen30-max/deepseek-cli.git
cd deepseek-cli/codex-rs
cargo build --release
```

### Настройка

1. Получите API-ключ на [platform.deepseek.com/api_keys](https://platform.deepseek.com/api_keys)
2. Установите переменную окружения:
   ```shell
   export DEEPSEEK_API_KEY="sk-..."
   ```
3. Или введите ключ при первом запуске — программа сама предложит его ввести.

### Запуск

```shell
./target/release/deepseek
```

## Модели

- **deepseek-v4-flash** (по умолчанию) — быстрая и эффективная модель для повседневных задач
- **deepseek-v4-pro** — мощная модель для сложных задач и глубокого анализа

## Как получить API-ключ DeepSeek

1. Зайдите на [platform.deepseek.com](https://platform.deepseek.com)
2. Зарегистрируйтесь или войдите в аккаунт
3. Перейдите в раздел **API Keys**
4. Нажмите «Создать новый ключ»
5. Скопируйте ключ и вставьте его при запуске DeepSeek CLI

## Лицензия

MIT License. См. файл [LICENSE](LICENSE).
