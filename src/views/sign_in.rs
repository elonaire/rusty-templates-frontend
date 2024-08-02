use std::ops::Deref;

use crate::{app::{AppStateContext, Route}, components::{button::BasicButton, forms::input::{InputField, InputFieldType}}, data::{context::users::sign_in, models::user::{LoginForm, LoginPayload, Logins}}};
use yew::prelude::*;
use yew_icons::IconId;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

#[function_component]
pub fn SignInPage() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let login_form = use_state_eq(|| LoginForm::default());
    let login_form_is_valid = use_state_eq(|| false);
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let current_state_clone = current_state.clone();
        let login_form = login_form.clone();
        let navigator_clone = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let logins = Logins {
                user_name: Some(login_form.username.clone()),
                password: Some(login_form.password.clone()),
                oauth_client: None
            };

            let payload = LoginPayload {
                raw_user_details: logins
            };

            let current_state_clone = current_state_clone.clone();
            let navigator_clone = navigator_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {

                let _sign_in = sign_in(&current_state_clone, payload).await;
                navigator_clone.push(&Route::Cart);
            });

        })
    };
    // reusing a handler is much cleaner
    let oninput = {
        let login_form_clone = login_form.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target_dyn_into::<HtmlInputElement>();

            // match your name props for your text fields
            match target {
                Some(target) => match target.name().as_str() {
                    "password" => login_form_clone.set(LoginForm {
                        password: target.value(),
                        ..login_form_clone.deref().clone()
                    }),
                    "username" => login_form_clone.set(LoginForm {
                        username: target.value(),
                        ..login_form_clone.deref().clone()
                    }),

                    _ => {}
                },
                None => {
                    // If its a textarea, the exception will be handled here e.g
                    // let target = event.target_dyn_into::<HtmlTextAreaElement>();
                    // match target {
                    //     Some(target) => match target.name().as_str() {
                    //         "text_area_name" => {},
                    //         _ => {}
                    //     },
                    //     None => {}
                    // }
                }
            }
        })
    };

    let login_form_clone_deps = login_form.clone();
    let login_form_is_valid_clone = login_form_is_valid.clone();
    use_effect_with_deps(
        move |_| {
            // set valid to true if none of the fields are empty
            login_form_is_valid_clone.set(!login_form_clone_deps.deref().username.is_empty() && !login_form_clone_deps.deref().password.is_empty());
            || ()
        },
        login_form.clone(),
    );

    html! {
        <div class="min-h-screen flex font-jost-sans">
            <div class="w-full md:w-1/2 flex flex-col items-center justify-center p-8 bg-white">
                <img class="my-4 w-32" src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/01f762dc-20a6-4842-30fb-2b2401c66200/public" alt="Logo" />
                <h1 class="text-4xl font-bold my-4">{"Log In"}</h1>
                <div class="flex w-full max-w-md justify-between my-4">
                    <BasicButton
                        button_text={"Sign in with Google".to_string()}
                        style_ext={"bg-red-500 hover:bg-red-400 transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full mr-2".to_string()}
                        onclick={Callback::from(|_| {
                            // Handle Google Sign-In
                            gloo::console::log!("Google Sign-In clicked");
                        })}
                        icon={Some(IconId::BootstrapGoogle)} // Assuming you have icons for Google
                        disabled={false}
                        button_type={"button".to_string()}
                        icon_before={true}
                    />
                    <BasicButton
                        button_text={"Sign in with GitHub".to_string()}
                        style_ext={"bg-gray-700 hover:bg-gray-600 transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full ml-2".to_string()}
                        onclick={Callback::from(|_| {
                            // Handle GitHub Sign-In
                            gloo::console::log!("GitHub Sign-In clicked");
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
                <form {onsubmit} class="w-full max-w-md">
                    <div class="mb-6">
                        <InputField
                            initial_value={login_form.username.clone()}
                            label={"Email/Username".to_string()}
                            field_type={InputFieldType::Text}
                            name={"username".to_string()}
                            required={true}
                            placeholder={"Enter your email or username"}
                            oninput={&oninput}

                            ext_wrapper_styles={"mb-4".to_string()}
                            ext_label_styles={"block text-gray-700 text-sm font-bold mb-2".to_string()}
                            ext_input_styles={"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline".to_string()}
                            autocomplete={"on".to_string()}
                        />
                    </div>
                    <div class="mb-6">
                        <InputField
                            initial_value={login_form.password.clone()}
                            label={"Password".to_string()}
                            field_type={InputFieldType::Password}
                            name={"password".to_string()}
                            required={true}
                            placeholder={"Enter your password"}
                            oninput={&oninput}

                            ext_wrapper_styles={"mb-4".to_string()}
                            ext_label_styles={"block text-gray-700 text-sm font-bold mb-2".to_string()}
                            ext_input_styles={"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline".to_string()}
                            autocomplete={"on".to_string()}
                        />
                    </div>
                    <div class="flex items-center justify-between mb-6">
                        <a class="text-sm text-blue-500 hover:text-blue-700" href="#">{ "Forgot Password?" }</a>
                    </div>
                    <BasicButton
                        button_text={"Sign In".to_string()}
                        style_ext={"bg-primary text-white px-4 py-2 text-sm hover:bg-secondary transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full".to_string()}
                        icon={None}
                        disabled={!*login_form_is_valid}
                        button_type={"submit".to_string()}
                        icon_before={true} // if you have an icon before the button text, set it to true
                    />
                    <div class="flex items-center justify-center mt-6 text-sm text-blue-500 hover:text-blue-400">
                        <Link<Route> to={Route::SignUp}>
                            {"Don't have an account? Sign up"}
                        </Link<Route>>
                    </div>
                </form>

            </div>
            <div class="w-full md:w-1/2 hidden md:flex items-center justify-center bg-cover bg-center bg-[url('/img/signin_background.svg')]">
                <div class="text-center p-8">
                    // <h2 class="text-4xl text-white font-bold mb-4">{ "Nice to see you again" }</h2>
                    // <p class="text-white text-lg">{ "Welcome back! Please login to your account." }</p>
                </div>
            </div>
        </div>
    }
}
