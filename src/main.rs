mod file;
mod primitives;
mod print;

fn main() {
    print::print_contents(file::read_file("hosts-copy").expect("Error while reading file."));
}
