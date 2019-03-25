mod txtparse;
mod cmdspec;

fn main() {
    let table = cmdspec::init_cmd_table();
    let res = txtparse::parsecode("syscall r10 -1\nhalt r10 -1", &table);

    for cmd in res.mem {
    	println!("{:#018b}\n", cmd);
    }
}