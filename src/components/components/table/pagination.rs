use yew::prelude::*;
use yew_icons::{IconId, Icon};

#[derive(Properties, PartialEq, Clone)]
pub struct PaginationProps {
    pub current_page: usize,
    pub total_pages: usize,
    pub on_page_change: Callback<usize>,
}

#[function_component]
pub fn Pagination(props: &PaginationProps) -> Html {
    let next_page = if props.current_page < props.total_pages {
        Some(props.current_page + 1)
    } else {
        None
    };

    let prev_page = if props.current_page > 1 {
        Some(props.current_page - 1)
    } else {
        None
    };

    let on_prev_click = {
        let on_page_change = props.on_page_change.clone();
        let prev_page = prev_page.unwrap_or(props.current_page);
        Callback::from(move |_| on_page_change.emit(prev_page))
    };

    let on_next_click = {
        let on_page_change = props.on_page_change.clone();
        let next_page = next_page.unwrap_or(props.current_page);
        Callback::from(move |_| on_page_change.emit(next_page))
    };

    let on_first_click = {
        let on_page_change = props.on_page_change.clone();
        Callback::from(move |_| on_page_change.emit(1))
    };

    let on_last_click = {
        let on_page_change = props.on_page_change.clone();
        let last_page = props.total_pages;
        Callback::from(move |_| on_page_change.emit(last_page))
    };

    html! {
        <div class="flex justify-between items-center mt-4">
            <div></div>
            <div class="flex items-center">
                <span class="text-xs mr-2">{ format!("{} of {}", props.current_page, props.total_pages) }</span>
                <button
                    class="bg-blue-200 text-gray-800 py-1 px-3 border-l border-y border-blue-500 rounded-l cursor-pointer"
                    disabled={props.current_page <= 1}
                    onclick={on_first_click}
                >
                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronBarLeft}/>
                </button>
                <button
                    class="bg-blue-200 text-gray-800 py-1 px-3 border-l border-y border-blue-500 cursor-pointer"
                    disabled={props.current_page == 1}
                    onclick={on_prev_click}
                >
                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronLeft}/>
                </button>
                <button
                    class="bg-blue-200 text-gray-800 py-1 px-3 border-l border-y border-blue-500 cursor-pointer"
                    disabled={props.total_pages <= 1 || props.current_page == props.total_pages}
                    onclick={on_next_click}
                >
                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronRight}/>
                </button>
                <button
                    class="bg-blue-200 text-gray-800 py-1 px-3 border-x border-y border-blue-500 rounded-r cursor-pointer"
                    disabled={props.current_page >= props.total_pages}
                    onclick={on_last_click}
                >
                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronBarRight}/>
                </button>
            </div>
        </div>
    }
}
