use std::process::Command;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;
use std::io::{self, Write};

// 新しい構造体の定義
struct DeviceInfo {
    ip_port: String,
    hostname: String,
    service_name: String,
}

fn browse_services(service_type: &str, search_duration: u64) -> Vec<DeviceInfo> {
    let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
    let receiver = mdns.browse(service_type).expect("Failed to create browser");

    let start_time = std::time::Instant::now();
    let mut devices = Vec::new();
    let mut services = std::collections::HashSet::new();

    while start_time.elapsed() < Duration::from_secs(search_duration) {
        if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
            if let ServiceEvent::ServiceResolved(info) = event {
                let service_name = info.get_fullname().to_string();
                if services.insert(service_name.clone()) {
                    for addr in info.get_addresses() {
                        if addr.is_ipv4() {
                            devices.push(DeviceInfo {
                                ip_port: format!("{}:{}", addr, info.get_port()),
                                hostname: info.get_hostname().to_string(),
                                service_name: service_name.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    devices
}

fn select_device(devices: &[DeviceInfo], auto_select: bool) -> Option<&DeviceInfo> {
    if devices.is_empty() {
        println!("No devices found.");
        return None;
    }

    // デバイスが1件のみの場合は、auto_selectフラグに応じて自動選択
    if devices.len() == 1 && auto_select {
        let device = &devices[0];
        println!("Found 1 device, automatically selected: {} ({}) - Service: {}", 
                 device.ip_port, device.hostname, device.service_name);
        return Some(device);
    }

    println!("Found devices:");
    for (index, device) in devices.iter().enumerate() {
        println!("{}: {} ({}) - Service: {}", index + 1, device.ip_port, device.hostname, device.service_name);
    }

    print!("Select a device by number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if let Ok(selection) = input.trim().parse::<usize>() {
        if selection > 0 && selection <= devices.len() {
            return Some(&devices[selection - 1]);
        }
    }
    println!("Invalid selection.");
    None
}

pub fn execute_adb_reserved_word(word: &str, search_duration: u64, auto_select: bool, additional_args: Vec<&str>) {
    let service_type = match word {
        "connect" => "_adb-tls-connect._tcp.local.",
        "pair" => "_adb-tls-pairing._tcp.local.",
        _ => {
            execute_adb_command(additional_args);
            return;
        }
    };

    println!("Searching for {} services...", service_type);
    let devices = browse_services(service_type, search_duration);

    if let Some(selected_device) = select_device(&devices, auto_select) {
        let mut command = Command::new("adb");
        command.arg(word);
        command.arg(selected_device.ip_port.clone());

        if word == "pair" {
            print!("Enter pairing code: ");
            io::stdout().flush().unwrap();
            let mut pair_code = String::new();
            io::stdin().read_line(&mut pair_code).expect("Failed to read line");
            command.arg(pair_code.trim());
        }

        let output = command.output().expect(&format!("Failed to execute adb {} command", selected_device.ip_port));
        println!("======= execute =======");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("error: {}", String::from_utf8_lossy(&output.stderr));

        // 追加のコマンドがある場合は実行
        if !additional_args.is_empty() {
            execute_adb_command_with_device(selected_device.ip_port.clone(), additional_args);
        }
    }
}

pub fn execute_adb_command(args: Vec<&str>) {
    let output = Command::new("adb")
        .arg("devices")
        .output()
        .expect("Failed to execute adb devices command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    if lines.len() <= 1 {
        println!("No devices found.");
        return;
    }

    println!("Found devices:");
    for (index, line) in lines.iter().skip(1).enumerate() {
        if !line.trim().is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                println!("{}: {}", index + 1, parts[0]);
            }
        }
    }

    print!("Select a device by number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if let Ok(selection) = input.trim().parse::<usize>() {
        if selection > 0 && selection <= lines.len() - 1 {
            let selected_device = lines[selection].split_whitespace().next().unwrap();
            println!("Selected device: {}", selected_device);

            execute_adb_command_with_device(selected_device.to_string(), args);
        } else {
            println!("Invalid selection.");
        }
    } else {
        println!("Invalid input.");
    }
}

pub fn execute_adb_command_with_device(device: String, args: Vec<&str>) {
    let mut command = Command::new("adb");
    command.arg("-s").arg(device);
    command.args(&args);

    let mut child = command.spawn().expect("Failed to execute adb command");
    child.wait().expect("Failed to wait on child process");
}
