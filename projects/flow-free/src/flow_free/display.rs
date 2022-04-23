use super::*;

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
        let mut colors = BTreeMap::<i8, Vec<(usize, usize)>>::new();
        let mut board = Array2::from_elem((h, w), -1);
        for (i, line) in game.lines().enumerate() {
            for (j, item) in line.split_ascii_whitespace().enumerate() {
                if let Ok(n) = item.parse::<i8>() {
                    board[[i, j]] = n;
                    match colors.get_mut(&n) {
                        Some(s) => {
                            s.push((i, j));
                        }
                        None => {
                            let mut v = Vec::with_capacity(2);
                            v.push((i, j));
                            colors.insert(n, v);
                        }
                    }
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
        if *colors.iter().last().ok_or(())?.0 != colors.len() as i8 {
            return Err(());
        }
        Ok(Self { board, colors })
    }
}
