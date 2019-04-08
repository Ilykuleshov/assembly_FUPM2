mod txtparse;
mod cmdspec;
mod procexec;

fn main() {
    let mut res = txtparse::parsecode("syscall r10 0\n halt r10 -1");
    res.exec();

    for cmd in res.state.mem {
    	println!("{:#018b}\n", cmd);
    }
}