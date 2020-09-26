use anyhow::Result;
use yew::agent::*;
use yew::services::{storage::*, Task};
use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    route::Route,
};

use std::collections::HashSet;

use crate::service::login::LoginService;

const STORAGE_KEY: &'static str = "token";

pub struct LoginAgent {
    link: AgentLink<Self>,
    bearer: Option<String>,
    subscriber: HashSet<HandlerId>,
    storage: StorageService,
    _task: Option<Box<dyn Task>>,
}

#[derive(Debug)]
pub enum Request {
    Login { email: String, password: String },
}

#[derive(Debug)]
pub enum Responce {
    LoggedIn(String),
    LoggedOut,
}

#[derive(Debug)]
pub enum Msg {
    GotBearer(String),
}

impl Agent for LoginAgent {
    type Reach = Context<Self>;

    type Message = Msg;

    type Input = Request;

    type Output = Responce;

    fn create(link: AgentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("failed to open local storage");
        let bearer = storage.restore::<Result<String>>(STORAGE_KEY).ok();
        Self {
            link,
            bearer,
            subscriber: HashSet::new(),
            storage,
            _task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::GotBearer(bearer) => {
                self.storage.store(STORAGE_KEY, Ok(bearer.clone()));
                self.bearer = Some(bearer);
                for id in self.subscriber.clone() {
                    self.link.respond(id, self.to_responce());
                }
                let mut routing = RouteAgentDispatcher::<()>::new();
                routing.send(RouteRequest::ChangeRoute(Route::new_no_state("/dashboard")));
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::Login { email, password } => {
                let callback = self
                    .link
                    .callback(|bearer: Result<String>| Msg::GotBearer(bearer.unwrap()));
                let task = LoginService::login(email, password, callback);
                self._task = Some(Box::new(task));
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscriber.insert(id);
        self.link.respond(id, self.to_responce())
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscriber.remove(&id);
    }
}

impl LoginAgent {
    fn to_responce(&self) -> Responce {
        match self.bearer.clone() {
            Some(bearer) => Responce::LoggedIn(bearer),
            None => Responce::LoggedOut,
        }
    }
}
