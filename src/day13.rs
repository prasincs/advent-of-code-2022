use std::any::type_name;
use std::cmp::Ordering;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Token {
    List(Vec<Token>),
    Int(i32),
}
impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            /* 4 cases
            (int, int)
            ([int], [int]),
            (int, [int]),
            ([int], int)
            */
            (Token::Int(a), Token::Int(b)) => a.cmp(b),
            (Token::List(a), Token::List(b)) => {
                // find the smollest
                for idx in 0..a.len().min(b.len()) {
                    let ord = a[idx].cmp(&b[idx]);
                    match ord {
                        Ordering::Equal => (),
                        _ => return ord,
                    }
                }
                // return the comparison, if everything so far matches because vector a needs to be smaller than vector b
                a.len().cmp(&b.len())
            }
            // encapsulate in list, so it now boils down to [int],[int]
            (Token::Int(a), Token::List(_b)) => Token::List(vec![Token::Int(*a)]).cmp(other),
            (Token::List(_a), Token::Int(b)) => self.cmp(&Token::List(vec![Token::Int(*b)])),
        }
    }
}

// just tried this https://gist.github.com/rust-play/eb9c04d890f65765191b27a29056a305
// fn is_pair_in_right_order<T: Ord>(vec1: Vec<&T>, vec2: Vec<&T>) -> bool {
//     for (v1_idx, v1) in vec1.iter().enumerate() {
//         // for (v2_idx, v2) in vec2.iter().enumerate() {
//         let v2 = vec2[v1_idx];
//         match type_of(v1) {
//             "i32" | "u32" => {
//                 if v1 > &v2 {
//                     return false;
//                 }
//             }
//             "&alloc::vec::Vec<u32>" => {
//                 if typeof(v2) != "&alloc::vec::Vec<u32>" {

//                 }
//             }
//         }
//     }
//     true
// }

pub fn run() {
    let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    assert_eq!(
        Ordering::Less,
        Token::List(vec![
            Token::Int(1),
            Token::Int(1),
            Token::Int(3),
            Token::Int(1),
            Token::Int(1)
        ])
        .cmp(&Token::List(vec![
            Token::Int(1),
            Token::Int(1),
            Token::Int(5),
            Token::Int(1),
            Token::Int(1)
        ]))
    )
}
