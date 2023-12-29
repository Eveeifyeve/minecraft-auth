mod server;

pub async fn main(port: usize, threads: Option<usize>) -> std::io::Result<(String, String)> {
    let threads = threads.unwrap_or_else(|| 1);
    let result = server::main(port, Some(threads));
    let (mc_token, state) = match result {
        Ok(value) => value,
        Err(e) => panic!("Server main function failed: {:?}", e),
    };
    if state != "12345" || mc_token == "" {
        panic!("State is not default");
    }
    Ok((mc_token, state))
}
