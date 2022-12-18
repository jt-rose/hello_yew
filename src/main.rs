use yew::prelude::*;
mod components;

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

    let titles = vec!["Winter Castle".to_string(), "Summer Grove".to_string(), "Autumn Cabin".to_string()];

    html! {
        <div>
            <components::header::Header />
            // <button {onclick}>{ "+1" }</button>
            // <p>{ *counter }</p>
            <h2 class="bg-blue-100 text-center">{ *name }</h2>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full text-center" {onclick}> { "toggle name" } </button>
            <div class="flex flex-col justify-center">
            {titles.into_iter().map(|t| { html! { <components::card::Card title={t}/> }}).collect::<Html>()}
            </div>


// 



        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}