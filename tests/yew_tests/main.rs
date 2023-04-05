use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 4;
            counter.set(value)
        }
    };

    html! {

        <div>
        <head>
                <title>{"HeptaFlip"}</title>
                <style>
                {" h1 {color:#d6a000;text-align:center;} "}
                {" h2 {color:#e48300;text-align:center;} "}
                {" p {color:#e48300;text-align:center;} "}
                {" body {color:yellow;background-color:#060020;} "}
                {" * {font-family: monospace;} "}
    </style>
        </head>

        <h1>{"Hello Everyone"}</h1>
            <button {onclick}>{ "+4"}</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
