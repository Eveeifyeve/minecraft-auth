[![Crates.io](https://img.shields.io/crates/v/minecraft-auth.svg?color=orange)](https://crates.io/crates/minecraft-auth)
[![Workflow Status](https://github.com/eveeifyeve/minecraft-auth/workflows/main/badge.svg)](https://github.com/eveeifyeve/minecraft-auth/actions?query=workflow%3A%22main%22)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# Minecraft-Auth

Minecraft authentication library for Rust.

## Features

- Complete - Supports the two main auth methods: OAuth 2.0 and Device Code.
- Fast - outperforms other authentication libraries
- Safe - `#![forbid(unsafe_code)]`

## Usage

This library provides two main structures: `Oauth` and `DeviceCode`.

## Examples

### OAuth

```rust,ignore
use minecraft_auth::*;

let client_id = "111231209837123098712";

let oauth = Oauth::new(client_id);
println!("Login here: {}", oauth.url());

let oauth_info = oauth.launch().await?;
```
