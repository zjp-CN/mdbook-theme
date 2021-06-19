fn main() {
    fn fn_<U, F: Fn() -> U>(f: F) -> U { f() }
    fn fn_once<U, F: FnOnce() -> U>(_: F) {}
    fn fn_mut<U, F: FnMut() -> U>(_: F) {}

    struct Tuple();
    let _a = fn_(Tuple);
    fn_once(Tuple);
    fn_mut(Tuple);

    enum A {
        Tuple(),
    }
    let _a = fn_(A::Tuple);
    fn_once(A::Tuple);
    fn_mut(A::Tuple);
}
