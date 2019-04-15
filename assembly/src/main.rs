mod txtparse;
mod cmdspec;
mod procexec;

fn main() {
    let mut res = txtparse::parsecode("syscall r8 100\n divi r8 2\n syscall r8 102\n syscall r9 102\n halt r10 0");
    res.exec();
}