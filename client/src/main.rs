use gloo::{
    console::log,
    timers::callback::{Interval, Timeout},
};

use web_sys::HtmlButtonElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct YourLanguage {
    value: String,
}

#[function_component]
fn CatButton() -> Html {
    let win = web_sys::window().expect("win");
    let doc = win.document().expect("doc");
    let bod = doc.body().expect("bod");
    let storage = win
        .local_storage()
        .expect("Having a Local Storage")
        .unwrap();
    // This is the cause of the Waiting times ):
    let timeout = Timeout::new(400, move || {
        // Get the Class image element
        let cat_bod = bod
            .get_elements_by_class_name("cat_image")
            .get_with_index(0)
            .expect("Cat Image");

        // Get the fetched data and make sure it's sanitized
        let cat_hole = bod
            .get_elements_by_class_name("hole")
            .get_with_index(0)
            .expect("Cat Image url");
        let test = cat_hole.inner_html().to_string();

        // Then set the attribute
        cat_bod.set_attribute("src", test.as_str());
        // gloo::console::log!(cat_bod);
    })
    .forget();

    let api_state = use_state(|| "".to_owned());

    let onclick = Callback::from(move |e: MouseEvent| {
        let store = win
            .local_storage()
            .expect("Having a Local Storage")
            .unwrap();

        let api_state = api_state.clone();
        let value = store.get_item("lang").unwrap().unwrap();

        let api = format!("/api/catfacts/{}", value);
        log!(format!("/api/catfacts/{}", value));

        api_state.set(api);
    });

    let timeout = Timeout::new(1_000, move || {
        let bod = doc.body().expect("bod");
        // Get the Class image element
        let cat_bod = bod
            .get_elements_by_class_name("kitten_me")
            .get_with_index(0)
            .expect("Cat Image");

        // Create new button
        // log!(cat_bod);
        // gloo::console::log!(cat_bod);
    })
    .forget();

    let value = storage.get_item("lang").unwrap().unwrap();
    let api = format!("/api/catfacts/{}", value);
    log!(format!("/api/catfacts/{}", value));

    html! {
        <button onclick={onclick} value={value} class="btnsubmit kitten_me" hx-put={api} hx-target="#cat_fact" hx-swap="innerHTML" hx-trigger="click">
            {"Kitten me!"}
        </button>
    }
}

#[function_component]
fn CatLanguage() -> Html {
    let win = web_sys::window().expect("win");
    let storage = win
        .local_storage()
        .expect("Having a Local Storage")
        .unwrap();

    // Change to Onclick
    let onclick = Callback::from(move |e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlButtonElement>() {
            // sends to lcoal storage here
            storage.set_item("lang", target.value().as_str());
        }
    });

    html! {
        <section hx-boost="true" class="" >
        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/eng-us" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="eng-us">{"English"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/ces-cz" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="ces-cz">{"Czech"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/ger-de" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="ger-de">{"German"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/ben-in" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="ben-in">{"Bengali"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/esp-es" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="esp-es">{"Spain Spanish"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/esp-mx" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="esp-mx">{"Mexican Spanish"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/rus-ru" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="rus-ru">{"Russian"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/por-br" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="por-br">{"Portuguese"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/fil-tl" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="fil-tl">{"Filipino"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/ukr-ua" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="ukr-ua">{"Ukranian"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/urd-ud" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="urd-ud">{"Urdu"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/ita-it" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="ita-it">{"Italian"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/zho-tw" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="zho-tw">{"Chinese"}</button></div>

        <div><button hx-target="#cat_fact" hx-get="/api/catfacts/kor-ko" id="cat_languagee" class="orphan123" onclick={onclick.clone()} value="kor-ko">{"Korean"}</button></div>
        </section>
    }
}

#[function_component]
fn CatCard() -> Html {
    html! {
        <p id="cat_fact">{"Got fact yet?"}</p>
    }
}

#[function_component]
fn CatLink() -> Html {
    html! {
        <a target="_blank" rel="noreferrer noopener" href="https://github.com/ethanAthompson">{"My Github"}</a>
    }
}

#[function_component]
fn CatController() -> Html {
    let win = web_sys::window().expect("win");
    let doc = win.document().expect("doc");
    let config_open = use_state(|| false);
    let onclick = {
        let config_open = config_open.clone();

        Callback::from(move |_| match *config_open {
            false => {
                config_open.set(true);

                // gloo::console::log!(*config_open, "Show");
                doc.get_element_by_id("config1")
                    .expect("Cat Config Card")
                    .set_class_name("card");
            }
            true => {
                config_open.set(false);

                // gloo::console::log!(*config_open, "Hide");
                doc.get_element_by_id("config1")
                    .expect("Cat Config Card")
                    .set_class_name("cardhidden");
            }
        })
    };

    html! {
        <>
            if *config_open == true {
                <button onclick={onclick} class="">{"Close Config?"}</button>
            } else {
                <button onclick={onclick} class="">{"Open Config?"}</button>
            }
        </>
    }
}

#[function_component]
fn HtmxCatNavbar() -> Html {
    html! {
        <nav class="nav">
            <img class="logo" src="https://cdn-icons-png.flaticon.com/512/616/616430.png"/>
            <h1 class="title"> {"Welcome to Htmx Cat Facts"} </h1>
        </nav>
    }
}

#[function_component]
fn HtmxCatFooter() -> Html {
    html! {
        <>
            <span class="gap"></span>

            <footer class="footer">
                <section class="sect">
                    {"Built with Htmx + Yew + Axum + Tailwind CSS"}
                </section>
            </footer>
        </>
    }
}

#[function_component]
fn HtmxCatC1() -> Html {
    html! {
        <main class="card">
            <img class="cat_image" id="cat_image" src="https://placehold.co/600x400"/>
            <article class="cardextra">
                <div class="title">{"The Cat Fact"}</div>
                <p class="hole hidden" id="test">{"https://cdn2.thecatapi.com/images/b7t.jpg"}</p>
                <p class="cardtext"><CatCard/></p>
            </article>

            <button class="hidden" hx-get="/api/catimages" hx-target="#test" hx-trigger="load, delay:0s">{"Test"}</button>
            // if cat fact is shown, then create a copy link to share on outloook or gmail;
            <div class="store">
                <span class="carditem "><CatButton/></span>
                <span class="carditem "><CatLink/></span>
                <span class="carditem "><CatController/></span>
            </div>
        </main>
    }
}

#[function_component]
fn HtmxCatC2() -> Html {
    html! {
        <main class="cardhidden" id="config1">
            <article class="cardextra">
                <div class="title">{"Cat Conifg"}</div>
                <p class="cardtext">{"Choose a Language For Next Time"}</p>
            </article>
            <div class="store">
                <CatLanguage/>
            </div>
        </main>
    }
}
#[function_component]
fn HtmxCatMain() -> Html {
    html! {
        <div class="centerdiv2 gap-x-4">
            <HtmxCatC1/>
            <HtmxCatC2/>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {

        // Context for data sharing aka pain sharing
        <>
            <script data-trunk="true" src="https://unpkg.com/htmx.org@1.9.4"></script>
            <HtmxCatNavbar/>
            <HtmxCatMain/>
            <HtmxCatFooter/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
