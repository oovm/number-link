use std::str::FromStr;
use ndarray::Array2;


pub struct FlowFree {
    /// -1 : cap = 1
    ///  0 : cap = 0
    pub board: Array2<i8>,
}

impl FromStr for FlowFree {
    type Err = ();
    /// - `-`: path
    /// - `x`: bridges, can through crossed
    /// - `#`: wall, cannot be passed
    ///
    /// ```game
    /// 1  -  2  3
    /// -  2  -  -
    /// -  3  -  4
    /// -  1  -  -
    /// ```
    fn from_str(game: &str) -> Result<Self, Self::Err> {
        let game = game.trim();
        let h = game.lines().count();
        let w = game.lines().nth(0).ok_or(())?.split_whitespace().count();
        println!("{} {}", h, w);
        let mut board = Array2::from_elem((h, w), -1);
        println!("{:?}", board);
        Ok(Self {
            board,
        })
    }
}

#[test]
fn test() {
    FlowFree::from_str(r#"
    1  -  2  3
    -  2  -  -
    -  3  -  4
    -  1  -  -
    "#);
}