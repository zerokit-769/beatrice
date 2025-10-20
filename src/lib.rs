mod common;
mod config;
mod proxy;

use crate::config::Config;
use crate::proxy::*;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
use worker::*;
use once_cell::sync::Lazy;
use regex::Regex;

static PROXYIP_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+-\d+$").unwrap());

#[event(fetch)]
async fn main(req: Request, env: Env, _: Context) -> Result<Response> {
    let uuid = env
        .var("UUID")
        .map(|x| Uuid::parse_str(&x.to_string()).unwrap_or_default())?;
    let host = req.url()?.host().map(|x| x.to_string()).unwrap_or_default();
    let config = Config { uuid, host: host.clone(), proxy_addr: host, proxy_port: 80};

    Router::with_data(config)
        .on("/link", link)
        .on_async("/:proxyip", tunnel)
        .on_async("/Benxx-Project/:proxyip", tunnel)
        .run(req, env)
        .await
}

async fn tunnel(req: Request, mut cx: RouteContext<Config>) -> Result<Response> {
    if let Some(proxyip) = cx.param("proxyip") {
        if PROXYIP_PATTERN.is_match(proxyip) {
            if let Some((addr, port_str)) = proxyip.split_once('-') {
                if let Ok(port) = port_str.parse() {
                    cx.data.proxy_addr = addr.to_string();
                    cx.data.proxy_port = port;
                }
            }
        }
    }
    
    let upgrade = req.headers().get("Upgrade")?.unwrap_or("".to_string());
    if upgrade == "websocket".to_string() {
        let WebSocketPair { server, client } = WebSocketPair::new()?;
        server.accept()?;
    
        wasm_bindgen_futures::spawn_local(async move {
            let events = server.events().unwrap();
            if let Err(e) = ProxyStream::new(cx.data, &server, events).process().await {
                console_log!("[tunnel]: {}", e);
            }
        });
    
        Response::from_websocket(client)
    } else {
        let req = Fetch::Url(Url::parse("https://example.com")?);
        req.send().await
    }
}

fn link(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    #[derive(Serialize)]
    struct Link {
        description: String,
        link: String,
    }

    fn link(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    #[derive(Serialize)]
    struct Link {
        description: String,
        link: String,
    }

    let vmess_link = {
        let config = json!({
            "ps": "siren vmess",
            "v": "2",
            "add": host,
            "port": "80",
            "id": uuid,
            "aid": "0",
            "scy": "zero",
            "net": "ws",
            "type": "none",
            "host": host,
            "path": "/8.222.161.217-443",
            "tls": "",
            "sni": "",
            "alpn": ""}
        );
        format!("vmess://{}", URL_SAFE.encode(config.to_string()))
    };
    let vless_link = format!("vless://{uuid}@{host}:443?encryption=none&type=ws&host={host}&path=%2FKR&security=tls&sni={host}#siren vless");
    let trojan_link = format!("trojan://{uuid}@{host}:443?encryption=none&type=ws&host={host}&path=%2FKR&security=tls&sni={host}#siren trojan");
    let ss_link = format!("ss://{}@{host}:443?plugin=v2ray-plugin%3Btls%3Bmux%3D0%3Bmode%3Dwebsocket%3Bpath%3D%2FKR%3Bhost%3D{host}#siren ss", URL_SAFE.encode(format!("none:{uuid}")));
    
    Response::from_json(&Link {
        link,
                description:
            "helo world".to_string()
    })
}
