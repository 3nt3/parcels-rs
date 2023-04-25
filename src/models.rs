use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Carrier {
    DHL,
    Unknown(String),
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
