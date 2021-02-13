fn some_thing(_val: Demo) {

}

struct Demo {
    a: i32,
}

fn main() {
    let demo = Demo{a: 123};
    some_thing(demo);
    println!("{}", demo.a);
}