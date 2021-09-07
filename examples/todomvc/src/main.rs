use gloo::storage::{LocalStorage, Storage};
use gloo::console as console;
use state::{Entry, Filter, State};
use strum::IntoEnumIterator;
use yew::html::Scope;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{classes, html, Component, Context, FocusEvent, Html, NodeRef, TargetCast};
use yew::{events::KeyboardEvent, Classes};

mod state;

const KEY: &str = "yew.todomvc.self";
/**
 * 消息枚举类型
 */
#[derive(Debug)]
pub enum Msg {
    Add(String),
    Edit((usize, String)),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
}
/**
 * 组件状态结构体，实际上是为给Model实现Component trait做准备
 */
pub struct Model {
    state: State,
    focus_ref: NodeRef,
}
/**
 * 给Model实现Component trait的实现
 * 所有组件生命周期方法都接收上下文对象。该对象提供了对组件作用域的引用，它允许向组件发送消息和传递给组件的 props。
 */
impl Component for Model {
    /**
     * Message 表示组件可以处理以触发某些副作用的各种消息。
     * 例如，你可能有一条 Click 消息，该消息触发 API 请求或者切换 UI 组件的外观。
     * 通常的做法是在组件模块中创建一个叫做 Msg 的枚举并将其用作组件中的消息类型。通常将“message”缩写为“msg”。
     */
    type Message = Msg;
    /**
     * Properties 表示从父级传递到组件的信息。
     * 此类型必须实现 Properties trait（通常通过派生），并且可以指定某些属性（properties）是必需的还是可选的。
     * 创建和更新组件时使用此类型。通常的做法是在组件模块中创建一个叫做 Props 的结构体并将其用作组件的 Properties 类型。
     * 通常将“properties”缩写为“props”。由于 props 是从父组件传递下来的，因此应用程序的根组件通常有一个类型为 () 的 Properties。
     * 如果你希望为根组件指定属性（properties），请使用 App::mount_with_props 方法。
     */
    type Properties = (); // 根组件初始为 ()
    /**
     * 新版生命周期，组件创建
     * 当一个组件被创建时，它会从其父组件以及一个 Context 接收属性（properties）。
     * 属性（properties）可用于初始化组件的状态，“link”可用于注册回调或向组件发送消息。
     */
    fn create(_ctx: &Context<Self>) -> Self {
        let entries = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        let state = State {
            entries,
            filter: Filter::All,
            edit_value: "".into(),
        };
        let focus_ref = NodeRef::default();
        Self { state, focus_ref }
    }
    /**
     * 新版生命周期，组件更新，
     * 组件是动态的，可以注册以接收异步信息。
     * update() 生命周期方法对于每个消息都会被调用。这使得组件可以根据消息的内容来更新自身，并决定是否需要重新渲染自己。
     * 消息可以由 HTML 元素监听器触发，或者由子组件，Agents，Services 或 Futures 发送。
     */
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        console::log!("update");
        match msg {
            Msg::Add(description) => {
                if !description.is_empty() {
                    let entry = Entry {
                        description: description.trim().to_string(),
                        completed: false,
                        editing: false,
                    };
                    self.state.entries.push(entry);
                }
            }
            Msg::Edit((idx, edit_value)) => {
                self.state.complete_edit(idx, edit_value.trim().to_string());
                self.state.edit_value = "".to_string();
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.clear_all_edit();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            }
        }
        LocalStorage::set(KEY, &self.state.entries).expect("failed to set");
        true
    }
    /**
     * 定义一个组件的结构，使用html!宏定义
     * 组件在 view() 方法中声明它的布局。Yew 提供了 html! 宏来声明 HTML 和 SVG 节点和它们的监听器及其子组件。
     * 这个宏的行为很像 React 中的 JSX，但是使用的是 Rust 表达式而不是 JavaScript
     */
    fn view(&self, ctx: &Context<Self>) -> Html {
        console::log!("view");
        let hidden_class = if self.state.entries.is_empty() {
            "hidden"
        } else {
            ""
        };
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        { self.view_input(ctx.link()) }
                    </header>
                    <section class={classes!("main", hidden_class)}>
                        <input
                            type="checkbox"
                            class="toggle-all"
                            id="toggle-all"
                            checked={self.state.is_all_completed()}
                            onclick={ctx.link().callback(|_| Msg::ToggleAll)}
                        />
                        <label for="toggle-all" />
                        <ul class="todo-list">
                            { 
                                for self.state.entries
                                    .iter()
                                    .filter(|e| self.state.filter.fits(e))
                                    .enumerate()
                                    .map(|e| self.view_entry(e, ctx.link()))
                            }
                        </ul>
                    </section>
                    <footer class={classes!("footer", hidden_class)}>
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { 
                                for Filter::iter().map(|flt| self.view_filter(flt, ctx.link()))
                            }
                        </ul>
                        <button class="clear-completed" onclick={ctx.link().callback(|_| Msg::ClearCompleted)}>
                            { format!("Clear completed ({})", self.state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
    /**
     * 在组件首次渲染完成调用
     * 组件生命周期方法调用是在 view() 被处理并且 Yew 已经把组件挂载到 DOM 上之后，浏览器刷新页面之前。
     * 组件通常希望实现此方法以执行只能在组件渲染元素之后才能执行的操作。如果你想在做出一些更改后重新渲染组件，返回 true 就可以了。
     */
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        console::log!("rendered", first_render);
        if first_render {
            if let Some(input) = self.focus_ref.cast::<InputElement>() {
                input.focus().unwrap();
            }
        }
    }
    /**
     * 组件可能被其父节点重新渲染。
     * 发生这种情况时，它们可以接收新的属性（properties）并选择重新渲染。
     * 这种设计通过更改属性（properties）来促进父子组件之间的通信。
     * 你不是必须实现 change()，但是如果想在组件被创建后通过 props 来更新组件，则可能要这么做。
     */ 
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        console::log!("changed");
        true
    }
    /**
     * 组件从 DOM 上被卸载后，Yew 调用 destroy() 生命周期方法来支持任何必要的清理操作。这个方法是可选的，默认情况下不执行任何操作。
     */
    fn destroy(&mut self, _ctx: &Context<Self>) {
        console::log!("destroy");
    }
}
/**
 * 实现Model组件自己的函数
 */
