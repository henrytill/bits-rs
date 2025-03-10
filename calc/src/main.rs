use calc::{parser, semantics};

fn main() {
    let input = parser::parse_expr("40 + 2").unwrap();
    let actual = semantics::simplify(input).unwrap();
    println!("actual: {:?}", actual);
}
