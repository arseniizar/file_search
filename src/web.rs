use warp::{reply::html, Filter, Reply};

pub async fn start() {
    let menu_route = warp::path::end().map(|| {
        html(
            "<html>
                <head><title>Menu</title></head>
                <body>
                    <h1>Welcome to Warp!</h1>
                    <ul>
                        <li><a href=\"/index\">Index</a></li>
                        <li><a href=\"/search/test\">Search for 'test'</a></li>
                    </ul>
                </body>
            </html>",
        )
    });

    let index_route = warp::path("index").map(|| "Indexing files...");

    let search_route =
        warp::path!("search" / String).map(|query| format!("Searching for: {}", query));

    let routes = menu_route.or(index_route).or(search_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
