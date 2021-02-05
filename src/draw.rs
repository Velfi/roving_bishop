use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum CellArt {
    Empty,
    No1,
    No2,
    No3,
    No4,
    No5,
    No6,
    No7,
    No8,
    No9,
    No10,
    No11,
    No12,
    No13,
    No14,
    Start,
    End,
}

//            11111 11
// 0 12345678901234 56
//   .o+=*BOX@%&#/^ SE

impl CellArt {
    pub fn inc(&self) -> Self {
        use CellArt::*;
        match *self {
            Empty => No1,
            No1 => No2,
            No2 => No3,
            No3 => No4,
            No4 => No5,
            No5 => No6,
            No6 => No7,
            No7 => No8,
            No8 => No9,
            No9 => No10,
            No10 => No11,
            No11 => No12,
            No12 => No13,
            No13 => No14,
            No14 => No14,
            Start => Start,
            End => End,
        }
    }
}

impl Display for CellArt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ca = match self {
            CellArt::Empty => " ",
            CellArt::No1 => ".",
            CellArt::No2 => "o",
            CellArt::No3 => "+",
            CellArt::No4 => "=",
            CellArt::No5 => "*",
            CellArt::No6 => "B",
            CellArt::No7 => "O",
            CellArt::No8 => "X",
            CellArt::No9 => "@",
            CellArt::No10 => "%",
            CellArt::No11 => "&",
            CellArt::No12 => "#",
            CellArt::No13 => "/",
            CellArt::No14 => "^",
            CellArt::Start => "S",
            CellArt::End => "E",
        };

        write!(f, "{}", ca)
    }
}

// impl Display for CellArt {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let ca = match self {
//             CellArt::Empty => " ",
//             CellArt::No1 => "✨",
//             CellArt::No2 => "💕",
//             CellArt::No3 => "😭",
//             CellArt::No4 => "🙏",
//             CellArt::No5 => "🥰",
//             CellArt::No6 => "😂",
//             CellArt::No7 => "👍",
//             CellArt::No8 => "😍",
//             CellArt::No9 => "🤣",
//             CellArt::No10 => "😊",
//             CellArt::No11 => "🥺",
//             CellArt::No12 => "😭",
//             CellArt::No13 => "😭",
//             CellArt::No14 => "😘",
//             CellArt::Start => "🦀",
//             CellArt::End => "🍄",
//         };

//         write!(f, "{}", ca)
//     }
// }
