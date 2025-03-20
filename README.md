# Invest API SDK (Rust)
Invest API SDK is a Rust library that provides a convenient wrapper for interacting with [Invest API](https://russianinvestments.github.io/investAPI/)

The SDK uses [Tonic](https://github.com/hyperium/tonic) to generate gRPC clients and allows configuring interceptors for each service separately.

## Features

- Easy setup of a gRPC channel for Invest API interaction
- Ability to use interceptors for each service
- Auto-generated clients based on Tonic
- Full compatibility with Invest API

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
invest-api-rust-sdk = "0.2"

```

## Usage

You can find usage examples in the examples directory.

Before running, set the TOKEN environment variable with your API access token (examples use SANDBOX).

## Important Notice

Use this crate at your own risk. The author assumes no responsibility for any issues arising from its usage.

# Invest API SDK (Rust)

Invest API SDK — это библиотека на языке Rust, предоставляющая удобную оболочку для взаимодействия с [Invest API](https://russianinvestments.github.io/investAPI/) через gRPC.

SDK использует [Tonic](https://github.com/hyperium/tonic) для генерации gRPC-клиентов и предоставляет возможность конфигурировать interceptor'ы для каждого сервиса отдельно.

## Возможности

- Простая настройка gRPC-канала для взаимодействия с Invest API
- Возможность использования interceptor'ов для каждого сервиса
- Генерируемые клиенты на основе Tonic
- Полная совместимость с Invest API

## Установка

Добавьте в ваш `Cargo.toml`:

```toml
[dependencies]
invest-api-rust-sdk = "0.2"
```

## Использование

Примеры использования можно посмотреть в директории `examples`

Перед запуском необходимо установить переменную окружения `TOKEN` в которой будет указан токен-доступа к API endpoint (в примерах используется SANDBOX)


## Важное уведомление

Используйте данный `crate` на свой страх и риск. Автор не несет никакой ответственности за использование данного `crate`
