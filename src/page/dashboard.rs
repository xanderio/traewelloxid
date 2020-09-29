use anyhow::Result;
use yew::{prelude::*, services::Task};

use crate::{
    service::DashboardService,
    types::{StatusPaginate, View},
};

pub struct Dashboard {
    statuses: Option<StatusPaginate>,
    _task: Option<Box<dyn Task>>,
}

pub enum Msg {
    Loaded(Result<StatusPaginate>),
}

impl Component for Dashboard {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|value| Msg::Loaded(value));
        let task = DashboardService::statuses(View::Personal, 1, callback);

        Self {
            statuses: None,
            _task: Some(Box::new(task)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded(value) => {
                log::info!("{:?}", value);
                self.statuses = value.ok();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {"Dashboard"}
    }
}
