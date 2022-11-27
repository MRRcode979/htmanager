// Replacement code for the custom Web Server
use std::str::FromStr;
use warp::{filters::BoxedFilter, http::Uri, path::FullPath, redirect, Filter, Reply};
use colored::Colorize;

#[tokio::main]
pub async fn serve(port: u16) {
    let current_dir = std::env::current_dir().expect("failed to read current directory");

    let routes = root_redirect().or(warp::fs::dir(current_dir));

    println!("{} {}", "WARNING:".yellow().bold(), "The serve function of htmanager is still in heavy development".bold());
    println!("local dev server hosted at port {} (^C to force shutdown dev server)", port.to_string().cyan().italic());

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
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
}
