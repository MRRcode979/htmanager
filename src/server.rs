// Replacement code for the custom Web Server
use std::str::FromStr;
use warp::{filters::BoxedFilter, http::Uri, path::FullPath, redirect, Filter, Reply};



fn main() {
serve(8080);
}

#[tokio::main]
async fn serve(port: u16) {
    let current_dir = std::env::current_dir().expect("failed to read current directory");

    let routes = root_redirect().or(warp::fs::dir(current_dir));

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

fn root_redirect() -> BoxedFilter<(impl Reply,)> {
    warp::path::full()
        .and_then(move |path: FullPath| async move {
            let path = path.as_str();

            if path.ends_with("/") || path.contains(".") {
                return Err(warp::reject());
            }

            Ok(redirect::redirect(
                Uri::from_str(&[path, "/"].concat()).unwrap(),
            ))
        })
        .boxed()
