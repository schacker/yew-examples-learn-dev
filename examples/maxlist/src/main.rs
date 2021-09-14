use instant::Instant;
use list::ListType;
use yew::html::Scope;
use yew::prelude::*;
use yew::web_sys::{HtmlElement, HtmlInputElement};

mod list;
mod random;

pub enum Msg {
    CreatePersons(usize),
    CreatePersonsPrepend(usize),
    ChangeRatio(f64),
    DeletePersonById(usize),
    DeleteEverybody,
    SwapRandom,
    ReverseList,
    SortById,
    SortByName,
    SortByAge,
    SortByAddress,
    ToggleKeyed,
    Rendered(Instant),
}

pub struct Model {
    lists: Vec<ListType>,
    last_id: usize,
    keyed: bool,
    build_component_ratio: f64,
    delta_ref: NodeRef,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut last_id = 0;
        let mut lists = Vec::new();
        for _ in 0..1000 {
          last_id += 1;
          lists.push(ListType::new_random(
              last_id,
              0.5,
          ));
        }
        Self {
            lists: lists,
            last_id: 1000,
            keyed: true,
            build_component_ratio: 0.5,
            delta_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreatePersons(n) => {
                for _ in 0..n {
                    self.last_id += 1;
                    self.lists.push(ListType::new_random(
                        self.last_id,
                        self.build_component_ratio,
                    ));
                }
                true
            }
            Msg::CreatePersonsPrepend(n) => {
                for _ in 0..n {
                    self.last_id += 1;
                    self.lists.insert(
                        0,
                        ListType::new_random(self.last_id, self.build_component_ratio),
                    );
                }
                true
            }
            Msg::ChangeRatio(ratio) => {
                #[allow(clippy::float_cmp)] // it's fine here?
                if self.build_component_ratio != ratio {
                    self.build_component_ratio = ratio;
                    log::info!("Ratio changed: {}", ratio);
                    true
                } else {
                    false
                }
            }
            Msg::DeletePersonById(id) => {
                if let Some(idx) = self.lists.iter().position(|p| p.info().id == id) {
                    self.lists.remove(idx);
                    true
                } else {
                    false
                }
            }
            Msg::DeleteEverybody => {
                self.lists.clear();
                true
            }
            Msg::SwapRandom => {
                let (a, b) = random::choose_two_distinct_mut(&mut self.lists).unwrap();
                log::info!("Swapping {} and {}.", a.info().id, b.info().id);
                std::mem::swap(a, b);
                true
            }
            Msg::ReverseList => {
                self.lists.reverse();
                true
            }
            Msg::SortById => {
                self.lists
                    .sort_unstable_by(|a, b| a.info().id.cmp(&b.info().id));
                true
            }
            Msg::SortByName => {
                self.lists
                    .sort_unstable_by(|a, b| a.info().name.cmp(&b.info().name));
                true
            }
            Msg::SortByAge => {
                self.lists.sort_by_key(|p| p.info().age);
                true
            }
            Msg::SortByAddress => {
                self.lists
                    .sort_unstable_by(|a, b| a.info().address.cmp(&b.info().address));
                true
            }
            Msg::ToggleKeyed => {
                self.keyed = !self.keyed;
                true
            }
            Msg::Rendered(time_before) => {
                let time_after = Instant::now();
                let elapsed_max = time_after - time_before;
                log::info!("Rendering started {} ms ago.", elapsed_max.as_millis());
                if let Some(input) = self.delta_ref.cast::<HtmlElement>() {
                    let delta_text =
                        format!("The last rendering took {} ms", elapsed_max.as_millis());
                    input.set_inner_text(&delta_text);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // 这里发送消息，但后续才是渲染DOM，那send_message是个异步的？
        ctx.link().send_message(Msg::Rendered(Instant::now()));

        html! {
            <div class="container">
                <div class="row">
                    <p class="h2" ref={self.delta_ref.clone()}/>
                    <hr />
                </div>
                // { self.action_view(ctx.link()) }
                { self.info_view() }
            </div>
        }
    }
}

impl Model {
    fn action_view(&self, link: &Scope<Self>) -> Html {
        html! {
            <>
                { self.button_view(link) }
                <div class="row">
                    <div class="col">
                        <p class="h5">
                            { "Person type ratio (0=only tags <= ratio <= 1=only components): " }
                            { self.build_component_ratio }
                        </p>
                        <input name="ratio" type="range" class="form-control-range" min="0.0" max="1.0" step="any"
                            oninput={link.callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::ChangeRatio(input.value_as_number())
                            })}
                        />
                    </div>
                </div>
            </>
        }
    }
    fn button_view(&self, link: &Scope<Self>) -> Html {
        html! {
            <>
                <div class="row">
                    <div class="col">
                        <button class="btn_size alert alert-danger" onclick={link.callback(|_| Msg::DeleteEverybody)}>
                            { "Delete everybody" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick={link.callback(|_| Msg::CreatePersons(1))}>
                            { "Create 1" }
                    </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick={link.callback(|_| Msg::CreatePersons(5))}>
                            { "Create 5" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick={link.callback(|_| Msg::CreatePersons(100))}>
                            { "Create 100" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick={link.callback(|_| Msg::CreatePersons(500))}>
                            { "Create 500" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick={link.callback(|_| Msg::CreatePersonsPrepend(1))}>
                            { "Prepend 1" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick={link.callback(|_| Msg::CreatePersonsPrepend(5))}>
                            { "Prepend 5" }
                        </button>
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <button class="btn_size alert alert-warning" onclick={link.callback(|_| Msg::ToggleKeyed)}>
                            { if self.keyed { "Disable keys" } else { "Enable keys" } }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick={link.callback(|_| Msg::SwapRandom)}>
                            { "Swap random" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick={link.callback(|_| Msg::ReverseList)}>
                            { "Reverse list" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick={link.callback(|_| Msg::SortById)}>
                            { "Sort by id" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick={link.callback(|_| Msg::SortByName)}>
                            { "Sort by name" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick={link.callback(|_| Msg::SortByAge)}>
                            { "Sort by age" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick={link.callback(|_| Msg::SortByAddress)}>
                            { "Sort by address" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
    fn info_view(&self) -> Html {
        let ids = if self.lists.len() < 20 {
            self.lists
                .iter()
                .map(|p| p.info().id.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            String::from("<too many>")
        };
        html! {
            <div>
                <p class="h5">{ "Number of persons: " }{ self.lists.len() }</p>
                <p class="h5">{ "Ids: " }{ ids }</p>
                <hr />
                <div class="persons">
                    { for self.lists.iter().map(|p| p.render(self.keyed)) }
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}