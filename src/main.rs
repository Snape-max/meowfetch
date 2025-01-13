use sysinfo::System;
use colored::*;
use std::env;
use sysinfo::Disks;
use sysinfo::Networks;
use std::fs;
use dirs;


fn parse_escape_sequences(input: &str) -> String {
    // Replace common escape sequences with their actual control characters
    input
        .replace("\\x1b", "\x1b")  // ESC (Escape)
        .replace("\\n", "\n")      // Newline
        .replace("\\t", "\t")      // Tab
        .replace("\\r", "\r")      // Carriage return
        .replace("\\\"", "\"")     // Double quote
        .replace("\\'", "'")       // Single quote
        .replace("\\\\", "\\")     // Backslash
}

fn load_logo_from_config() -> Option<String> {
    // Get the home directory
    let home_dir = dirs::home_dir()?;

    // .meowrc path
    let meowrc_path = home_dir.join(".config").join(".meowrc");

    if !meowrc_path.exists() {
        return None;
    }
    let content = fs::read_to_string(meowrc_path).ok()?;
    if content.trim().is_empty() {
        return None;
    }
    // Parse escape sequences in the content
    let parsed_content = parse_escape_sequences(&content);

    Some(parsed_content)
}

// Function to colorize percentage based on value
fn colorize_percentage(percentage: f64) -> ColoredString {
    let percentage_str = format!("{:.1}%", percentage);
    if percentage < 50.0 {
        percentage_str.green()
    } else if percentage >= 50.0 && percentage < 90.0 {
        percentage_str.yellow()
    } else {
        percentage_str.red()
    }
}
fn get_local_ip() -> Option<String> {
    let networks = Networks::new_with_refreshed_list();
    let mut result = Vec::new();

    for (interface_name, network) in &networks {
        // Filter VMware networks
        if interface_name.contains("VMware") {
            continue;
        }

        // Filter empty Ip Networks 
        let ip_networks = network.ip_networks();
        if ip_networks.is_empty() {
            continue;
        }

        // Extract IPv4 addresses
        for ip_network in ip_networks {
            if ip_network.addr.is_ipv4() {
                let ip = ip_network.addr.to_string();
                let prefix = ip_network.prefix;
                result.push(format!("{}/{} ({})", ip, prefix, interface_name.cyan()));
            }
        }
    }

    // If there are multiple IPs, join them with ", "
    if !result.is_empty() {
        Some(result.join(", "))
    } else {
        None
    }
}


fn get_disk_info() -> String {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_info = Vec::new();
    let mut disk_count = 0;

    for disk in disks.list() {

        if disk_count >= 5 {
            break;
        }

        // Get the disk mount point
        let mount_point = disk.mount_point().to_string_lossy().to_string();

        // Get total disk size and available size (converted to GiB)
        let total_size_gb = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_size_gb = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);

        // Calculated usage percentage
        let used_percentage = ((total_size_gb - available_size_gb) / total_size_gb) * 100.0;

        // Get the file system type
        let file_system = disk.file_system().to_string_lossy().to_string();

        let prefix = format!(
            "disk ({})",
            mount_point
        );

        // Formatted output
        let info = format!(
            "{}: {:.2} GiB >> {:.2} GiB ({}) - {}",
            prefix.bright_blue(),
            total_size_gb - available_size_gb, // Used space
            total_size_gb,                     // Total space
            colorize_percentage(used_percentage), 
            file_system                        // File system type
        );

        disk_info.push(info);
        disk_count += 1;
    }

    disk_info.join("\n")
}


fn count_visible_chars(s: &str) -> usize {
    // Calculate the number of chars（include Unicode chars and spaces）
    s.chars()
        .filter(|&c| c == '▄' || c == '▀' || c == ' ')
        .count()
}

fn render_fetch(logo: &str, info: &str) {
    // Split the logo and info into lines
    let logo_lines: Vec<&str> = logo.lines().collect();
    let info_lines: Vec<&str> = info.lines().collect();

    // Determine the maximum number of lines
    let max_lines = std::cmp::max(logo_lines.len(), info_lines.len());

    // Calculate the maximum width of the logo lines (based on visible characters)
    let logo_width = logo_lines
        .iter()
        .map(|line| count_visible_chars(line))
        .max()
        .unwrap_or(0);

    // Calculate the number of padding lines needed to center the logo vertically
    let logo_padding = if logo_lines.len() < max_lines {
        (max_lines - logo_lines.len()) / 2
    } else {
        0
    };

    // Print the logo and info lines side by side
    for i in 0..max_lines {
        // Determine the logo line to print (centered vertically)
        let logo_line = if i >= logo_padding && i < logo_padding + logo_lines.len() {
            logo_lines[i - logo_padding]
        } else {
            ""
        };

        // Get the corresponding info line
        let info_line = info_lines.get(i).unwrap_or(&"");

        // Calculate the visible width of the logo line
        let visible_logo_width = count_visible_chars(logo_line);

        // Calculate the padding needed to align the info line
        let padding = logo_width.saturating_sub(visible_logo_width);

        // Print the logo line and info line with correct padding
        print!("{}", logo_line); // Print logo_line with color
        print!("{:padding$}", ""); // padding with spaces
        println!("{}", info_line); // print info_line
    }
}


fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let cat_type = if args.len() > 2 && (args[1] == "-t" || args[1] == "--type") {
        args[2].parse::<usize>().unwrap_or(1)
    } else {
        1
    };


    let logo = load_logo_from_config().unwrap_or_else(|| {
        // If the loading fails, use the logo specified by the command line parameter or the default logo
        match cat_type {
            1 => CAT_ART_3.to_string(),
            2 => CAT_ART_2.to_string(),
            3 => CAT_ART_1.to_string(),
            _ => CAT_ART_3.to_string(),
        }
    });

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

    let memory_status = format!(
        "{:.2} >> {:.2} GB ({})",
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
        "{:.2} >> {:.2} GB ({})",
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
        None => format!("{}: {}", "ipv4".bright_blue(), "unknown"), 
    };

    let disk_info = get_disk_info();

    // Collect all information lines
    let info = vec![
        user_info.bright_green().to_string(),
        "━".repeat(user_info.len()),
        system_info,
        cpu_info,
        memory_info,
        swap_info,
        ip_info,
        disk_info,
        bright_colors,
        dark_colors,
    ].join("\n");
    render_fetch(&logo, &info);

}



// Define different cat arts
// Define the big cat ASCII art
const CAT_ART_1: &str = "                     \n    \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m     \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m    \n   \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;253;171;214m\x1b[38;2;253;171;214m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;253;171;214m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;253;171;214m▄\x1b[0m\x1b[48;2;253;171;214m\x1b[38;2;253;171;214m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m   \n  \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m  \n \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m \n\x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;8;27;94m\x1b[38;2;94;206;207m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;8;27;94m\x1b[38;2;94;206;207m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;47;29;12m▄\x1b[0m\x1b[48;2;164;150;136m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\n\x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;47;29;12m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;86;70;55m▄\x1b[0m\x1b[48;2;86;70;55m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;164;150;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\n  \x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m  \n     \x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m     ";
const CAT_ART_2: &str = "                  \n       \x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m     \x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m  \n       \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m  \n \x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m  \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m \n \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;223;113;38m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;223;113;38m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m \n  \x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m  \n    \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m   \n    \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;49;49;49m\x1b[38;2;49;49;49m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m   \n                  ";
const CAT_ART_3: &str = "    \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m                     \n   \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m                   \n  \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m           \n  \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[38;2;0;0;0m▄\x1b[0m   \n  \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;145;150;164m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m \n \x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;145;150;164m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;250;186;201m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m \n\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;21;203;251m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;21;203;251m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;21;203;251m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;250;186;201m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m  \n\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;251;106;136m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;251;106;136m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;251;106;136m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;21;203;251m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;21;203;251m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;21;203;251m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;224;224;224m\x1b[38;2;224;224;224m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m   \n\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;251;106;136m\x1b[38;2;251;106;136m▄\x1b[0m\x1b[48;2;251;106;136m\x1b[38;2;251;106;136m▄\x1b[0m\x1b[48;2;251;106;136m\x1b[38;2;251;106;136m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;21;203;251m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m    \n\x1b[38;2;0;0;0m▀\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;222;219;220m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;251;106;136m▄\x1b[0m\x1b[48;2;251;106;136m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;251;106;136m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;251;106;136m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m    \n\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;222;219;220m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;222;219;220m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;222;219;220m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;222;219;220m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;222;219;220m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;255;255;255m▄\x1b[0m\x1b[48;2;255;255;255m\x1b[38;2;222;219;220m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;1;1;1m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;254;254;254m▄\x1b[0m\x1b[38;2;1;1;1m▄\x1b[0m    \n\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;158;155;156m▄\x1b[0m\x1b[48;2;158;155;156m\x1b[38;2;225;221;222m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;158;155;156m▄\x1b[0m\x1b[48;2;158;155;156m\x1b[38;2;225;221;222m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;0;0;0m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;222;219;220m\x1b[38;2;0;0;0m▄\x1b[0m\x1b[48;2;1;1;1m\x1b[38;2;1;1;1m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;224;224;224m▄\x1b[0m\x1b[48;2;158;155;156m\x1b[38;2;224;224;224m▄\x1b[0m\x1b[48;2;254;254;254m\x1b[38;2;158;155;156m▄\x1b[0m\x1b[48;2;158;155;156m\x1b[38;2;224;224;224m▄\x1b[0m\x1b[48;2;224;224;224m\x1b[38;2;158;155;156m▄\x1b[0m\x1b[48;2;1;1;1m\x1b[38;2;0;0;0m▄\x1b[0m    \n  \x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m\x1b[38;2;0;0;0m▀\x1b[0m              \x1b[38;2;1;1;1m▀\x1b[0m\x1b[38;2;1;1;1m▀\x1b[0m\x1b[38;2;1;1;1m▀\x1b[0m\x1b[38;2;1;1;1m▀\x1b[0m\x1b[38;2;1;1;1m▀\x1b[0m     ";
