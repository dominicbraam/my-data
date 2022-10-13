use reqwest;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Entity {
    entity_id: String,
}

#[tokio::main]
pub async fn fan_switch(state: bool) -> Result<(), Box<dyn std::error::Error>> {

    let fan_switch = Entity {
        entity_id: "switch.lumi_lumi_plug_maus01_on_off".into(),
    };

    let url: String = "https://home.dominicbraam.com/api/services/switch/turn_".to_owned();

    let state: &str = if state {
        "on"
    } else {
        "off"
    };

    let url = url + state;

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header(AUTHORIZATION, "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIxYzZhNGY3NjRkZmQ0NjQyOGQ1NWRhM2E3ODAxOWUxZCIsImlhdCI6MTY2MzUzNTA1NywiZXhwIjoxOTc4ODk1MDU3fQ.XKGMb252-sZXEvC0K8gIM9gyKEsqluz1KZYAileLTV8")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&fan_switch)
        .send()
        .await?;

    let res = response.text().await?;

    println!("{}",res);
    
    Ok(())
}
