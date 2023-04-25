use leptos::*;

#[component]
pub fn Container(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    view! {
        cx,
        <div class="flex rounded-xl bg-slate-800 ring-2 ring-slate-700">
            {children(cx)}
        </div>
    }
}
