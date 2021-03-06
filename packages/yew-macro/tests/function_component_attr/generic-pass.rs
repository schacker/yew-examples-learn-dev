#![no_implicit_prelude]

#[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
struct Props {
    a: usize,
}

#[::yew::function_component(Comp)]
fn comp<P>(_props: &P) -> ::yew::Html
where
    P: ::yew::Properties + ::std::cmp::PartialEq,
{
    ::yew::html! {
        <p></p>
    }
}

#[::yew::function_component(Comp1)]
fn comp1<T1, T2>(_props: &()) -> ::yew::Html {
    ::yew::html! {
        <p></p>
    }
}

#[::yew::function_component(ConstGenerics)]
fn const_generics<const N: i32>() -> ::yew::Html {
    ::yew::html! {
        <div>
            { N }
        </div>
    }
}

fn compile_pass() {
    ::yew::html! { <Comp<Props> a=10 /> };
    ::yew::html! { <Comp1<usize, usize> /> };

    ::yew::html! { <ConstGenerics<10> /> };
}

fn main() {}
