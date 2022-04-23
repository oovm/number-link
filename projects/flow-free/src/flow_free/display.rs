use super::*;
use std::collections::BTreeSet;

impl Debug for FlowFreeBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for FlowFreeBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
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
        let mut color = BTreeSet::new();
        let mut board = Array2::from_elem((h, w), -1);
        for (i, line) in game.lines().enumerate() {
            for (j, item) in line.split_ascii_whitespace().enumerate() {
                if let Ok(n) = item.parse::<i8>() {
                    board[[i, j]] = n;
                    color.insert(n);
                    continue;
                }
                board[[i, j]] = match item {
                    "-" | "_" => -1,
                    "+" | "x" => -2,
                    "#" | "@" => 0,
                    _ => continue,
                }
            }
        }
        if color != BTreeSet::from_iter(0..color.len()) {
            return Err(());
        }
        Ok(Self { board, colors: color.len() })
    }
}
