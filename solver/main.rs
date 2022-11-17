use std::io;

#[derive(Debug)]
struct Root {
    board: Vec<i32>,
    width: i8,
    height: i8,
}

pub fn main() {
    let mut input_width = String::new();
    let mut input_height = String::new();
    let mut input_board = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input_width).expect("Insira a largura");
    stdin.read_line(&mut input_height).expect("Insira a altura");
    stdin.read_line(&mut input_board).expect("Insira o tabuleiro");

    let mut root = Root {
        board: input_board.split_whitespace().map(|n| n.parse().expect("O tabuleiro deve ser uma lista numérica")).collect(),
        width: input_width.trim().parse().expect("A largura deve ser numérica"),
        height: input_height.trim().parse().expect("A altura deve ser numérica"),
    };

    println!("{:?}", root);
}