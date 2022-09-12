use std::vec::Vec;

fn main<'a>() {
    struct State {
        lm: u32,
        lc: u32,
        rm: u32,
        rc: u32,
        pb: &'a str,
    }
    let mut state = State {
        lm: 2,
        lc: 2,
        rm: 0,
        rc: 0,
        pb: "L",
    };
    let mut open: Vec<State> = Vec::new();
    open.push(state);
    let mut closed: Vec<State> = Vec::new();
    let k = open.pop().unwrap();
    closed.push(k);
    if k.pb == "L" {
        // 左の船で、右に2人まで移動させることを考える
    } else if k.pb == "R" {
        // 右の船で、右に2人まで移動させることを考える
    } else {
        panic!();
    }
}