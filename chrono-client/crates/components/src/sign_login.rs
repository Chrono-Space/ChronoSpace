use data_types::friend::friend_list::{FriendListReq, FriendListRes};
use data_types::signup_login::{LoginReq, LoginRes, ResetPasswordReq, SignupReq};
use data_types::verification_code::SendVerificationCodeReq;
use leptos::html;
use leptos::html::tr;
use leptos::leptos_dom::log;
use leptos::*;
use storage::set_token;
use wasm_http::http_ctx::HttpCtx;

#[component]
pub fn SignLogin(router_set: WriteSignal<String>) -> impl IntoView {
    let (params, params_set) = create_signal("Login".to_string());
    view! {
        <div class="sign_login">
            <div class="ring">
                <i style="--clr:#00ff0a;"></i>
                <i style="--clr:#ff0057;"></i>
                <i style="--clr:#fffd44;"></i>
                <div class="login">
                    <Show
                        when=move || { params.get() == "Forget Password" }
                        fallback=move || {
                            view! {
                                <Show
                                    when=move || { params.get() == "Login" }
                                    fallback=move || {
                                        view! { <Signup params_set=params_set.clone()/> }
                                    }
                                >

                                    <Login
                                        params_set
                                        router_set
                                    />
                                </Show>
                            }
                        }
                    >

                        <ForgetPassword params_set=params_set.clone()/>
                    </Show>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Login(params_set: WriteSignal<String>, router_set: WriteSignal<String>) -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (login, set_login) = create_signal(false);
    create_resource(
        move || (login.get(), email.get(), password.get()),
        move |(value, email, password)| async move {
            log!("test");
            if value && ! email.is_empty() && !password.is_empty(){
                if let Ok(Some(data)) = HttpCtx::default()
                    .post::<LoginReq, LoginRes>(
                        "/api/login",
                        &LoginReq {
                            email,
                            password,
                        },
                    )
                    .await
                {
                    set_token(&data.token);
                    router_set.set("Home".to_string());
                } else {
                    router_set.set("Login".to_string());
                }
                log!("test1");
                set_login.set(false);
            }
        },
    );

    view! {
        <h2>Login</h2>
        <div class="inputBx">
            <input
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_email.set(val);
                }

                prop:value=email
                type="email"
                placeholder="Email"
            />
        </div>
        <div class="inputBx">
            <input
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_password.set(val);
                }

                prop:value=password
                type="password"
                placeholder="Password"
            />
        </div>
        <div class="inputBx">
            <input on:click=move |_| { set_login.set(true) } type="submit" value="Sign in"/>
        </div>
        <div class="links">
            <a on:click=move |_| {
                params_set.set("Forget Password".to_string())
            }>Forget Password</a>
            <a on:click=move |_| { params_set.set("Signup".to_string()) }>Signup</a>
        </div>
    }
}

