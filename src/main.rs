use warp::Filter;

#[cfg(not(feature = "hot_reload"))]
macro_rules! read_file {
    ($path:expr) => {{
        include_str!($path)
    }};
}

#[cfg(feature = "hot_reload")]
macro_rules! read_file {
    ($path:expr) => {{
        use std::fs::read_to_string;
        read_to_string(($path).replace("../", "")).unwrap()
    }};
}

#[tokio::main]
async fn main() {
    let root = warp::path!().map(|| warp::reply::html(read_file!("../route/root/index.html")));

    let unknown = warp::any().map(|| warp::reply::html(read_file!("../route/404/index.html")));

    let route = root.or(unknown);

    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}
