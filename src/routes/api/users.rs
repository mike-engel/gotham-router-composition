use mime;
use hyper::{Method, Request, Response, StatusCode};
use routes::static_route;
use gotham::http::response::create_response;
use gotham::state::State;
use gotham::router::tree::node::{NodeBuilder, SegmentType};

pub fn router() -> NodeBuilder {
    let mut node = NodeBuilder::new("users", SegmentType::Static);

    node.add_route(static_route(vec![Method::Get, Method::Options], || Ok(get)));

    node
}

pub fn get(state: State, _req: Request) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some(("".to_owned().into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}
