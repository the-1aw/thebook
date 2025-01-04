use std::io;

pub fn write_result_io(buff: &mut impl io::Write, content: &[u8]) -> io::Result<usize> {
    buff.write(content)
}

pub fn write_result(buff: &mut impl io::Write, content: &[u8]) -> Result<usize, io::Error> {
    buff.write(content)
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn function() {
    println!("420");
}

fn take_long_type(f: Thunk) {
    f()
}

fn returns_long_type() -> Thunk {
    Box::new(function)
}

pub fn never() -> ! {
    loop {}
}

fn generic<T: ?Sized>(_t: &T) {}

fn main() {
    let f = returns_long_type();
    take_long_type(f);

    // This makes no sense irl but since Box is not a Dynamicly sized type,
    // we use a ref to the dyn Fn inside the box to try out the ?Sized generic type.
    generic(&(*returns_long_type()));
}
