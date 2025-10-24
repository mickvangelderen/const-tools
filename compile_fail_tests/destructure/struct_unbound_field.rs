use const_tools::destructure;

struct Wrap<T> {
    value: T,
}

impl<T> Wrap<T> {
    const fn test(self) {
        destructure!(let Wrap { } = self);
    }
}

fn main() {}
