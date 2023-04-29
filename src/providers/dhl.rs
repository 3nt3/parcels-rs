use anyhow::Result;
use serde::Deserialize;

use crate::models::{TrackingInfo, TrackingStatus};

use super::TrackingProvider;

struct ApiResponse {
    pub shipments: Vec<Shipment>,
}

struct Shipment {
    pub events: Vec<Event>,
}

struct Event {
    pub description: String,
    pub location: Location,
    pub status: DhlTrackingStatus,
}

type DhlTrackingStatus = TrackingStatus;

impl Deserialize for DhlTrackingStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
    }
}

impl TrackingProvider for Carrier::DHL {
    async fn track_parcel(&self, tracking_id: String) -> Result<TrackingInfo> {
        // GET /track/shipments?trackingNumber=SOME_STRING_VALUE HTTP/1.1
        // Dhl-Api-Key: REPLACE_KEY_VALUE
        // Host: api-test.dhl.com

        let _ = dotenvy::dotenv();
        let api_key = std::env::var("DHL_KEY")?;

        let url = "https://api-eu.dhl.com/track/shipments";

        // example tracking id: 00340434162997311450

        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header("dhl-api-key", api_key)
            .query(&[("trackingNumber", tracking_id)])
            .send()
            .await?;

        // I have to do this before res.json consumes the response
        let status = res.status();

        let json = &res.json::<serde_json::Value>().await?;
        if !status.is_success() {
            log::warn!(
                "DHL API error (status {}): {}",
                status,
                json.get("detail").unwrap().as_str().unwrap_or_default()
            );
            return Err(anyhow::anyhow!(
                "DHL API error: {}",
                json.get("detail").unwrap().as_str().unwrap_or_default()
            ));
        }

        dbg!(&json);

        Ok(TrackingInfo { events: todo!() })
    }
}
