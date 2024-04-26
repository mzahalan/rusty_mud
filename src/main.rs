

#[link(name="mud", kind="static")]
extern "C" {
    pub fn do_main(port:i32);
}

fn main() {
    unsafe {
        do_main(7000);
    }
}
