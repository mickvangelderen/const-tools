use const_tools::destructure;

fn main() {
    destructure!(let (a,) = (1, 2));
}
