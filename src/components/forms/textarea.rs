use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or("".to_string())]
    pub initial_value: String,
    pub label: String,
    pub name: String,
    pub input_node_ref: Option<NodeRef>,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or(Callback::noop())]
    pub oninput: Callback<InputEvent>,
    #[prop_or("".to_string())]
    pub ext_input_styles: String,
}

#[function_component]
pub fn Textarea(props: &TextAreaProps) -> Html {
    let TextAreaProps {
        initial_value,
        label,
        name,
        input_node_ref,
        readonly,
        required,
        placeholder,
        oninput,
        ext_input_styles,
    } = props;

    let display_error = use_state(|| false);

    html! {
        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for={name.clone()}>
                { label }
                {
                    if *required {
                        html!{ <span class="text-red-500">{ "*" }</span> }
                    } else {
                        html!{}
                    }
                }
            </label>
            <textarea
                class={format!("form-input ring-0 shadow appearance-none border border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent flex-grow {}", ext_input_styles)}
                value={initial_value.clone()}
                name={name.clone()}
                ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                readonly={*readonly}
                oninput={oninput.clone()}
                placeholder={placeholder.clone()}
                id={name.clone()}
            />
            <p class="text-red-500 text-xs italic">{
                if *display_error {
                    "This field is required"
                } else {
                    ""
                }
            }</p>
        </div>
    }
}
