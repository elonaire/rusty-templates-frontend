use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct PopoverProps {
    #[prop_or_default]
    pub children: Children,
    pub display_item: Html,
    #[prop_or("bottom".to_string())]
    pub position: String,
    #[prop_or_default]
    pub style_ext: String,
}

#[function_component]
pub fn Popover(props: &PopoverProps) -> Html {
    let show_popover = use_state(|| false);

    // Close the popover when clicking outside of it
    let on_click_outside = {
        log::info!("Clicked outside");
        let show_popover = show_popover.clone();
        Callback::from(move |_: MouseEvent| {
            show_popover.set(false);
        })
    };

    // Toggle the popover display
    let toggle_popover = {
        let show_popover = show_popover.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation(); // stop the event from propagating upwards
            show_popover.set(!*show_popover);
        })
    };

    // Determine the classes for the position and arrow
    let (position_class, _arrow_class) = match props.position.as_str() {
        "top" => ("bottom-full mb-2", "border-b-gray-800"),
        "bottom" | _ => ("top-full mt-2", "border-t-gray-800"),
    };

    let open_classes = if *show_popover {
        "opacity-100 transform translate-y-0 visible"
    } else {
        "opacity-0 transform translate-y-2 invisible"
    };

    html! {
        <>
            { if *show_popover {
                html! {
                    <div onclick={on_click_outside.clone()} class="fixed inset-0 z-40 bg-transparent"></div>
                }
            } else {
                html! {}
            }}
            <div class="relative flex justify-center items-center">
                <div onclick={toggle_popover} class="cursor-pointer">
                    {props.display_item.clone()}
                </div>

                <div class={format!("absolute {} md:min-w-48 bg-slate-50 border-[1px] border-gray-200 shadow-lg text-white text-sm rounded transition-all duration-300 z-50 right-0 {} {}", position_class, open_classes, props.style_ext)}>
                    { for props.children.iter() }
                </div>
            </div>
        </>
    }
}
