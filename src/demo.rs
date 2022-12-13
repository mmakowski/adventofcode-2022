struct A {
    x: u32
}

struct B {
    f: Box<dyn Fn(u32) -> u32>
}

fn mk_b(a: &A) -> B {
    let x = a.x.to_owned();
    let fc = move |y| { y + x };
    B { f: Box::new(fc) }
}