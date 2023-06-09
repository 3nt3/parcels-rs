use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;

use crate::components::container::*;
use crate::models::{Carrier, Parcel};
use crate::providers::{self, dhl};

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
        _ = TrackParcel::register();
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
  }
}

impl std::fmt::Display for Carrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Carrier::DHL => write!(f, "DHL"),
            Carrier::Unknown(carrier) => write!(f, "Unknown carrier '{}'", carrier),
        }
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

#[server(GetParcels, "/api")]
async fn get_parcels(cx: Scope) -> Result<Vec<Parcel>, ServerFnError> {
    let mut conn = db().await?;

    let parcels = sqlx::query_as::<_, Parcel>("select * from parcels")
        .fetch_all(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    dbg!(&parcels);

    Ok(parcels)
}

#[server(DeleteParcel, "/api")]
async fn delete_parcel(parcel_id: i32) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    sqlx::query!("delete from parcels where id = $1;", parcel_id)
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}

#[server(TrackParcel, "/api")]
async fn track_parcel(parcel_id: i32) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    let parcel = sqlx::query_as::<_, Parcel>("select * from parcels where id = $1")
        .bind(parcel_id)
        .fetch_one(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match parcel.carrier {
        Carrier::DHL => dhl::track_parcel(parcel.tracking_id)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string())),
        _ => Err(ServerFnError::ServerError(format!(
            "Unknown carrier: {}",
            parcel.carrier
        ))),
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let delete_parcel = create_server_action::<DeleteParcel>(cx);
    let add_parcel = create_server_multi_action::<AddParcel>(cx);

    let parcels = create_resource(
        cx,
        move || (add_parcel.version().get(), delete_parcel.version().get()),
        move |_| get_parcels(cx),
    );

    view! {cx,
      <div class="w-[600px] flex flex-col gap-8 items-start">
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
                    <ParcelItem clone:parcel parcel={parcel} delete_parcel=delete_parcel />
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

/// Render a single parcel tile
#[component]
fn ParcelItem(
    cx: Scope,
    parcel: Parcel,
    delete_parcel: Action<DeleteParcel, Result<(), ServerFnError>>,
) -> impl IntoView {
    let track_parcel = create_server_action::<TrackParcel>(cx);

    view! {
      cx,
      <Container>
        <div class="flex flex-col items-start">
          <div>{parcel.carrier.to_string()}</div>
          <ActionForm action=delete_parcel clone:parcel>
            <input type="hidden" name="parcel_id" value={parcel.id} />
            <input type="submit" value="Delete"/>
          </ActionForm>
          <ActionForm action=track_parcel clone:parcel>
            <input type="hidden" name="parcel_id" value={parcel.id} />
            <input type="submit" value="Track"/>
          </ActionForm>
        </div>
      </Container>
    }
}
