use sysinfo::System;
use colored::*;
use std::env;
use std::net::UdpSocket;
use std::time::Duration;


fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.set_read_timeout(Some(Duration::from_secs(1))).ok()?;
    socket.set_write_timeout(Some(Duration::from_secs(1))).ok()?;
    if socket.connect("8.8.8.8:80").is_err() {
        return None; 
    }
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let cat_type = if args.len() > 2 && (args[1] == "-t" || args[1] == "--type") {
        args[2].parse::<usize>().unwrap_or(1)
    } else {
        1
    };

    // Define different cat arts
    // Define the big cat ASCII art
    let cat_art_1 = "                     \n    \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m     \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m    \n   \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;253;171;214m\x1b[38;2;253;171;214m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;253;171;214m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;253;171;214m▄\x1b[0m\x1b[48;2;253;171;214m\x1b[38;2;253;171;214m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m   \n  \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m  \n \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m \n\x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;8;27;94m\x1b[38;2;94;206;207m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;8;27;94m\x1b[38;2;94;206;207m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\n\x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\n  \x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m  \n     \x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m     ";
    let cat_art_2 = "                  \n       \x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m     \x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m  \n       \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m  \n \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m  \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m \n \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;223;113;38m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;223;113;38m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m \n  \x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m  \n    \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m   \n    \x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;49;49;49m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;49;49;49m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;49;49;49m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;49;49;49m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m   \n";


    let cat_art = match cat_type {
        1 => cat_art_1,
        2 => cat_art_2,
        _ => cat_art_1,
    };

    // Initialize the system information
    let mut sys = System::new_all();
    sys.refresh_all();

    // Convert bytes to GB
    let total_memory_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_memory_gb = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let memory_percentage = (used_memory_gb / total_memory_gb) * 100.0;
    let total_swap_gb = sys.total_swap() as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_swap_gb = sys.used_swap() as f64 / (1024.0 * 1024.0 * 1024.0);
    let swap_percentage = (used_swap_gb / total_swap_gb) * 100.0;

    // Get CPU information using sysinfo
    let cpu_name = if let Some(cpu) = sys.cpus().first() {
        cpu.brand().to_string()
    } else {
        "Unknown CPU".to_string()
    };

    // Get user and host information
    let user_info = format!("{}{}{}", whoami::username(), "@", System::host_name().unwrap());

    // Split the cat art into lines
    let cat_art_lines: Vec<&str> = cat_art.lines().collect();
    let cat_art_height = cat_art_lines.len();

    // Get system name and OS version, or use "Unknown" if unavailable
    let system_name = System::name().unwrap_or("Unknown".to_string());
    let os_version = System::os_version().unwrap_or("Unknown".to_string());

    let system_info = format!(
        "{}: {} {}",
        "sys ".bright_blue(),
        system_name,
        os_version
    );

    // Colorize the labels and their values
    let cpu_info = format!(
        "{}: {}",
        "cpu ".bright_blue(),
        cpu_name
    );

    // Function to colorize percentage based on value
    fn colorize_percentage(percentage: f64) -> ColoredString {
        let percentage_str = format!("({:.1}%)", percentage);
        if percentage < 50.0 {
            percentage_str.green()
        } else if percentage >= 50.0 && percentage < 90.0 {
            percentage_str.yellow()
        } else {
            percentage_str.red()
        }
    }

    let memory_status = format!(
        "{:.2} >> {:.2} GB {}",
        used_memory_gb,
        total_memory_gb,
        colorize_percentage(memory_percentage)
    );

    let memory_info = format!(
        "{}: {}",
        "mem ".bright_blue(),
        memory_status
    );

    let swap_status = format!(
        "{:.2} >> {:.2} GB {}",
        used_swap_gb,
        total_swap_gb,
        colorize_percentage(swap_percentage)
    );

    let swap_info = format!(
        "{}: {}",
        "swap".bright_blue(),
        swap_status
    );

    // Define the color blocks
    let bright_colors = format!(
        "{}{}{}{}{}{}{}{}",
        "███".bright_red(),
        "███".bright_yellow(),
        "███".bright_green(),
        "███".bright_cyan(),
        "███".bright_blue(),
        "███".bright_magenta(),
        "███".bright_black(),
        "███".bright_white()
    );
    let dark_colors = format!(
        "{}{}{}{}{}{}{}{}",
        "███".red(),
        "███".yellow(),
        "███".green(),
        "███".cyan(),
        "███".blue(),
        "███".magenta(),
        "███".black(),
        "███".white()
    );

    // Get local IP address
    let ip_info = match get_local_ip() {
        Some(ip) => format!("{}: {}", "ipv4".bright_blue(), ip),
        None => "".to_string(), 
    };

    // Collect all information lines
    let info_lines = vec![
        user_info.bright_green().to_string(),
        "━".repeat(user_info.len()),
        system_info,
        cpu_info,
        memory_info,
        swap_info,
        ip_info,
        bright_colors,
        dark_colors,
    ];
    let info_height = info_lines.len();

    // Calculate the number of empty lines needed to center the cat art vertically
    let padding = if info_height > cat_art_height {
        (info_height - cat_art_height) / 2
    } else {
        0
    };

    // Add padding to the cat art lines
    let mut padded_cat_art_lines = vec!["".to_string(); padding];
    padded_cat_art_lines.extend(cat_art_lines.iter().map(|&line| line.to_string()));
    while padded_cat_art_lines.len() < info_height {
        padded_cat_art_lines.push("".to_string());
    }

    // Print the cat art and info lines side by side
    for i in 0..info_height {
        let cat_line = &padded_cat_art_lines[i];
        let info_line = &info_lines[i];
        println!("{:<18} {}", cat_line, info_line);
    }
}