#[macro_use]
extern crate text_io;


use std::io::{self, Write};

mod xsetwacom;
use xsetwacom::{Device, Set};
use xsetwacom::getting::get_sets_from_id;
use xsetwacom::parsing::generate_line_set;


fn main() {
    let devices: Vec<Device> = xsetwacom::getting::get_devices();
    'main: loop {
        menu(&devices);
        println!("Type the number of the device you want to configure:");
        print!("$:");
        io::stdout().flush().expect("Not flushed");
        let input: u8 = read!();
        configure(input);
    }
}

fn configure(id: u8) -> Vec<Set> {
    let sets: Vec<Set> = get_sets_from_id(id);
    let mut new_sets: Vec<Set> = Vec::new();
    println!("Would you like to configure all the bindings [1]/[0]");
    print!("$:");
    io::stdout().flush().expect("Not flushed");
    let input: u8 = read!();
    if input == 0 {
        for set in sets {
            println!("Configuring :");
            print!("Type : {}", set.typ);
            if set.id != 0 {
                print!(" ID : {}", set.id);
            }
            print!("Binding: {} \n", set.mapped_to);
            println!("Would you like to configure that 1/0:");
            print!("$:");
            io::stdout().flush().expect("Not flushed");
            let input_config: u8 = read!();
            if input_config == 1 {
                print!("New mapping:");
                io::stdout().flush().expect("Not flushed");
                let new_mapping: String = read!();
                let line = generate_line_set(id, &set.typ, set.id, &set.mapped_to);
                new_sets.push(
                    Set {
                        typ: set.typ.clone(),
                        id: set.id,
                        mapped_to: new_mapping.clone(),
                        line,
                    });
            }
        }
    } else {
        for set in sets {
            println!("Configuring :");
            print!("Type : {}", set.typ);
            if set.id != 0 {
                print!(" ID : {}", set.id);
            }
            print!(" Binding: {}\n", set.mapped_to);
            print!("New mapping:");
            io::stdout().flush().expect("Not flushed");
            let new_mapping: String = read!();
            let line = generate_line_set(id, &set.typ, set.id, &set.mapped_to);
            new_sets.push(
                Set {
                    typ: set.typ.clone(),
                    id: set.id,
                    mapped_to: new_mapping.clone(),
                    line,
                });
        }
    }
    return new_sets;
}


fn menu(devices: &Vec<Device>) {
    println!("This are your devices connected on the computer:");
    for d in devices.iter() {
        println!("{} Name : {}", d.id, d.name);
    }
}

