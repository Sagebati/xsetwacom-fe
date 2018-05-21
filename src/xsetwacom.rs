
#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub typ: String,
    pub hid: String,
    pub id: u8,
}

#[derive(Debug)]
pub struct Set {
    pub typ: String,
    pub id: u8,
    pub mapped_to: String,
    pub line: String,
}


pub mod parsing {
    use xsetwacom::Device;
    use xsetwacom::Set;
    use std::vec::Vec;


    pub fn generate_line_set(device_id: u8, typ: &str, id: u8, mapped_to: &str) -> String {
        let res: String;
        if id != 0 {
            res = format!("xsetwacom set \"{}\" \"{}\" \"{}\" \"{}\"", device_id, typ, id, mapped_to);
        } else {
            res = format!("xsetwacom set \"{}\" \"{}\" \"{}\"", device_id, typ, mapped_to);
        }
        return res;
    }

    pub fn parse_list(output: &str) -> Vec<Device> {
        let lines = output.lines();
        let mut devices: Vec<Device> = Vec::new();
        for line in lines {
            devices.push(parse_line(line));
        }
        return devices;
    }

    fn parse_line(line: &str) -> Device {
        print!("{}", line);
        let mut name: String = "Not founded".to_owned();
        let mut id: u8 = 0;
        let mut hid: String = "XXXX_XXXX".to_owned();
        let mut typ: String = "XXX".to_owned();
        let words: Vec<&str> = line.split_whitespace().collect();
        for i in 0..words.len() {
            match words[i] {
                "HID" => {
                    hid = words[i + 1].to_owned();
                    let mut s: String = words[i + 2].to_owned();
                    s.push(' ');
                    s.push_str(words[i + 3]);
                    name = s;
                }
                "id:" => {
                    id = words[i + 1].parse().unwrap();
                }
                "type:" => {
                    typ = words[i + 1].to_owned()
                }
                _ => {}
            }
        }
        return Device { name, typ, hid, id };
    }

    pub fn parse_all_output(all: &str) -> Vec<Set> {
        let mut res: Vec<Set> = Vec::new();
        for line in all.lines() {
            res.push(parse_line_all(line));
        }
        return res;
    }

    fn parse_line_all(line: &str) -> Set {
        let mut typ: String = "XXX".to_owned();
        let mut id = 0u8;
        let mut mapped_to: String = "Undefined".to_owned();
        let words: Vec<String> = complex_split_line_all(line);
        for i in 0..words.len() {
            match words[i].as_ref() {
                "xsetwacom" => {
                    typ = words[3].clone();
                    if words.len() == 6 {
                        id = words[4].replace('"', "").parse().unwrap();
                        mapped_to = words[5].clone();
                    } else if words.len() == 5 {
                        mapped_to = words[4].clone();
                    } else {
                        //mapped_to = words[4].clone();
                    }
                }
                "Property" => {}
                _ => {}
            }
        }
        return Set { typ, id, mapped_to, line: line.to_string() };
    }

    fn complex_split_line_all(line: &str) -> Vec<String> {
        let mut in_quotes = false;
        let mut words: Vec<String> = Vec::new();
        let mut s = String::new();
        for c in line.chars() {
            if in_quotes {
                if c == '"' {
                    in_quotes = false;
                }
                s.push(c);
            } else {
                if c != ' ' {
                    s.push(c)
                }
                if c == '"' {
                    in_quotes = true;
                }
                if c == ' ' && !in_quotes {
                    words.push(s.clone());
                    s.clear();
                }

            }
        }
        words.push(s.clone());
        return words;
    }
}

pub mod getting {
    use xsetwacom::Device;
    use std::process::Command;
    use xsetwacom::parsing::*;
    use xsetwacom::Set;

    pub fn get_devices() -> Vec<Device> {
        let output = Command::new("xsetwacom").args(&["--list"]).output().expect("failed to \
        execute xsetwacom, maybe not installed or not in the path");

        return parse_list(&String::from_utf8_lossy(&output.stdout).to_string());
    }

    pub fn _get_all(device: Device) -> String {
        let output = Command::new("xsetwacom").args(&["-s", "--get", &device.id.to_string(),
            "all"]).output().expect("Could not get the configuration of the device");
        return String::from_utf8_lossy(&output.stdout).to_string();
    }

    pub fn get_all_by_id(id: u8) -> String {
        let output = Command::new("xsetwacom").args(&["-s", "--get", &id.to_string(),
            "all"]).output().expect("Could not get the configuration of the device");
        return String::from_utf8_lossy(&output.stdout).to_string();
    }

    pub fn get_sets_from_id(id: u8) -> Vec<Set> {
        return parse_all_output(&get_all_by_id(id));
    }
}