#[component]
pub fn Signup(params_set: WriteSignal<String>) -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (code, set_code) = create_signal(String::new());
    let (reset, reset_set) = create_signal("SendCode");
    let input_element: NodeRef<html::Input> = create_node_ref();
    let (send_code, set_send_code) = create_signal(false);
    let (signup, set_signup) = create_signal(false);
    create_resource(
        move || send_code.get(),
        move |value| async move {
            if value {
                if let Err(e) = HttpCtx::default()
                    .post::<SendVerificationCodeReq, ()>(
                        "/api/verification_code/send",
                        &SendVerificationCodeReq { to: email.get() },
                    )
                    .await
                {
                    log!("{e}");
                } else {

                }
                reset_set.set("SetPassword");
                set_send_code.set(false);
            }
        },
    );
    create_resource(
        move || signup.get(),
        move |value| async move {
            if value {
                if let Err(e) = HttpCtx::default()
                    .post::<SignupReq, ()>(
                        "/api/signup",
                        &SignupReq {
                            email: email.get(),
                            code: code.get(),
                            password: password.get(),
                        },
                    )
                    .await
                {
                    log!("{e}");
                } else {
                    params_set.set("Login".to_string())
                }
                set_send_code.set(false);
            }
        },
    );
    view! {
        <h2>Signup</h2>
        <div class="inputBx">
            <input
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_email.set(val);
                }

                prop:value=email
                type="email"
                placeholder="Email"
            />
        </div>
        <Show
            when=move || { reset.get() == "SendCode" }
            fallback=move || {
                view! {
                    <div class="inputBx">
                        <input
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_code.set(val);
                            }

                            prop:value=code
                            type="text"
                            placeholder="VerificationCode"
                        />
                    </div>
                    <div class="inputBx">
                        <input
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_password.set(val);
                            }

                            prop:value=password
                            type="password"
                            placeholder="Password"
                        />
                    </div>
                    <div class="inputBx">
                        <input
                            on:click=move |_| { set_signup.set(true) }
                            type="submit"
                            value="Signup"
                        />
                    </div>
                }
            }
        >

            <div class="inputBx">
                <input
                    type="submit"
                    value="SendCode"
                    on:click=move |_| { set_send_code.set(true) }
                />
            </div>
        </Show>
        <div class="links">
            <a on:click=move |_| {
                params_set.set("Forget Password".to_string())
            }>Forget Password</a>
            <a on:click=move |_| { params_set.set("Login".to_string()) }>Login</a>
        </div>
    }
}

#[component]
pub fn ForgetPassword(params_set: WriteSignal<String>) -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (code, set_code) = create_signal(String::new());
    let (reset, reset_set) = create_signal("SendCode");
    let (send_code, set_send_code) = create_signal(false);
    let (reset_password, set_reset_password) = create_signal(false);
    create_resource(
        move || send_code.get(),
        move |value| async move {
            if value {
                if let Err(e) = HttpCtx::default()
                    .post::<SendVerificationCodeReq, ()>(
                        "/api/verification_code/send",
                        &SendVerificationCodeReq { to: email.get() },
                    )
                    .await
                {
                    log!("{e}");
                } else {
                    reset_set.set("ResetPassword");
                }
                set_send_code.set(false);
            }
        },
    );
    create_resource(
        move || reset_password.get(),
        move |value| async move {
            if value {
                if let Err(e) = HttpCtx::default()
                    .post::<ResetPasswordReq, ()>(
                        "/api/password/reset",
                        &ResetPasswordReq {
                            email: email.get(),
                            code: code.get(),
                            password: password.get(),
                        },
                    )
                    .await
                {
                    log!("{e}");
                } else {
                    params_set.set("Login".to_string());
                }
                set_reset_password.set(false);
            }
        },
    );
    view! {
        <h2>ResetPassword</h2>
        <div class="inputBx">
            <input
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_email.set(val);
                }

                prop:value=email
                type="email"
                placeholder="Email"
            />
        </div>
        <Show
            when=move || { reset.get() == "SendCode" }
            fallback=move || {
                view! {
                    <div class="inputBx">
                        <input
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_code.set(val);
                            }

                            prop:value=code
                            type="text"
                            placeholder="VerificationCode"
                        />
                    </div>
                    <div class="inputBx">
                        <input
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_password.set(val);
                            }

                            prop:value=password
                            type="password"
                            placeholder="NewPassword"
                        />
                    </div>
                    <div class="inputBx">
                        <input
                            type="submit"
                            value="ResetPassword"
                            on:click=move |_| { set_reset_password.set(true) }
                        />
                    </div>
                }
            }
        >

            <div class="inputBx">
                <input
                    type="submit"
                    value="SendCode"
                    on:click=move |_| { set_send_code.set(true) }
                />
            </div>
        </Show>
        <div class="links">
            <a on:click=move |_| { params_set.set("Login".to_string()) }>Login</a>
            <a on:click=move |_| { params_set.set("Signup".to_string()) }>Signup</a>
        </div>
    }
}
