use crate::components::button::BasicButton;
use yew::prelude::*;

// Define properties for the Stepper
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct StepperProps {
    pub children: ChildrenWithProps<Step>,
    pub steps_titles: Vec<String>,
    pub final_button_text: String,
    #[prop_or(Callback::noop())]
    pub on_click_final_button: Callback<MouseEvent>,
    #[prop_or("".to_string())]
    pub button_theme_ext: String,
    #[prop_or("".to_string())]
    pub indicator_no_theme_ext: String,
    #[prop_or("".to_string())]
    pub indicator_text_theme_ext: String
}

// Stepper Component
#[function_component]
pub fn Stepper(props: &StepperProps) -> Html {
    let current_step = use_state(|| 0);
    let step_count = props.children.len();

    let indicator_class = |is_current| {
            if is_current {
                format!("bg-blue-500 text-white {}", props.indicator_no_theme_ext.clone())
            } else {
                "bg-gray-200 text-gray-800".to_string()
            }
    };
    let text_class = |is_current| {
        if is_current {
            format!("font-bold text-blue-500 {}", props.indicator_text_theme_ext.clone())
        } else {
            "text-gray-800".to_string()
        }
    };

    let onclick_next = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            if *current_step < step_count - 1 {
                current_step.set(*current_step + 1);
            }
        })
    };

    let onclick_prev = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            if *current_step > 0 {
                current_step.set(*current_step - 1);
            }
        })
    };

    html! {
        <div class="flex flex-col items-center p-4 max-w-full">
            <div class="relative flex items-center w-full mb-4">
                // Line between steps (md+ screens only)
                <div class="hidden md:flex justify-center w-full absolute top-4">
                    <div class="w-full border-t-2 border-gray-300 absolute z-0" />
                </div>
                <div class="relative flex flex-wrap md:flex-nowrap justify-center md:justify-between w-full md:space-x-2">
                    {
                        for props.steps_titles.iter().enumerate().map(|(index, title)| {
                            let is_current = index == *current_step;
                            let ind_class = indicator_class(is_current);
                            let txt_class = text_class(is_current);
                            html! {
                                <div class={format!("relative flex items-center bg-slate-50 space-x-2 mb-2 z-10 {}", if !is_current { "hidden md:flex" } else { "" })}>
                                    <div class={format!(
                                        "w-8 h-8 flex items-center justify-center rounded-full text-sm {}", ind_class
                                    )}>
                                        { index + 1 }
                                    </div>
                                    <div class={format!(
                                        "text-sm {}", txt_class
                                    )}>
                                        { title }
                                    </div>
                                </div>
                            }
                        })
                    }
                </div>
            </div>
            <div class="mb-4 p-4 border rounded w-full">
                { props.children.iter().nth(*current_step).unwrap() }
            </div>
            <div class="flex w-full justify-between">
                {
                    if *current_step == 0 {
                        html! {
                            <BasicButton button_text={""} />
                        }
                    } else {
                        html! {
                            <BasicButton onclick={onclick_prev} button_text={"Previous"} style_ext={"bg-gray-200 disabled:opacity-50"} />
                        }
                    }
                }

                {
                    if *current_step == step_count - 1 {
                        html! {
                            <BasicButton onclick={props.on_click_final_button.clone()} button_text={props.final_button_text.clone()} style_ext={format!("px-4 py-2 bg-blue-500 text-white rounded disabled:opacity-50 {}", &props.button_theme_ext)} />
                        }
                    } else {
                        html! {
                            <BasicButton onclick={onclick_next} button_text={"Next"} style_ext={format!("px-4 py-2 bg-blue-500 text-white rounded disabled:opacity-50 {}", &props.button_theme_ext)} />
                        }
                    }
                }
            </div>
        </div>
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct StepProps {
    pub children: Children,
}

#[function_component]
pub fn Step(props: &StepProps) -> Html {
    html! {
        <>
            { props.children.iter().collect::<Html>() }
        </>
    }
}
