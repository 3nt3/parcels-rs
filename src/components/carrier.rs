use leptos::*;
use crate::models::Carrier;

#[component]
pub fn CarrierIcon<'a>(cx: Scope, carrier: &'a Carrier) -> impl IntoView {
  view! {
    cx,
    <img class="h-8 w-8" src=format!("/assets/carriers/{}.svg", carrier.to_string())/>
  }
}
