use std::collections::HashSet;
use yew::{agent::*, services::fetch::*};

use crate::service::config::ConfigService;
use crate::Config;

pub struct ConfigAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    _task: FetchTask,
}

pub enum Msg {
    Fetch(Config),
}

pub enum Response {
    Changed,
}

impl Agent for ConfigAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = ();
    type Output = Response;
    fn create(link: AgentLink<Self>) -> Self {
        let task = ConfigService::load(
            link.callback(|config: Result<Config, _>| Msg::Fetch(config.expect("config none"))),
        );
        log::debug!("ConfigAgent created");

        Self {
            link,
            subscribers: HashSet::default(),
            _task: task,
        }
    }
    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Fetch(config) => {
                log::trace!("Fetched new config");

                ConfigService::put(config);

                for id in self.subscribers.iter() {
                    log::trace!("Sending config change to {:?}", id);
                    self.link.respond(*id, Response::Changed);
                }
            }
        }
    }

    fn handle_input(&mut self, _msg: Self::Input, _id: HandlerId) {}

    fn connected(&mut self, id: HandlerId) {
        log::trace!("new config client connect {:?}", id);
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        log::trace!("config client disconnect {:?}", id);
        self.subscribers.remove(&id);
    }
}
