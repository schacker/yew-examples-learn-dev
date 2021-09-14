use crate::random;
use std::rc::Rc;
use yew::{html, Component, Context, Html, Properties};

use fake::faker::address::raw::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListInfo {
    pub id: usize,
    pub name: Rc<str>,
    pub address: Rc<str>,
    pub age: usize,
}
impl ListInfo {
    pub fn new_random(id: usize) -> Self {
        let address = {
            let no = random::range_exclusive(1, 300);
            let state = StateAbbr(EN).fake::<String>();
            let city = CityName(EN).fake::<String>();
            let street = StreetName(EN).fake::<String>();

            Rc::from(format!("{} {} St., {}, {}", no, street, city, state).as_str())
        };

        Self {
            id,
            name: Rc::from(Name(EN).fake::<String>().as_str()),
            age: random::range_exclusive(7, 77),
            address,
        }
    }

    fn render(&self) -> Html {
        html! {
            <div class="card w-50 card_style">
                <div class="card-body">
                    <h5 class="card-title">{ format!("{} - {}", &self.id, &self.name) }</h5>
                    <p class="card-text">{ format!("Age: {}", &self.age) }</p>
                    <p class="card-text">{ format!("Address: {}", &self.address) }</p>
                </div>
            </div>
        }
    }
}

#[derive(Debug, Eq, PartialEq, Properties)]
pub struct ListProps {
    info: ListInfo,
}

pub struct ListComponent;

impl Component for ListComponent {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="text-info" id={ctx.props().info.id.to_string()}>
                { ctx.props().info.render() }
            </div>
        }
    }
}

pub enum ListType {
    Inline(ListInfo),
    Component(ListInfo),
}
impl ListType {
    pub fn info(&self) -> &ListInfo {
        match self {
            Self::Inline(info) => info,
            Self::Component(info) => info,
        }
    }

    pub fn new_random(id: usize, ratio: f64) -> Self {
        let info = ListInfo::new_random(id);
        if random::chance(ratio) {
            Self::Inline(info)
        } else {
            Self::Component(info)
        }
    }

    pub fn render(&self, keyed: bool) -> Html {
        match self {
            Self::Inline(info) => {
                if keyed {
                    html! {
                        <div key={info.id.to_string()} class="text-danger" id={info.id.to_string()}>
                            { info.render() }
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-danger" id={info.id.to_string()}>
                            { info.render() }
                        </div>
                    }
                }
            }
            Self::Component(info) => {
                if keyed {
                    html! { <ListComponent key={info.id.to_string()} info={info.clone()} /> }
                } else {
                    html! { <ListComponent info={info.clone()} /> }
                }
            }
        }
    }
}
