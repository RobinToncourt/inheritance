use inheritance::{prototype, extends};

#[prototype]
struct A {
    a: usize,
    b: bool,
    c: String,
}

impl A {
    fn new() -> Self {
        Self {
            a: 1,
            b: true,
            c: "c".to_string(),
        }
    }
}

#[prototype]
impl A {
    fn get_a(&self) -> usize {
        self.a
    }

    fn set_b(&mut self, b: bool) {
        self.b = b;
    }

    fn into_string(self) -> String {
        self.c
    }

    fn from_a(&self) -> String {
        println!("Function implemented on struct A.");
        "coucou".to_string()
    }
}

#[extends(A, C, D)]
#[derive(Debug)]
struct B {
    d: i8,
    e: Vec<u16>,
}

fn main() {
    println!("Bonjour.");

    let b = B {
        a: 1,
        b: false,
        c: "3".to_string(),
        d: 4,
        e: vec![5],
    };

    println!("{b:#?}");
    println!("{}", b.from_a());
}
