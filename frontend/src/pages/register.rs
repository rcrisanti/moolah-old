use reqwest::Client;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use weblog::console_debug;
use yew::prelude::*;

use crate::components::{Footer, Header};
use shared::{models::UserForm, routes};

pub enum RegisterMsg {
    UsernameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    Submitted,
}

pub struct Register {
    username: String,
    email: String,
    password: String,
}

impl Component for Register {
    type Message = RegisterMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Register {
            username: String::new(),
            email: String::new(),
            password: String::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange_username = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| RegisterMsg::UsernameChanged(input.value()))
        });

        let onchange_email = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| RegisterMsg::EmailChanged(input.value()))
        });

        let onchange_password = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| RegisterMsg::PasswordChanged(input.value()))
        });

        let onsubmit = ctx.link().callback(|ev: FocusEvent| {
            ev.prevent_default();
            RegisterMsg::Submitted
        });

        html! {
            <>
                <Header heading="register" title="register" logged_in=false />

                <form {onsubmit}>
                // <form method="post" action={routes::REGISTER}>
                    <div>
                        <label for="username">{ "username:" }</label>
                        <input id="username" type="text" placeholder="username" onchange={onchange_username}/>
                    </div>
                    <div>
                        <label for="email">{ "email:" }</label>
                        <input type="email" placeholder="email" onchange={onchange_email}/>
                    </div>
                    <div>
                        <label for="password">{ "password:" }</label>
                        <input type="password" placeholder="password" onchange={onchange_password}/>
                    </div>
                    <input type="submit" value="register"/>
                </form>

                <Footer />
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RegisterMsg::UsernameChanged(username) => {
                self.username = username;
                console_debug!("changed username");
            }
            RegisterMsg::EmailChanged(email) => {
                self.email = email;
                console_debug!("changed email");
            }
            RegisterMsg::PasswordChanged(password) => {
                self.password = password;
                console_debug!("changed password");
            }
            RegisterMsg::Submitted => {
                console_debug!("submitting form");
                let user_form = UserForm {
                    username: self.username.clone(),
                    email: self.email.clone(),
                    password: self.password.clone(),
                };

                wasm_bindgen_futures::spawn_local(async move {
                    Client::new()
                        .post(format!(
                            "{}{}",
                            web_sys::window().expect("could not get window").origin(),
                            routes::REGISTER
                        ))
                        .json(&user_form)
                        .send()
                        .await
                        .expect("could not post user form");
                });

                console_debug!("successfully submitted form");
            }
        }
        true
    }
}
