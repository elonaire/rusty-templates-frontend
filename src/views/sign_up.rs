use crate::{components::{forms::input::{InputField, InputFieldType}, button::BasicButton}, app::Route};
use yew::prelude::*;
use yew_icons::IconId;
use yew_router::prelude::*;

#[function_component]
pub fn SignUpPage() -> Html {
    let email = use_state_eq(|| "".to_string());
    let password = use_state_eq(|| "".to_string());
    let username = use_state_eq(|| "".to_string());

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                email.set(input.value());
            }
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                password.set(input.value());
            }
        })
    };

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                username.set(input.value());
            }
        })
    };

    let is_valid = !email.clone().is_empty() && !password.clone().is_empty() && !username.clone().is_empty();

    html! {
        <div class="min-h-screen flex font-jost-sans">
            <div class="w-full md:w-1/2 flex flex-col items-center justify-center p-8 bg-white">
                <img class="my-4 w-32" src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/01f762dc-20a6-4842-30fb-2b2401c66200/public" alt="Logo" />
                <h1 class="text-4xl font-bold my-4">{"Sign Up"}</h1>
                <div class="flex w-full max-w-md justify-between my-4">
                    <BasicButton
                        button_text={"Sign up with Google".to_string()}
                        style_ext={"bg-red-500 hover:bg-red-400 transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full mr-2".to_string()}
                        onclick={Callback::from(|_| {
                            // Handle Google Sign-Up
                            gloo::console::log!("Google Sign-Up clicked");
                        })}
                        icon={Some(IconId::BootstrapGoogle)} // Assuming you have icons for Google
                        disabled={false}
                        button_type={"button".to_string()}
                        icon_before={true}
                    />
                    <BasicButton
                        button_text={"Sign up with GitHub".to_string()}
                        style_ext={"bg-gray-700 hover:bg-gray-600 transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full ml-2".to_string()}
                        onclick={Callback::from(|_| {
                            // Handle GitHub Sign-Up
                            gloo::console::log!("GitHub Sign-Up clicked");
                        })}
                        icon={Some(IconId::BootstrapGithub)} // Assuming you have icons for GitHub
                        disabled={false}
                        button_type={"button".to_string()}
                        icon_before={true}
                    />
                </div>
                <div class="w-full max-w-md flex items-center my-6">
                    <hr class="flex-grow border-t border-gray-300"/>
                    <span class="mx-4 text-gray-500">{"OR"}</span>
                    <hr class="flex-grow border-t border-gray-300"/>
                </div>
                <form class="w-full max-w-md">
                    <div class="mb-6">
                        <InputField
                            initial_value={Some((*username).clone())}
                            label={"Username".to_string()}
                            field_type={InputFieldType::Text}
                            name={"username".to_string()}
                            required={true}
                            placeholder={"Enter your username"}
                            oninput={on_username_input}

                            ext_wrapper_styles={"mb-4".to_string()}
                            ext_label_styles={"block text-gray-700 text-sm font-bold mb-2".to_string()}
                            ext_input_styles={"focus:ring-secondary"}
                            autocomplete={"on".to_string()}
                        />
                    </div>
                    <div class="mb-6">
                        <InputField
                            initial_value={Some((*email).clone())}
                            label={"Email".to_string()}
                            field_type={InputFieldType::Email}
                            name={"email".to_string()}
                            required={true}
                            placeholder={"Enter your email"}
                            oninput={on_email_input}

                            ext_wrapper_styles={"mb-4".to_string()}
                            ext_label_styles={"block text-gray-700 text-sm font-bold mb-2".to_string()}
                            ext_input_styles={"focus:ring-secondary"}
                            autocomplete={"on".to_string()}
                        />
                    </div>
                    <div class="mb-6">
                        <InputField
                            initial_value={Some((*password).clone())}
                            label={"Password".to_string()}
                            field_type={InputFieldType::Password}
                            name={"password".to_string()}
                            required={true}
                            placeholder={"Enter your password"}
                            oninput={on_password_input}

                            ext_wrapper_styles={"mb-4".to_string()}
                            ext_label_styles={"block text-gray-700 text-sm font-bold mb-2".to_string()}
                            ext_input_styles={"focus:ring-secondary"}
                            autocomplete={"on".to_string()}
                        />
                    </div>
                    <BasicButton
                        button_text={"Sign Up".to_string()}
                        style_ext={"bg-primary text-white px-4 py-2 text-sm hover:bg-secondary transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full".to_string()}
                        icon={None}
                        disabled={!is_valid}
                        button_type={"submit".to_string()}
                        icon_before={true} // if you have an icon before the button text, set it to true
                    />
                    <div class="flex items-center justify-center mt-6 text-sm text-blue-500 hover:text-blue-400">
                        <Link<Route> to={Route::SignIn}>
                            {"Already have an account? Login."}
                        </Link<Route>>
                    </div>
                </form>

            </div>
            <div class="w-full md:w-1/2 hidden md:flex items-center justify-center bg-no-repeat bg-cover bg-center bg-[url('/img/signup_background.jpg')]">
                <div class="text-center p-8">
                    // <h2 class="text-4xl text-white font-bold mb-4 drop-shadow-3xl">{ "Join Us Today" }</h2>
                    // <p class="text-white text-lg drop-shadow-3xl">{ "Create an account and start your journey with us." }</p>
                </div>
            </div>
        </div>
    }
}
