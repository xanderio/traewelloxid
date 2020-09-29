use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

use crate::agent::{config, login, ConfigAgent, LoginAgent};

use crate::components::navbar::Navbar;
use crate::page::{Dashboard, Home, Login};

pub struct App {
    config_loaded: bool,
    login_loaded: bool,
    logged_in: bool,
    _config_agent: Box<dyn Bridge<ConfigAgent>>,
    _login_agent: Box<dyn Bridge<LoginAgent>>,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/login"]
    Login,
    #[to = "/dashboard"]
    Dashboard,
    #[to = "/"]
    Home,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

#[derive(Debug)]
pub enum Msg {
    Config(config::Response),
    Login(login::Responce),
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::Config);
        let config_agent = ConfigAgent::bridge(callback);

        let callback = link.callback(Msg::Login);
        let login_agent = LoginAgent::bridge(callback);
        log::debug!("App created");
        Self {
            config_loaded: false,
            login_loaded: false,
            logged_in: LoginAgent::has_storad_token(),
            _config_agent: config_agent,
            _login_agent: login_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Config(config::Response::Changed) => {
                self.config_loaded = true;
                true
            }
            Msg::Login(login::Responce::LoggedIn(_)) => {
                self.login_loaded = true;
                self.logged_in = true;
                true
            }
            Msg::Login(login::Responce::LoggedOut) => {
                self.login_loaded = true;
                self.logged_in = false;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let logged_in = self.logged_in;
        match (self.config_loaded, self.login_loaded) {
            (true, true) => html! {
                <div class="app">
                    <Navbar/>
                    <main class="py-4">
                        <Router<AppRouter, ()>
                            render = Router::render(move |switch: AppRouter| {
                                if logged_in {
                                    match switch {
                                        AppRouter::Login => html!{<Login/>},
                                        AppRouter::Home => html!{<Home/>},
                                        AppRouter::Dashboard => html!{<Dashboard/>},
                                        AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                        AppRouter::PageNotFound(Permissive(Some(missing_route))) =>
                                            html!{format!("Page '{}' not found", missing_route)},
                                    }
                                } else {
                                    match switch {
                                        AppRouter::Home => html!{<Home/>},
                                        _ => html!{<Login/>},
                                    }
                                }
                            })
                            redirect = Router::redirect(|route: Route<()>| {
                                AppRouter::PageNotFound(Permissive(Some(route.route)))
                            })
                        />
                    </main>
               </div>
            },
            (_, _) => html! {"Loading"},
        }
    }
}
