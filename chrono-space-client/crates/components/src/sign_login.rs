use leptos::prelude::*;

#[component]
pub fn SignLogin() -> impl IntoView {
    let (params, params_set) = signal("Login");
    view! {
        <div class="ring">
            <i style="--clr:#00ff0a;"></i>
            <i style="--clr:#ff0057;"></i>
            <i style="--clr:#fffd44;"></i>
            <div class="login">
                <Show
                    when=move || { params.get() == "Forget Password" }
                    fallback=move || view! {
                        <Show
                            when=move || { params.get() == "Login" }
                            fallback=move || view! { <Signup params_set=params_set.clone()/> }
                        >
                            <Login params_set=params_set.clone()/>
                        </Show>
                    }
                >
                    <ForgetPassword params_set=params_set.clone()/>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn Login(params_set: WriteSignal<&'static str>) -> impl IntoView {
    view! {
        <h2>Login</h2>
        <div class="inputBx">
            <input type="email" placeholder="Email"/>
        </div>
        <div class="inputBx">
            <input type="password" placeholder="Password"/>
        </div>
        <div class="inputBx">
            <input type="submit" value="Sign in"/>
        </div>
        <div class="links">
            <a on:click=move |_| {params_set.set("Forget Password")}>Forget Password</a>
            <a on:click=move |_| {params_set.set("Signup")}>Signup</a>
        </div>
    }
}

#[component]
pub fn Signup(params_set: WriteSignal<&'static str>) -> impl IntoView {
    view! {
        <h2>Signup</h2>
        <div class="inputBx">
            <input type="email" placeholder="Email"/>
        </div>
        <div class="inputBx">
            <input type="password" placeholder="Password"/>
        </div>
        <div class="inputBx">
            <input type="submit" value="Signup"/>
        </div>
        <div class="links">
            <a on:click=move |_| {params_set.set("Forget Password")}>Forget Password</a>
            <a on:click=move |_| {params_set.set("Login")}>Login</a>
        </div>
    }
}

#[component]
pub fn ForgetPassword(params_set: WriteSignal<&'static str>) -> impl IntoView {
    let (reset, reset_set) = create_signal("SendCode");

    view! {
        <h2>ResetPassword</h2>
        <div class="inputBx">
            <input type="email" placeholder="Email"/>
        </div>
        <Show
            when=move || { reset.get() == "SendCode" }
            fallback=move || view! {
                <div class="inputBx">
                    <input type="text" placeholder="VerificationCode"/>
                </div>
                <div class="inputBx">
                    <input type="password" placeholder="NewPassword"/>
                </div>
                <div class="inputBx">
                    <input type="submit" value="ResetPassword"/>
                </div>
            }
        >
            <div class="inputBx">
                <input type="submit" value="SendCode" on:click=move |_| {reset_set.set("ResetPassword")}/>
            </div>
        </Show>
        <div class="links">
            <a on:click=move |_| {params_set.set("Login")}>Login</a>
            <a on:click=move |_| {params_set.set("Signup")}>Signup</a>
        </div>
    }
}
