use std::fs::File;

use std::io::Write;
fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn main() {
    let mut local_filea = File::create("hello.txt")?;
    say_hello(&mut local_file)?; // works
    let mut bytes = vec![];

    say_hello(&mut bytes)?; // also works
    assert_eq!(bytes, b"hello world\n");
}
