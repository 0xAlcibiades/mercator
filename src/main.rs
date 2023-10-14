use yew::prelude::*;

#[function_component(Content)]
fn content() -> HtmlResult {
    let counter = use_state_eq(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    Ok(html! {
        <div>
            // This calls the onclick function
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    })
}

#[function_component]
fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}