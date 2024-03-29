use colored::Colorize;
use std::convert::Infallible;
use std::str::FromStr;
use warp::http::HeaderMap;
use warp::{filters::BoxedFilter, http::Uri, path::FullPath, redirect, Filter, Reply};
use std::net::{UdpSocket, SocketAddr};

// use UDP to get ip to serve on
pub fn get_ip() -> Option<[u8; 4]> {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind socket");
    socket.connect("1.1.1.1:80").expect("Could not connect to address");

    if let Ok(sa) = socket.local_addr() {
        if let SocketAddr::V4(sa) = sa {
            return Some(sa.ip().octets());
        }
    }
    None
}

#[tokio::main]
pub async fn serve(addr: &[u8], port: u16) {
    let current_dir = std::env::current_dir().expect("failed to read current directory");

    let routes = root_redirect()
        .or(warp::fs::dir(current_dir))
        .and(log_headers());

    println!(
        "{} {}",
        "WARNING:".yellow().bold(),
        "The serve function of htmanager is still in heavy development".bold()
    );
    println!("dev server hosted at port {} (^C to force shutdown dev server) connection logs will be displayed below:", port.to_string().cyan().italic());
    warp::serve(routes)
        .run(([addr[0], addr[1], addr[2], addr[3]], port))
        .await;
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

fn log_headers() -> impl Filter<Extract = (), Error = Infallible> + Copy {
    warp::header::headers_cloned()
        .map(|headers: HeaderMap| {
            println!("Connection established:");
            println!("{:#?}", headers);
        })
        .untuple_one()
}
