use std::fmt;
use std::ops::{Add, Mul};

// By default, we treat Values as leaf nodes
// i.e. no gradient and no backward function.

#[derive(Debug)]
enum ValueKind {
    Leaf,
    Add,
    Mul,
    // Pow,
}

#[derive(Debug)]
struct Value {
    data: f32,
    grad: f32, // TODO: change to Option<f32>?
    kind: ValueKind,
    kids: Vec<Value>,
}

impl Default for Value {
    fn default() -> Self {
        Self {
            data: 0.0,
            grad: 0.0,
            kind: ValueKind::Leaf,
            kids: vec![],
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V({:?}, {}, {:?})", self.kind, self.data, self.grad)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: self.data + other.data, // TODO: other can be just an f32
            kind: ValueKind::Add,
            kids: vec![self, other],
            ..Default::default()
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: self.data * other.data,
            kind: ValueKind::Mul,
            kids: vec![self, other],
            ..Default::default()
        }
    }
}

impl Value {
    fn backward(mut self, upstream: f32) -> f32 {
        // match self.kind {
        //     ValueKind::Add => {
        //         self.kids[0].backward(1.0 * upstream);
        //         self.kids[1].backward(1.0 * upstream);
        //     }

        //     ValueKind::Mul => {
        //         self.kids[0].backward(self.kids[1].data * upstream);
        //         self.kids[1].backward(self.kids[0].data * upstream);
        //     }

        //     ValueKind::Leaf => 0.0,
        // }
        unimplemented!();
    }
}

fn main() {
    let a = Value {
        data: 2.0,
        ..Default::default()
    };
    let b = Value {
        data: 3.0,
        ..Default::default()
    };

    let y = a + b;

    println!("{}", y);
}
