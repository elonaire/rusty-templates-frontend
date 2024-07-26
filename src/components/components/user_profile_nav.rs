use yew::*;

use crate::data::models::general::User;

#[derive(Properties, PartialEq)]
pub struct UserProfileNavProps {
    pub logged_in_user: User,
}

#[function_component]
pub fn UserProfileNav(props: &UserProfileNavProps) -> Html {
    html! {
        <div class="col-span-2 bg-blue-950 text-slate-400 border border-slate-500 rounded p-2 mb-5">
            <div class="flex flex-row items-center gap-2">
                <img class="w-10 rounded-full" src={props.logged_in_user.profile_picture.clone()} alt="dp" />
                <div class="flex flex-col">
                    <p class="text-sm">{props.logged_in_user.name.clone()}</p>
                    <p class="text-xs text-slate-500">{"Admin"}</p>
                </div>
            </div>
        </div>
    }
}
