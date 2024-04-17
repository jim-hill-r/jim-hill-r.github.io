use leptos::*;

#[component]
pub fn Todo() -> impl IntoView {
    view! {
        <div class="bg-amber-50 min-h-screen m-0 p-0 text-slate-600">
            <ul>
                <li> "Add link from landing page to secret todo and bookmarks" </li>
                <li> "Convert profile content to markdown" </li>
            </ul>
        </div>
    }
}