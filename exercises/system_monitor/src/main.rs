use sysinfo::System; // Import the System struct from the sysinfo crate

fn main() {
    let mut system = System::new_all();

    system.refresh_all();

    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    println!(
        "Total memory:            {} GB",
        system.total_memory() / 1024 / 1024
    );
    println!(
        "Used memory:             {} GB",
        system.used_memory() / 1024 / 1024
    );
    println!(
        "Total swap:              {} GB",
        system.total_swap() / 1024 / 1024
    );
    println!(
        "Used swap:               {} GB",
        system.used_swap() / 1024 / 1024
    );
}
