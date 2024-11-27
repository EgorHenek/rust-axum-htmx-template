# Шаблон Axum

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/axum-latest-blue.svg)](https://github.com/tokio-rs/axum)
[![Bun](https://img.shields.io/badge/bun-latest-black.svg)](https://bun.sh)
[![HTMX](https://img.shields.io/badge/htmx-latest-blue.svg)](https://htmx.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

Шаблон для создания веб-приложений с использованием фреймворка Axum, включающий метрики и интеграцию с SQLite.

## Требования

- Rust (stable)
- [Bun](https://bun.sh/) - Необходим для разработки фронтенда
- SQLite

## Переменные окружения

| Переменная   | Описание                        | Значение по умолчанию |
|-------------|----------------------------------|----------------------|
| HOST        | Адрес сервера                    | 127.0.0.1           |
| PORT        | Основной порт сервера            | 3000                |
| METRICS_PORT| Порт для эндпоинта метрик        | 3001                |
| DB_URL      | URL подключения к базе данных    | sqlite:./db.sqlite  |

## Начало работы

1. Создайте собственный репозиторий из этого шаблона, нажав кнопку "Use this template" на GitHub
2. Склонируйте ваш новый репозиторий локально

3. Соберите и запустите сервер (фронтенд будет собран автоматически)
```bash
cargo run
```

## Разработка

Для режима разработки:

```bash
cargo run
```

## Возможности

- Веб-фреймворк Axum
- Интеграция с SQLite
- Эндпоинт для метрик
- Поддержка фронтенда с использованием Bun
- Конфигурация на основе переменных окружения
- Структурированное логирование

## Лицензия

Этот проект распространяется под лицензией MIT - подробности см. в файле [LICENSE](LICENSE).
