mod txtparse;
mod cmdspec;

fn main() {
    let table = cmdspec::init_cmd_table();
    let res = txtparse::makeword("syscall r10 -1", table);

    let var = cmdspec::CpuState::new();
    println!("{:#018b}", res);
}
