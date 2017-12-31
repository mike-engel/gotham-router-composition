use gotham::router::tree::node::{NodeBuilder, SegmentType};

pub mod ping;
pub mod users;

pub fn router() -> NodeBuilder {
    let mut node = NodeBuilder::new("api", SegmentType::Static);

    node.add_child(ping::router());
    node.add_child(users::router());

    node
}
