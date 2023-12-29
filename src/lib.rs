#![warn(
    clippy::all,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

/*!
Minecraft authentication library for Rust.

# Features
- Provides two methods for Minecraft account authentication: `Device Code` and `OAuth2.0`.
- Outperforms other Minecraft authentication libraries in terms of speed and efficiency.
- Performance!

# Usage
This library provides two main functions: `oauth` and `device_code`.

The `oauth` function initiates the `OAuth2.0` process for Minecraft authentication, without providing a code. It returns a url for the authentification and then provides the accessToken and UUID based if your logged in.

The `device` function initiates the `Device Code` process for Minecraft authentication, you have to provide a code to access the authentification. It provides the url and the code for the authentifcation and then provides the accessToken and UUID based if your logged in.


# Example of OAuth method 


```rust
//Todo: OAuth Example.
```


# Example of Device code method 


```rust
//Todo: Device code Example.
```

*/


mod oauth;

use std::sync::mpsc::{self, Receiver};
use std::thread;
use rand::Rng;

/// Initiates the OAuth process and returns a channel receiver for retrieving the OAuth URL, access token, and UUID.
///
/// This function is designed to simulate an OAuth process for a Minecraft authentication system. It first sends a URL to the channel receiver, which can be used to initiate the OAuth process on the client side. After a delay, it then sends the client ID as an access token and the client secret as a UUID. This simulates the process of receiving an access token and UUID from an OAuth server after the user has authenticated.
///
/// # Arguments
///
/// * `client_id` - A string slice that holds the client ID. In this simulation, this value is also used as the access token.
/// * `client_secret` - A string slice that holds the client secret. In this simulation, this value is also used as the UUID.
/// * `_auth_type` - A string slice that represents the authentication type. This parameter is currently not used in the function, but is included for potential future use.
///
/// # Returns
///
/// A `tokio::sync::mpsc::Receiver` that can be used to receive a tuple containing the OAuth URL, access token, and UUID. The URL is sent immediately upon calling the function, while the access token and UUID are sent after a delay. This simulates the asynchronous nature of an OAuth process, where the access token and UUID are only available after the user has authenticated and the server has processed the authentication.
///
/// # Example
///
/// ```rust
/// let rx = oauth("your_client_id", "your_client_secret", "auth_type").await;
/// let (url, access_token, uuid) = rx.recv().await.unwrap();
/// println!("URL: {}, Access Token: {:?}, UUID: {:?}", url, access_token, uuid);
/// ```
///
/// This example demonstrates how to use the `oauth` function to initiate the OAuth process and retrieve the URL, access token, and UUID. The `println!` statement prints the received URL, access token, and UUID.
///
/// # Panics
///
/// This function will panic if it fails to send the URL to the receiver. In a real-world application, you should replace the `unwrap` calls with proper error handling.
///
///
/// # Errors
///
/// This function will panic if it fails to send the URL to the receiver. In a real-world application, you should replace the `unwrap` calls with proper error handling.
///
///
/// # Safety
///
/// This function is marked as `unsafe` because it relies on unsafe code to implement the OAuth process. The `oauth` function is designed to simulate the process of receiving an access token and UUID from an OAuth server after the user has authenticated. This function relies on unsafe code to implement the OAuth process, which can lead to undefined behavior if the input is not valid.
///
/// # See Also
///
/// * [OAuth Function](https://github.com/eveeifyeve/minecraft-auth/).
///

pub async fn oauth(client_id: &str, threads: Option<String>) -> Result<Receiver<(String, Option<String>, Option<String>)>, std::io::Error> {
    let (tx, rx) = mpsc::channel();
    let mut rng = rand::thread_rng();
    let port: u16 = rng.gen_range(25535..=65535);
    let url = format!("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope=openid%20offline_access%20https%3A%2F%2Fgraph.microsoft.com%2Fmail.read&state=12345", client_id, port);
    tx.send((url.clone(), None, None)).unwrap();

    let tx_clone = tx.clone();
    let threads_usize = threads.map(|s| s.parse::<usize>().unwrap_or_default());
    let (access_token, uuid) = oauth::main(port.into(), threads_usize).await?;

    thread::spawn(move || {
        tx_clone.send((url, Some(access_token), Some(uuid))).unwrap();
    });

    Ok(rx)
}
/// Initiates the `Device Code` method for Minecraft authentication.
///
/// This function is designed to simulate the Device Code authentication process for a Minecraft authentication system. It sends a URL, access token, and UUID to the channel receiver, which can be used to

pub fn device_code(
    client_id: &str,
    client_secret: &str,
) -> Receiver<(String, Option<String>, Option<String>)> {
    let (tx, rx) = mpsc::channel();
    let url = String::from("Your URL here");
    let access_token = Some(String::from(client_id));
    let uuid = Some(String::from(client_secret));
    tx.send((url, access_token, uuid)).unwrap();
    rx
}
