use clap::{Arg, Command as CommandClap};
use core::str;
use std::process::Command;

fn main() -> Result<(), String> {
    let matches = CommandClap::new("Mac Changer")
        .about("Generating random mac address.")
        .arg(
            Arg::new("interface")
                .help("Input your Interface")
                .required(true),
        )
        .get_matches();

    let interface = matches.get_one::<String>("interface").unwrap();

    let random_mc = generate_random_mac_address();

    // Function to handle command output and error reporting
    fn run_command(cmd: &mut Command, err_msg: &str) -> Result<(), String> {
        let output = cmd.output().map_err(|e| format!("{}: {}", err_msg, e))?;

        // Check if the exit status is successful
        if !output.status.success() {
            return Err(format!(
                "{}: {}",
                err_msg,
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    let current_mac = Command::new("ifconfig")
        .arg(interface)
        .output()
        .expect("Cannot find interface");

    println!(
        "\nYour current mac:\n{}",
        str::from_utf8(&current_mac.stdout).expect("Failed to read stdout")
    );

    // Bring down the interface
    run_command(
        Command::new("sudo")
            .arg("ifconfig")
            .arg(interface)
            .arg("down"),
        "Failed to bring down interface",
    )?;

    // Change MAC address
    run_command(
        Command::new("sudo")
            .arg("ifconfig")
            .arg(interface)
            .arg("hw")
            .arg("ether")
            .arg(&random_mc),
        "Failed to change MAC address",
    )?;

    run_command(
        Command::new("sudo")
            .arg("ifconfig")
            .arg(interface)
            .arg("up"),
        "Failed to bring up interface",
    )?;

    println!("{} mac address changed to: {}", &interface, &random_mc);

    let the_new_mac = Command::new("ifconfig")
        .arg(interface)
        .output()
        .expect("Cannot find interface");

    println!(
        "{}",
        str::from_utf8(&the_new_mac.stdout).expect("Failed to read stdout")
    );
    Ok(())
}

fn generate_random_mac_address() -> String {
    format!(
        "02:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>()
    )
}
