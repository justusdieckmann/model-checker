mod parsing;

fn main() {
    dbg!(parsing::parse("a & b U c & a").expect("Got no result"));

    dbg!(parsing::parse("a & (b U c) & a").expect("Got no result"));
}

