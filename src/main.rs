use sysinfo::{System, SystemExt};
use prettytable::*;

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(&[
            format::LinePosition::Top,
            format::LinePosition::Bottom
        ], format::LineSeparator::new('-', '+', '+', '+'))
        .padding(1,1)
        .build();

    table.set_format(format);

    table.add_row(row![bFb->"OS NAME", os_name(&system)]);
    table.add_row(row![bFg->"MEM", total_ram(&system)]);
    table.add_row(row![bFr->"USED", used_ram(&system)]);
    table.add_row(row![bFc->"CPUS", cpu_cores(&system)]);
    table.add_row(row![bFm->"KERNEL", kernel_version(&system)]);

    table.printstd();
}

fn os_name(system: &System) -> String {
    system.name().expect("Error while reading OS Name")
}

fn total_ram(system: &System) -> String {
    format!("{:.2}GB", system.total_memory() as f64 / 1000000.)
}

fn used_ram(system: &System) -> String {
    format!("{:.2}GB", system.used_memory() as f64 / 1000000.)
}

fn cpu_cores(system: &System) -> usize {
    system.processors().len()
}

fn kernel_version(system: &System) -> String {
    system.kernel_version().expect("Error while reading Kernel version")
}
