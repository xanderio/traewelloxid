use yew::prelude::*;
use yew_router::prelude::*;

use crate::agent::login::{self, LoginAgent};
use crate::app::AppRouter;

pub struct Navbar {
    link: ComponentLink<Self>,
    logged_in: bool,
    _login_agent: Box<dyn Bridge<LoginAgent>>,
}

pub enum Msg {
    Login(login::Responce),
}

impl Component for Navbar {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|responce| Msg::Login(responce));
        let login_agent = LoginAgent::bridge(callback);
        Self {
            link,
            logged_in: false,
            _login_agent: login_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login(login::Responce::LoggedIn(_)) => self.logged_in = true,
            Msg::Login(login::Responce::LoggedOut) => self.logged_in = false,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar navbar-expand-md navbar-dark bg-trwl">
                <div class="container">
                    <RouterAnchor<AppRouter> route=AppRouter::Home, classes="navbar-brand">
                        { "Träwelloixd" }
                    </RouterAnchor<AppRouter>>
                    // left side of the navbar
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav mr-auto">
                            {
                                if self.logged_in {
                                    html! {
                                        <li class="nav-item">
                                            <a class="nav-link" >{"Dashboard"}</a>
                                        </li>
                                    }
                                } else {
                                    html!{}
                                }
                            }
                            <li class="nav-item" >
                                <a class="nav-link">{"Top-Träwellers"}</a>
                            </li>
                            <li class="nav-item" >
                                <a class="nav-link">{"Unterwegs"}</a>
                            </li>
                        </ul>
                        <ul class="navbar-nav ml-auto">
                            {
                                if self.logged_in {
                                    html! {}
                                } else {
                                    html! {
                                        <li class="nav-item">
                                            <RouterAnchor<AppRouter> route=AppRouter::Login, classes="nav-link">
                                                {"Login"}
                                            </RouterAnchor<AppRouter>>
                                        </li>
                                    }
                                }
                            }
                        </ul>
                    </div>
                </div>
            </nav>
        }
    }
}
