use yew::prelude::*;

use std::str::FromStr;

use crate::agent::login::{self, LoginAgent};

pub struct Login {
    link: ComponentLink<Self>,
    login_agent: Box<dyn Bridge<LoginAgent>>,
    email: String,
    email_valid: bool,
    password: String,
    remember: bool,
}

pub enum Msg {
    Email(ChangeData),
    Password(ChangeData),
    Remenber(ChangeData),
    Submit,
    Login(login::Responce),
}

impl Component for Login {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|value| Msg::Login(value));
        let login_agent = LoginAgent::bridge(callback);
        Self {
            link,
            login_agent,
            email: "".into(),
            email_valid: true,
            password: "".into(),
            remember: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Email(ChangeData::Value(value)) => {
                self.email_valid = validator::validate_email(&value);
                self.email = value;
            }
            Msg::Email(_) => {}
            Msg::Password(ChangeData::Value(value)) => self.password = value,
            Msg::Password(_) => {}
            Msg::Remenber(ChangeData::Value(value)) => {
                self.remember = bool::from_str(&value).unwrap()
            }
            Msg::Remenber(_) => {}
            Msg::Submit => {
                self.login_agent.send(login::Request::Login {
                    email: self.email.clone(),
                    password: self.password.clone(),
                });
            }
            Msg::Login(login) => {
                log::info!("{:?}", login);
            }
        }
        log::info!("{} {}", &self.email, &self.password);
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
                <div class="container">
                    <div class="row justify-content-center">
                        <div class="col-md-8">
                            <div class="card">
                                <div class="card-header">{"Login"}</div>
                                <div class="card-body">
                                    <div class="form-group row">
                                        <label for="email" class="col-md-4 col-form-label text-md-right">{"E-Mail-Addresse"}</label>

                                        <div class="col-md-6">
                                            <input id="email", type="email",
                                                class={if self.email_valid { "form-control" } else { "form-control is-invalid" }},
                                                name="email",
                                                value={self.email.clone()},
                                                required=true,
                                                autocomplete="email",
                                                autofocus=true,
                                                onchange=self.link.callback(|value| Msg::Email(value)) />

                                                {
                                                    if self.email_valid {
                                                        html! {
                                                            <span class="invalid-feedback" role="alert">
                                                                <strong>{"Bitte gibt eine gültige Email Adresse ein."}</strong>
                                                            </span>
                                                        }
                                                    } else {
                                                        html!{}
                                                    }
                                                }
                                        </div>
                                    </div>

                                    <div class="form-group row">
                                        <label for="password", class="col-md-4 col-form-label text-md-right">{"Password"}</label>

                                        <div class="col-md-6">
                                            <input id="password", type="password",
                                                class="form-control",
                                                name="password",
                                                required=true,
                                                autocomplete="current-password",
                                                onchange=self.link.callback(|value| Msg::Password(value))/>

                                            // @error('password')
                                            //     <span class="invalid-feedback" role="alert">
                                            //         <strong>{{ $message }}</strong>
                                            //     </span>
                                            // @enderror
                                        </div>
                                    </div>

                                    <div class="form-group row">
                                        <div class="col-md-6 offset-md-4">
                                            <div class="form-check">
                                                <input type="checkbox", id="remember",
                                                    class="form-check-input",
                                                    name="remember",
                                                    value=self.remember,
                                                    onchange=self.link.callback(|value| Msg::Remenber(value))/>

                                                <label class="form-check-label" for="remember">
                                                    {"Angemeldet bleiben"}
                                                </label>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="form-group row mb-0">
                                        <div class="col-md-8 offset-md-4">
                                            <button class="btn btn-primary" onclick=self.link.callback(|_| Msg::Submit)>
                                                {"Anmelden"}
                                            </button>

                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
        }
    }
}
