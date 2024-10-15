// This file is part of Donatsu.
//
// Donatsu is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Donatsu is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Donatsu. If not, see <https://www.gnu.org/licenses/>.

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/add")]
    Add,
    #[not_found]
    #[at("/404")]
    NotFount,
}

#[derive(Clone, PartialEq)]
pub struct Nav {
    pub lable: String,
    pub router_link: Route,
}

#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    pub navs: Vec<Nav>,
}

#[function_component(NavBar)]
pub fn nav_bar(NavBarProps { navs }: &NavBarProps) -> Html {
    navs.iter()
        .map(|nav| {
            html! {
                <Link<Route> to={nav.router_link.clone()}>{nav.lable.clone()}</Link<Route>>
            }
        })
        .collect()
}
