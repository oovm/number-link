use std::num::ParseIntError;
use std::str::FromStr;
use ndarray::Array2;


pub struct FlowFreeBoard {
    /// - `-1` : cap = 1
    /// - `0` : cap = 0
    board: Array2<i8>,
}

impl FromStr for FlowFreeBoard {
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
        let w = game.lines().nth(0).ok_or(())?.split_ascii_whitespace().count();
        let mut board = Array2::from_elem((h, w), -1);
        for (i, line) in game.lines().enumerate() {
            for (j, item) in line.split_ascii_whitespace().enumerate() {
                if let Ok(n) = item.parse::<i8>() {
                    board[[i, j]] = n;
                    continue;
                }
                board[[i, j]] = match item {
                    "-" => { -1 }
                    _ => { continue; }
                }
            }
        }


        println!("{:?}", board);
        Ok(Self {
            board,
        })
    }
}

#[test]
fn test() {
    FlowFreeBoard::from_str(r#"
    1  -  2  3
    -  2  -  -
    -  3  -  4
    -  1  -  -
    "#);
}