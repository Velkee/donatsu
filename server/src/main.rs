// This file is part of Donatsu.
//
// Donatsu is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Donatsu is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Donatsu. If not, see <https://www.gnu.org/licenses/>.

use std::fs;

use axum::{http::Method, response::Html, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/icon", get(icon)).layer(
        CorsLayer::new()
            .allow_methods(Method::GET)
            .allow_origin(Any),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn icon() -> Html<String> {
    Html(fs::read_to_string("../images/favicon.svg").unwrap())
}
