#![deny(warnings)]

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let wg_test: String = r#"[Interface]
# Device: Warm Kiwi
PrivateKey = wN8rzyW34ZonBJtp8WxZbpSADQvpZYfG29ZnDNNBkEc=
Address = 10.64.210.67/32,fc00:bbbb:bbbb:bb01::1:d242/128
DNS = 10.64.0.1

[Peer]
PublicKey = ov323GyDOEHLT0sNRUUPYiE3BkvFDjpmi1a4fzv49hE=
AllowedIPs = 0.0.0.0/0,::0/0
Endpoint = 193.32.126.66:51820
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
