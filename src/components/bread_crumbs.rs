use std::collections::HashMap;

use crate::app::{AppRoute, Route};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct BreadcrumbItem {
    pub name: String,
    pub link: AppRoute,
}

#[function_component]
pub fn Breadcrumbs() -> Html {
    let location = use_location().expect("No location available");
    let location_clone = location.clone();
    let breadcrumbs = use_state_eq(|| vec![] as Vec<BreadcrumbItem>);
    let breadcrumbs_clone = breadcrumbs.clone();

    use_effect_with_deps(
        move |_| {
            let path_segments = location.path().split('/').collect::<Vec<_>>();

            // let mut breadcrumbs: Vec<BreadcrumbItem> = vec![];
            let mut cumulative_path = String::new();
            let mut new_crumbs = vec![] as Vec<BreadcrumbItem>;

            for segment in path_segments.iter().filter(|&&segment| !segment.is_empty()) {
                cumulative_path.push('/');
                cumulative_path.push_str(segment);
                new_crumbs.push(BreadcrumbItem {
                    name: segment.to_string(),
                    link: match Route::from_path(cumulative_path.clone().as_str(), &HashMap::new()) {
                        Some(route) => AppRoute::Route(route),
                        None => {
                            AppRoute::Route(Route::NotFound)
                            // AppRoute::WizardsRoute(WizardsRoute::from_path(cumulative_path.clone().as_str(), &HashMap::new()).unwrap())
                        }
                    }
                });
            }

            // append the home link
            new_crumbs.insert(
                0,
                BreadcrumbItem {
                    name: "Dashboard".to_string(),
                    link: AppRoute::Route(Route::Landing),
                },
            );

            breadcrumbs_clone.set(new_crumbs);
        },
        location_clone,
    );

    html! {
        <nav class="p-3 rounded">
            <ul class="flex items-center space-x-2">
                {
                    breadcrumbs.iter().enumerate().map(|(i, item)| {
                        html!{
                            <li class="flex flex-row gap-2 items-center">
                                {
                                    match item.link.clone() {
                                        AppRoute::Route(i) => {
                                            html!{
                                                <Link<Route> to={ i }
                                                    classes="hover:text-blue-800"
                                                >
                                                    { item.name.clone() }
                                                </Link<Route>>
                                            }
                                        },
                                            _ => {
                                                html!{
                                                    <Link<Route> to={ Route::NotFound }
                                                        classes="hover:text-blue-800"
                                                    >
                                                        { item.name.clone() }
                                                    </Link<Route>>
                                                }
                                            }
                                    }
                                }

                            if i < breadcrumbs.len() - 1 { <span class="text-xs mx-2"><Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronRight}/></span> } else {}
                        </li>
                    }
                    }).collect::<Html>()
                }
            </ul>
        </nav>
    }
}
