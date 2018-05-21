#[macro_use]
extern crate text_io;

use std::io::{self, Write};

mod xsetwacom;
mod fe;

use xsetwacom::{Device, Set};
use xsetwacom::getting::get_sets_from_id;
use xsetwacom::parsing::generate_line_set;
use std::fs::File;
use fe::gtk_fe;



fn main() {
    let devices: Vec<Device> = xsetwacom::getting::get_devices();
    gtk_fe::init();
    /* 'main: loop {
        menu(&devices);
        println!("Type the number of the device you want to configure:");
        print!("$:");
        io::stdout().flush().expect("Not flushed");
        let input: u8 = read!();
        let inputt: char = read!();
        println!("Enter character: {}", inputt);
        let new_sets  = configure(input);
    } */
}

fn sets_to_bash(sets: &Vec<Set>){
    let mut f: File = File::create("xsetwacom.sh").expect("Could not open xsetwacom.sh");
    let mut to_write: String = "#!bin/bash\n".to_owned();
    for s in sets{
        to_write.push_str(&s.line);
    }
    f.write(&to_write.into_bytes()).expect("Could not write into xsetwacom.sh");
}

fn apply_sets(sets: &Vec<Set>){

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

