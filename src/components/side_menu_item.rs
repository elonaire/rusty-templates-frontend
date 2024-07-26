use crate::app::{AppRoute, Route};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SideMenuItemProps {
    pub title: String,
    pub icon: IconId,
    pub to: AppRoute,
    #[prop_or("1.5em".to_string())]
    pub icon_width: String,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component]
pub fn SideMenuItem(props: &SideMenuItemProps) -> Html {
    let icon_width = props.icon_width.clone();
    let link_classes = |is_disabled| {
        if is_disabled {
            classes!("flex", "flex-row", "items-center", "gap-2", "mb-2", "p-2", "rounded", "text-gray-400", "cursor-not-allowed")
        } else {
            classes!("flex", "flex-row", "items-center", "gap-2", "mb-2", "p-2", "rounded", "hover:bg-blue-800")
        }
    };

    let link_content = html! {
        <>
            <Icon width={icon_width.clone()} height={icon_width.clone()} icon_id={props.icon} />
            <span>{&props.title}</span>
        </>
    };

    match props.to.clone() {
        AppRoute::Route(i) => {
            if props.disabled {
                html! {
                    <span class={link_classes(true)}>
                        {link_content}
                    </span>
                }
            } else {
                html! {
                    <Link<Route> to={i} classes={link_classes(false)}>
                        {link_content}
                    </Link<Route>>
                }
            }
        },
        _ => {
            if props.disabled {
                html! {
                    <span class={link_classes(true)}>
                        {link_content}
                    </span>
                }
            } else {
                html! {
                    <Link<Route> to={Route::NotFound} classes={link_classes(false)}>
                        {link_content}
                    </Link<Route>>
                }
            }
        }
    }
}
