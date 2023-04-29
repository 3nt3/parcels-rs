use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Carrier {
    DHL,
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackingInfo {
    events: Vec<TrackingEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackingEvent {
    status: TrackingStatus,
    location: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingStatus {
    InTransit,
    Delivered,
    Exception,
    Unknown,
}

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
    pub struct Parcel {
      pub id: i32,
      pub tracking_id: String,
      pub created_at: chrono::DateTime<chrono::Utc>,
      pub carrier: Carrier
    }
  } else {
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Parcel {
      pub id: i32,
      pub tracking_id: String,
      pub created_at: chrono::DateTime<chrono::Utc>,
      pub carrier: Carrier
    }
  }
}

impl Serialize for Carrier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Carrier::DHL => "dhl",
            Carrier::Unknown(carrier) => carrier,
        })
    }
}

impl<'de> Deserialize<'de> for Carrier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "dhl" => Carrier::DHL,
            _ => Carrier::Unknown(s),
        })
    }
}
