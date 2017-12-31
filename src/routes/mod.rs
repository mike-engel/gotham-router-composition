pub mod api;

use hyper::Method;

use gotham::handler::NewHandler;
use gotham::router::Router;
use gotham::router::route::{Delegation, Extractors, Route, RouteImpl};
use gotham::router::route::matcher::MethodOnlyRouteMatcher;
use gotham::router::route::dispatch::{finalize_pipeline_set, new_pipeline_set, DispatcherImpl};
use gotham::router::request::path::NoopPathExtractor;
use gotham::router::request::query_string::NoopQueryStringExtractor;
use gotham::router::response::finalizer::ResponseFinalizerBuilder;
use gotham::router::tree::TreeBuilder;

pub fn static_route<NH>(methods: Vec<Method>, new_handler: NH) -> Box<Route + Send + Sync>
where
    NH: NewHandler + 'static,
{
    let matcher = MethodOnlyRouteMatcher::new(methods);
    let pipeline_set = finalize_pipeline_set(new_pipeline_set());
    let extractors: Extractors<NoopPathExtractor, NoopQueryStringExtractor> = Extractors::new();
    let dispatcher = DispatcherImpl::new(new_handler, (), pipeline_set);
    let route = RouteImpl::new(
        matcher,
        Box::new(dispatcher),
        extractors,
        Delegation::Internal,
    );

    Box::new(route)
}

pub fn router() -> Router {
    let mut tree_builder = TreeBuilder::new();
    let api = api::router();

    tree_builder.add_child(api);

    let tree = tree_builder.finalize();

    let response_finalizer_builder = ResponseFinalizerBuilder::new();
    let response_finalizer = response_finalizer_builder.finalize();

    Router::new(tree, response_finalizer)
}
