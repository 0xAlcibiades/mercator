use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state_eq(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    // This draws a div with a button and then a reference to teh counter
    html! {
        <div>
            // This calls the onclick function
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}