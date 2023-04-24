use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = AddParcel::register();
            _ = DeleteParcel::register();
            _ = GetParcels::register();
        }
    }
}

#[server(AddParcel, "/api", "Cbor")]
async fn add_parcel(parcel_id: String) -> Result<String, ServerFnError> {
    println!("parcel id: {parcel_id}");
    Ok(format!("Added parcel {}", parcel_id))
}

#[server(GetParcels, "/api")]
async fn get_parcels(cx: Scope) -> Result<Vec<String>, ServerFnError> {
    Ok(vec!["parcel1".to_string(), "parcel2".to_string()])
}

#[server(DeleteParcel, "/api")]
async fn delete_parcel(parcel_id: String) -> Result<String, ServerFnError> {
    Ok(format!("Deleted parcel {}", parcel_id))
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let add_parcel = create_server_multi_action::<AddParcel>(cx);
    let delete_parcel = create_server_action::<DeleteParcel>(cx);
    let submissions = add_parcel.submissions();

    let parcels = create_resource(
        cx,
        move || (add_parcel.version().get(), delete_parcel.version().get()),
        move |_| get_parcels(cx),
    );

    view! {cx,
      <h1>"Home"</h1>
      <MultiActionForm action=add_parcel>
        <label>
          "Add parcel"
          <input type="text" name="parcel_id" />
        </label>
        <input type="submit" value="Submit"/>
      </MultiActionForm>
      <Transition fallback=move || view! {cx, <div>"Loading..."</div>} >
        {move || {
          parcels.read(cx).map(move |parcels| match parcels {
            Err(why) => { view! { cx, <div>{format!("Error: {why}")}</div> }.into_view(cx) },
            Ok(parcels) => {
              parcels.into_iter().map(move |parcel| {
                view! {cx,
                  <div>
                    <span>{parcel.clone()}</span>
                    <ActionForm action=delete_parcel>
                      <input type="hidden" name="parcel_id" value={parcel.clone()} />
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
    }
}