impl Model {
    fn view_filter(&self, filter: Filter, link: &Scope<Self>) -> Html {
        let cls = if self.state.filter == filter {
            "selected"
        } else {
            "not-selected"
        };
        html! {
            <li>
                <a class={cls}
                   href={filter.as_href()}
                   onclick={link.callback(move |_| Msg::SetFilter(filter))}
                >
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Add(value))
            } else {
                None
            }
        });
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry), link: &Scope<Self>) -> Html {
        let mut class = Classes::from("todo");
        if entry.editing {
            class.push(" editing");
        }
        if entry.completed {
            class.push(" completed");
        }
        html! {
            <li {class}>
                <div class="view">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked={entry.completed}
                        onclick={link.callback(move |_| Msg::Toggle(idx))}
                    />
                    <label ondblclick={link.callback(move |_| Msg::ToggleEdit(idx))}>{ &entry.description }</label>
                    <button class="destroy" onclick={link.callback(move |_| Msg::Remove(idx))} />
                </div>
                { self.view_entry_edit_input((idx, entry), link) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry), link: &Scope<Self>) -> Html {
        let edit = move |input: InputElement| {
            let value = input.value();
            input.set_value("");
            Msg::Edit((idx, value))
        };

        let onblur = link.callback(move |e: FocusEvent| edit(e.target_unchecked_into()));

        let onkeypress = link.batch_callback(move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| edit(e.target_unchecked_into()))
        });

        if entry.editing {
            html! {
                <input
                    class="edit"
                    type="text"
                    ref={self.focus_ref.clone()}
                    value={self.state.edit_value.clone()}
                    onmouseover={link.callback(|_| Msg::Focus)}
                    {onblur}
                    {onkeypress}
                />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
