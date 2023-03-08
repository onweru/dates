use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
struct Link {
    label: String,
    url: String,
    icon: String,
}

#[derive(Properties, PartialEq)]
struct LinksListProps {
    links: Vec<Link>,
    on_click: Callback<Link>,
}

#[derive(Properties, PartialEq)]
struct LinksDetailsProps {
    link: Link,
}

#[function_component(LinkDetails)]
fn link_details(LinksDetailsProps { link }: &LinksDetailsProps) -> Html {
    html! {
        <a href={ link.url.clone() } class="sidebar_link">
          <span class="na">{ link.label.clone() }</span>
        </a>
    }
}

#[function_component(LinksList)]
fn links_list(LinksListProps { links, on_click }: &LinksListProps) -> Html {
    let on_click = on_click.clone();
    links.iter().map(|link| {
        let on_link_select = {
            let on_click = on_click.clone();
            let link = link.clone();
            Callback::from(move |_| {
                on_click.emit(link.clone())
            })
        };

        html! {
            <div>
                <a href={ link.url.clone() } class="sidebar_link" onclick={on_link_select}>
                    <svg class="icon sidebar_icon">
                        <use href={ format!("sprites.svg#{}", link.icon) }></use>
                    </svg>
                    <span class="sidebar_label">{ link.label.clone() }</span>
                </a>
            </div>
        }
    }).collect()
}

#[function_component]
fn App() -> Html {

    let selected_link = use_state(|| None);

    let on_link_select = {
        let selected_link = selected_link.clone();
        Callback::from(move | link: Link| {
            selected_link.set(Some(link))
        })
    };

    let details = selected_link.as_ref().map(|link| html! {
        <LinkDetails link={link.clone()} />
    });

    let links = use_state(|| vec![]);
    {
        let links = links.clone();
        use_effect_with_deps(move |_| {
            let links = links.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_links: Vec<Link> = Request::get("/onweru/yew/main/src/nav.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                links.set(fetched_links);
            });
            || ()
        }, ());
    }

    html! {
        <div class="sidebar">
            <div>
                <LinksList links={(*links).clone()} on_click={on_link_select.clone()} />
            </div>
            { for details }
            <img src="sprites.svg" alt="sprites" class="hidden" />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
