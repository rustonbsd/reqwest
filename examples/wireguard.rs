#![deny(warnings)]

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let wg_test: String = r#"[Interface]
PrivateKey = EJHiDdrGDd1pJsr/BXoBN2r0Y7nQn6eYxgbCUfmSWWo=
Address = 10.67.65.251/24
DNS = 10.64.0.1

[Peer]
PublicKey = tzSfoiq9ZbCcE5I0Xz9kCrsWksDn0wgvaz9TiHYTmnU=
AllowedIPs = 0.0.0.0/0
Endpoint = 37.19.221.143:51820
"#.to_string();

    // Make sure you are running tor and this is your socks port
    let proxy = reqwest::Proxy::all(wg_test).expect("tor proxy should be there");
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .expect("should be able to build reqwest client");

    let res = client.get("https://am.i.mullvad.net/json").send().await?;
    println!("Status: {}", res.status());
    println!("Content: {}", res.text().await?);

    Ok(())
}
