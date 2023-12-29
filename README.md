[![Crates.io](https://img.shields.io/crates/v/minecraft-auth.svg?color=orange)](https://crates.io/crates/minecraft-auth)
[![Workflow Status](https://github.com/eveeifyeve/minecraft-auth/workflows/main/badge.svg)](https://github.com/eveeifyeve/minecraft-auth/actions?query=workflow%3A%22main%22)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# Minecraft-Auth

Minecraft authentication library for Rust.

## Features
- Provides two methods for Minecraft account authentication: `Device Code` and `OAuth2.0`.
- Outperforms other Minecraft authentication libraries in terms of speed and efficiency.
- Performance!

## Usage
This library provides two main functions: `oauth` and `device_code`.

The `oauth` function initiates the `OAuth2.0` process for Minecraft authentication, without providing a code. It returns a url for the authentification and then provides the accessToken and UUID based if your logged in.

The `device` function initiates the `Device Code` process for Minecraft authentication, you have to provide a code to access the authentification. It provides the url and the code for the authentifcation and then provides the accessToken and UUID based if your logged in.


## Example of OAuth method


```rust
//Todo: OAuth Example.
```


## Example of Device code method


```rust
//Todo: Device code Example.
```

