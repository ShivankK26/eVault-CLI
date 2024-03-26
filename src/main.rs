mod db;
use db::*;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let conn = init_database().expect("Failed to initialize the database");
    clr();

    let ascii = r#"
                                                     ___      ___ ________  ___  ___  ___   _________
                                      ____          |\  \    /  /|\   __  \|\  \|\  \|\  \ |\___   ___\
                                     |  __|         \ \  \  /  / | \  \|\  \ \  \\\  \ \  \\|___ \  \_|
                                     | |__           \ \  \/  / / \ \   __  \ \  \\\  \ \  \    \ \  \
                                     |  _|            \ \    / /   \ \  \ \  \ \  \\\  \ \  \____\ \  \
                                     | |__             \ \__/ /     \ \__\ \__\ \_______\ \_______\ \__\
                                     |____|             \|__|/       \|__|\|__|\|_______|\|_______|\|__|


        "#;
    println!("{ascii}");

    loop {
        println!("eVault Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                write_password_to_db(&conn, &entry.service, &entry.username, &entry.password)
                    .expect("Failed to write to the database");
                println!("Entry added successfully.");
            }
            "2" => {
                clr();
                let services = read_passwords_from_db(&conn).unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
    - Username : {}
    - Password : {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let search = prompt("Search by service name:");
                match search_service_by_name(&conn, &search) {
                    Ok(Some(entry)) => {
                        println!(
                            "Service = {}
                - Username : {}
                - Password : {:?}",
                            entry.service, entry.username, entry.password
                        );
                    }
                    Ok(None) => {
                        println!("Service not found.");
                    }
                    Err(err) => {
                        eprintln!("Error searching for service: {}", err);
                    }
                }
            }

            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
        println!("\n\n");
    }
}
