use leptos::*;

#[component]
pub fn Container(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    view! {
        cx,
        <div class="bg-slate-800 drop-shadow-xl p-4 rounded-xl w-full ring-2 ring-slate-700">
            {children(cx)}
        </div>
    }
}
