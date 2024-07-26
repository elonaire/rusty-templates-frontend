use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub initial_value: Option<String>,
    #[prop_or("".to_string())]
    pub label: String,
    pub field_type: InputFieldType,
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
    #[prop_or(Callback::noop())]
    pub onclick: Callback<MouseEvent>,
    #[prop_or("".to_string())]
    pub ext_wrapper_styles: String,
    #[prop_or("".to_string())]
    pub ext_label_styles: String,
    #[prop_or("".to_string())]
    pub ext_input_styles: String,
    #[prop_or("on".to_string())]
    pub autocomplete: String,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum InputFieldType {
    Text,
    Email,
    Date,
    Number,
    Password,
    Tel,
    Url,
    Search,
    Color,
    Range,
    File,
    Hidden,
    Image,
    Month,
    Time,
    Week,
}

#[function_component]
pub fn InputField(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        initial_value,
        label,
        field_type,
        name,
        input_node_ref,
        readonly,
        required,
        placeholder,
        oninput,
        onclick,
        ext_wrapper_styles,
        ext_label_styles,
        ext_input_styles,
        autocomplete,
    } = props;

    let display_error = use_state(|| false);
    let input_field_type_str = match field_type {
        InputFieldType::Text => "text",
        InputFieldType::Email => "email",
        InputFieldType::Date => "date",
        InputFieldType::Number => "number",
        InputFieldType::Password => "password",
        InputFieldType::Tel => "tel",
        InputFieldType::Url => "url",
        InputFieldType::Search => "search",
        InputFieldType::Color => "color",
        InputFieldType::Range => "range",
        InputFieldType::File => "file",
        InputFieldType::Hidden => "hidden",
        InputFieldType::Image => "image",
        InputFieldType::Month => "month",
        InputFieldType::Time => "time",
        InputFieldType::Week => "week",
    };

    html! {
        <div class={format!("{}", ext_wrapper_styles)}>
            <label class={format!("block text-gray-700 text-sm font-bold {}", ext_label_styles)} for={name.clone()}>
                { label }
                {
                    if required.clone() {
                        html!{ <span class="text-red-500">{ "*" }</span> }
                    } else {
                        html!{}
                    }
                }
            </label>
            <input
                class={format!("form-input focus:ring-0 shadow appearance-none border-2 border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-blue-500 flex-grow {}", ext_input_styles)}
                type={input_field_type_str.to_string()}
                value={initial_value.clone().unwrap_or("".to_string())}
                name={name.clone()}
                ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                readonly={readonly.clone()}
                oninput={oninput.clone()}
                placeholder={placeholder.clone()}
                autocomplete={autocomplete.clone()}
                id={name.clone()}
                onclick={onclick.clone()}
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
