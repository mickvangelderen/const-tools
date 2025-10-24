use const_tools::destructure;

struct Wrap<T> {
    value: T,
}

impl<T> Drop for Wrap<T> {
    fn drop(&mut self) {}
}

impl<T> Wrap<T> {
    const fn test(self) -> T {
        destructure!(let Wrap { value } = self);
        value
    }
}

fn main() {}
