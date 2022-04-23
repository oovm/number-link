use super::*;
use ndarray::Axis;
use rs_graph::{
    maxflow::dinic,
    string::{from_ascii, Data},
    traits::*,
    Net,
};
use std::collections::BTreeSet;

impl FlowFreeBoard {
    fn get_start(&self) -> usize {
        0
    }
    fn get_end(&self) -> usize {
        self.colors.len() + 1
    }
    pub fn get_vertex_capacity(&self) -> Vec<u8> {
        let mut out = vec![0];
        for i in &self.board {
            match -i as u8 {
                n if *i < 0 => out.push(n),
                _ if *i > 0 => out.push(1),
                _ => out.push(0),
            }
        }
        out.push(0);
        out
    }

    pub fn get_edges(&self) -> BTreeSet<(usize, usize)> {
        let mut out = BTreeSet::new();
        let mut color = BTreeSet::new();
        let (h, w) = match self.board.shape() {
            [h, w] => (h, w),
            _ => panic!("Dimension mismatch"),
        };
        for (i, line) in self.board.lanes(Axis(0)).into_iter().enumerate() {
            for (j, n) in line.iter().enumerate() {
                let index = i * w + j;
                if *n > 0 {
                    match color.insert(*n) {
                        // 流入
                        true => out.insert((self.get_start(), index)),
                        // 流出
                        false => out.insert((index, self.get_end())),
                    };
                }
                if i + 1 <= *w {
                    let right = self.board[[j, i + 1]];
                    let right_index = index + 1;
                    if right != 0 {
                        out.insert((index, right_index));
                    }
                }
                if j + 1 <= *h {
                    let down = self.board[[j + 1, i]];
                    let down_index = index + w;
                    if down != 0 {
                        out.insert((index, down_index));
                    }
                }
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
    let g = FlowFreeBoard::from_str(
        r#"
    1  -  2  3 -
    -  2  -  - -
    -  3  -  4 -
    -  1  -  - -
    "#,
    )
    .unwrap();
    println!("{:?}", g.get_vertex_capacity());
    g.get_edges();
}
