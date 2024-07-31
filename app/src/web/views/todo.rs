use crate::data::user::get_user;
use leptos::*;

#[component]
pub fn Todo() -> impl IntoView {
    let user = get_user();
    view! {
        <div class="bg-amber-50 min-h-screen m-0 p-0 text-slate-600">
            <ul>
                <li> "Add link from landing page to secret todo and bookmarks" </li>
                <li> "Convert profile content to markdown" </li>
                <li> {user} </li>
            </ul>
        </div>
    }
}
