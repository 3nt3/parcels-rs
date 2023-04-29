pub mod dhl;

use anyhow::Result;

use crate::models::TrackingInfo;

pub trait TrackingProvider {
    async fn track_parcel(&self, tracking_id: String) -> Result<TrackingInfo>;
}
