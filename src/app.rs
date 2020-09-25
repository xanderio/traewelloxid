use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

use crate::agent::config::{self, ConfigAgent};
use crate::components::navbar::Navbar;
use crate::page::{Dashboard, Home, Login};

pub struct App {
    loading: bool,
    _config_agent: Box<dyn Bridge<ConfigAgent>>,
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

pub enum Msg {
    Config(config::Response),
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|x| Msg::Config(x));
        let config_agent = ConfigAgent::bridge(callback);
        log::debug!("App created");
        Self {
            loading: true,
            _config_agent: config_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Config(config::Response::Changed) => self.loading = false,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        if self.loading {
            html! { "loading ..." }
        } else {
            html! {
                <div class="app">
                    <Navbar/>
                    <main class="py-4">
                        <Router<AppRouter, ()>
                            render = Router::render(|switch: AppRouter| {
                                match switch {
                                    AppRouter::Login => html!{<Login/>},
                                    AppRouter::Home => html!{<Home/>},
                                    AppRouter::Dashboard => html!{<Dashboard/>},
                                    AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                    AppRouter::PageNotFound(Permissive(Some(missing_route))) => html!{format!("Page '{}' not found", missing_route)},
                                }
                            })
                            redirect = Router::redirect(|route: Route<()>| {
                                AppRouter::PageNotFound(Permissive(Some(route.route)))
                            })
                        />
                    </main>
               </div>
            }
        }
    }
}
