fn i() -> usize {
    0
}

fn h(_x: usize) {}

fn g() {
    h(i());
}

fn f() {
    g();
}

fn main() {
    f();
    h(0);
}
