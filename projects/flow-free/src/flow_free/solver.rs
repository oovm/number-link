use super::*;
use rs_graph::{
    maxflow::{dinic, pushrelabel, MaxFlow},
    string::{from_ascii, Data},
    traits::*,
    vec::NodeIndexer,
    vecgraph::Edge,
    EdgeVec, Graph, LinkedListGraph, Net,
};

impl FlowFreeBoard {
    fn get_edges(&self) {}
    fn get_colors(&self) {}

    fn as_graph(&self) -> Net {
        let mut out = LinkedListGraph::new();

        let Data { graph: g, weights: upper, nodes } = from_ascii::<Net>(
            r"
     a---2-->b
    @|\      ^\
   / | \     | 4
  5  |  \    |  \
 /   |   |   |   @
s    1   1   2    t
 \   |   |   |   @
  5  |    \  |  /
   \ |     \ | 5
    @v      @|/
     c---2-->d
    ",
        )
        .unwrap();

        let s = nodes[&'s'];
        let t = nodes[&'t'];
        let v1 = nodes[&'a'];
        let v2 = nodes[&'b'];
        let v3 = nodes[&'c'];
        let v4 = nodes[&'d'];

        let (value, flow, mut mincut) = dinic(&g, s, t, |e| upper[e.index()]);

        mincut.sort_by_key(|u| u.index());
        println!("{:?}", value);
        for i in flow.iter() {
            println!("{:?}", i);
        }
        println!("{:?}", mincut);
        todo!()
    }
}

#[test]
fn test() {
    FlowFreeBoard::from_str(
        r#"
    1  -  2  3
    -  2  -  -
    -  3  -  4
    -  1  -  -
    "#,
    )
    .unwrap()
    .as_graph();
}
