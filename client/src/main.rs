// This file is part of Donatsu.
//
// Donatsu is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Donatsu is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Donatsu. If not, see <https://www.gnu.org/licenses/>.

use donatsu_client::nav::*;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <p>{ "Home" }</p> },
        Route::Add => html! {
            <form action="http://127.0.0.1:3000/add" method="get">
                <input name="search" />
                <input type="submit" hidden=true />
            </form>
        },
        Route::NotFount => html! { <img src="https://http.cat/404.jpg" /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let navs = vec![
        Nav {
            lable: "Media".to_string(),
            router_link: Route::Home,
        },
        Nav {
            lable: "Add New".to_string(),
            router_link: Route::Add,
        },
    ];

    let icon = use_state(String::new);

    {
        let icon = icon.clone();
        use_effect_with((), move |_| {
            let icon = icon.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_icon = Request::get("http://127.0.0.1:3000/icon")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                icon.set(fetched_icon);
            });
        })
    }

    let icon = Html::from_html_unchecked((*icon).clone().into());

    html! {
        <BrowserRouter>
            <header>
                <h2 display="inline">{icon}{"Donatsu"}</h2>
            </header>
            <nav class="sidebar">
                <NavBar navs={navs} />
            </nav>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
