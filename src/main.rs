use yew::prelude::*;

#[function_component]
fn App() -> Html {
    // let counter = use_state(|| 0);
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter + 1;
    //         counter.set(value);
    //     }
    // };

    let name = use_state(|| "Jeff");
    let onclick = {
        let name = name.clone();
        move |_| {
            match *name {
                "Jeff" => {
                    name.set("Rose")
                },
                _ => {
                    name.set("Jeff")
                }
            }
        }
    };

    html! {
        <div>
            // <button {onclick}>{ "+1" }</button>
            // <p>{ *counter }</p>
            <h3>{ *name }</h3>
            <button {onclick}> { "toggle name" } </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}