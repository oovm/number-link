use super::*;
use ndarray::Axis;
use rs_graph::{
    maxflow::{dinic, pushrelabel, MaxFlow},
    string::{from_ascii, Data},
    traits::*,
    vec::NodeIndexer,
    vecgraph::Edge,
    EdgeVec, Graph, LinkedListGraph, Net,
};
use std::collections::BTreeSet;

impl FlowFreeBoard {
    unsafe fn get_edges(&self) -> BTreeSet<(usize, usize, usize)> {
        let mut out = BTreeSet::new();
        let mut color = BTreeSet::new();
        let h = self.board.shape().get_unchecked(0);
        let w = self.board.shape().get_unchecked(1);
        let mut index = 1;
        for line in self.board.lanes(Axis(0)).into_iter() {
            for n in line.iter() {
                if *n > 0 {
                    match color.insert(*n) {
                        true => out.insert((0, index, 1)),
                        false => out.insert((index, self.colors + 1, 1)),
                    };
                }
                if i + 1 < *h {
                    let right = self.board[[i + 1, j]];
                    if right < 0 {
                        out.insert((*n as usize, -right as usize, 1));
                    }
                    else if right > 0 {
                        out.insert((*n as usize, -right as usize, 1));
                    }
                }
                if j + 1 < *w {
                    let down = self.board[[i, j + 1]];
                    if down > 0 {}
                }
                index += 1
            }
        }
        out
    }
    fn get_colors(&self) {}

    fn as_graph(&self) -> Net {
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
    unsafe {
        FlowFreeBoard::from_str(
            r#"
    1  -  2  3 -
    -  2  -  - -
    -  3  -  4 -
    -  1  -  - -
    "#,
        )
        .unwrap()
        .get_edges();
    }
}
