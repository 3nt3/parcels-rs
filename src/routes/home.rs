use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::components::container::*;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use sqlx::{Connection, PgConnection, postgres::PgRow};

    pub async fn db() -> Result<PgConnection, ServerFnError> {
      PgConnection::connect("postgres://parcels:parcels@localhost:46257/parcels").await.map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    pub fn register_server_functions() {
        _ = AddParcel::register();
        _ = DeleteParcel::register();
        _ = GetParcels::register();
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
    pub struct Parcel {
      pub id: i32,
      pub tracking_id: String,
      pub created_at: chrono::DateTime<chrono::Utc>,
      pub carrier: Carrier
    }


    impl sqlx::Decode<'_, sqlx::Postgres> for Carrier {
      fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
      ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let carrier = String::decode(value)?;
        Ok(match carrier.as_str() {
          "dhl" => Carrier::DHL,
          _ => Carrier::Unknown(carrier),
        })
      }
    }

    impl sqlx::Type<sqlx::Postgres> for Carrier {
      fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("varchar")
      }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Carrier {
    DHL,
    Unknown(String),
}

impl std::fmt::Display for Carrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Carrier::DHL => write!(f, "dhl"),
            Carrier::Unknown(carrier) => write!(f, "{}", carrier),
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

#[server(AddParcel, "/api")]
pub async fn add_parcel(parcel_id: String) -> Result<String, ServerFnError> {
    let mut conn = db().await?;

    let lol = sqlx::query!(
        "insert into parcels (tracking_id) values ($1) returning id",
        parcel_id
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?
    .id;

    println!("Added parcel {} ({})", parcel_id, &lol);
    Ok(format!("Added parcel {} ({})", parcel_id, &lol))
}

#[server(GetParcels, "/api", "getjson")]
async fn get_parcels(cx: Scope) -> Result<Vec<Parcel>, ServerFnError> {
    let mut conn = db().await?;

    let parcels = sqlx::query_as::<_, Parcel>("select * from parcels")
        .fetch_all(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(parcels)
}

#[server(DeleteParcel, "/api")]
async fn delete_parcel(parcel_id: String) -> Result<String, ServerFnError> {
    Ok(format!("Deleted parcel {}", parcel_id))
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let add_parcel = create_server_multi_action::<AddParcel>(cx);
    let delete_parcel = create_server_action::<DeleteParcel>(cx);

    let parcels = create_resource(
        cx,
        move || (add_parcel.version().get(), delete_parcel.version().get()),
        move |_| get_parcels(cx),
    );

    view! {cx,
      <div class="max-w-screen-sm overflow-hidden flex flex-col gap-8 items-start">
        <h1 class="mt-8 text-slate-50 text-4xl font-bold">"Home"</h1>
        <Container>
          <MultiActionForm action=add_parcel class="flex flex-col gap-4 items-start">
            <label class="text-2xl flex flex-col items-start">
              "Add parcel"
              <input type="text" name="parcel_id" class="bg-slate-700 shadow-slate-900" />
            </label>
            <input type="submit" value="Submit"/>
          </MultiActionForm>
        </Container>
        <Transition fallback=move || view! {cx, <div>"Loading..."</div>} >
          {move || {
            parcels.read(cx).map(move |parcels| match parcels {
              Err(why) => { view! { cx, <div>{format!("Error: {why}")}</div> }.into_view(cx) },
              Ok(parcels) => {
                parcels.into_iter().map(move |parcel: Parcel| {
                  view! {cx,
                    <div>
                      <div>{format!("{:?}", &parcel)}</div>
                      <ActionForm action=delete_parcel>
                        <input type="hidden" name="parcel_id" value={&parcel.tracking_id.clone()} />
                        <input type="submit" value="Delete"/>
                      </ActionForm>
                    </div>
                  }
                }).collect::<Vec<_>>()
                .into_view(cx)
              }
            }).unwrap_or_default()
          }}
        </Transition>
      </div>
    }
}
