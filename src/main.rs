//@Author: pqqou
//Version: 1.0
//Date: 01.11.2024

use std::io;

fn m_cm(command: String) {
    if command == "m into cm" {
        //if the command is m into cm
        println!("Enter the amount of meters you would like to convert into centimeters:");
        let mut m = String::new();
        io::stdin().read_line(&mut m).expect("Failed to read line");
        let m: f64 = m.trim().parse().expect("Please type a number!");
        let cm = m * 100.0;
        println!("{} meters is equal to {} centimeters", m, cm);
    } else {
        //if the command is cm into m
        println!("Enter the amount of centimeters you would like to convert to meters:");
        let mut cm = String::new();
        io::stdin().read_line(&mut cm).expect("Failed to read line");
        let cm: f64 = cm.trim().parse().expect("Please type a number!");
        let m = cm / 100.0;
        println!("{} centimeters is equal to {} meters", cm, m);
    }
}

fn m_dm(command: String) {
    if command == "m into dm" {
        //if the command is m into dm
        println!("Enter the amount of meters you would like to convert into decimeters:");
        let mut m = String::new();
        io::stdin().read_line(&mut m).expect("Failed to read line");
        let m: f64 = m.trim().parse().expect("Please type a number!");
        let dm = m * 10.0;
        println!("{} meters is equal to {} decimeters", m, dm);
    } else {
        //if the command is dm into m
        println!("Enter the amount of decimeters you would like to convert to meters:");
        let mut dm = String::new();
        io::stdin().read_line(&mut dm).expect("Failed to read line");
        let dm: f64 = dm.trim().parse().expect("Please type a number!");
        let m = dm / 10.0;
        println!("{} decimeters is equal to {} meters", dm, m);
    }
}

fn m_km(command: String) {
    if command == "m into km" {
        //if the command is m into km
        println!("Enter the amount of meters you would like to convert into kilometers:");
        let mut m = String::new();
        io::stdin().read_line(&mut m).expect("Failed to read line");
        let m: f64 = m.trim().parse().expect("Please type a number!");
        let km = m / 1000.0;
        println!("{} meters is equal to {} kilometers", m, km);
    } else {
        //if the command is km into m
        println!("Enter the amount of kilometers you would like to convert to meters:");
        let mut km = String::new();
        io::stdin().read_line(&mut km).expect("Failed to read line");
        let km: f64 = km.trim().parse().expect("Please type a number!");
        let m = km * 1000.0;
        println!("{} kilometers is equal to {} meters", km, m);
    }
}

fn m_mm(command: String) {
    if command == "m into mm" {
        //if the command is m into mm
        println!("Enter the amount of meters you would like to convert into millimeters:");
        let mut m = String::new();
        io::stdin().read_line(&mut m).expect("Failed to read line");
        let m: f64 = m.trim().parse().expect("Please type a number!");
        let mm = m * 1000.0;
        println!("{} meters is equal to {} millimeters", m, mm);
    } else {
        //if the command is mm into m
        println!("Enter the amount of millimeters you would like to convert to meters:");
        let mut mm = String::new();
        io::stdin().read_line(&mut mm).expect("Failed to read line");
        let mm: f64 = mm.trim().parse().expect("Please type a number!");
        let m = mm / 1000.0;
        println!("{} millimeters is equal to {} meters", mm, m);
    }
}

fn mm_cm(command: String) {
    if command == "mm into cm" {
        //if the command is mm into cm
        println!("Enter the amount of millimeters you would like to convert into centimeters:");
        let mut mm = String::new();
        io::stdin().read_line(&mut mm).expect("Failed to read line");
        let mm: f64 = mm.trim().parse().expect("Please type a number!");
        let cm = mm / 10.0;
        println!("{} millimeters is equal to {} centimeters", mm, cm);
    } else {
        //if the command is cm into mm
        println!("Enter the amount of centimeters you would like to convert to millimeters:");
        let mut cm = String::new();
        io::stdin().read_line(&mut cm).expect("Failed to read line");
        let cm: f64 = cm.trim().parse().expect("Please type a number!");
        let mm = cm * 10.0;
        println!("{} centimeters is equal to {} millimeters", cm, mm);
    }
}

fn mm_dm(command: String) {
    if command == "mm into dm" {
        //if the command is mm into dm
        println!("Enter the amount of millimeters you would like to convert into decimeters:");
        let mut mm = String::new();
        io::stdin().read_line(&mut mm).expect("Failed to read line");
        let mm: f64 = mm.trim().parse().expect("Please type a number!");
        let dm = mm / 100.0;
        println!("{} millimeters is equal to {} decimeters", mm, dm);
    } else {
        //if the command is dm into mm
        println!("Enter the amount of decimeters you would like to convert to millimeters:");
        let mut dm = String::new();
        io::stdin().read_line(&mut dm).expect("Failed to read line");
        let dm: f64 = dm.trim().parse().expect("Please type a number!");
        let mm = dm * 100.0;
        println!("{} decimeters is equal to {} millimeters", dm, mm);
    }
}

fn mm_km(command: String) {
    if command == "mm into km" {
        //if the command is mm into km
        println!("Enter the amount of millimeters you would like to convert into kilometers:");
        let mut mm = String::new();
        io::stdin().read_line(&mut mm).expect("Failed to read line");
        let mm: f64 = mm.trim().parse().expect("Please type a number!");
        let km = mm / 1000000.0;
        println!("{} millimeters is equal to {} kilometers", mm, km);
    } else {
        //if the command is km into mm
        println!("Enter the amount of kilometers you would like to convert to millimeters:");
        let mut km = String::new();
        io::stdin().read_line(&mut km).expect("Failed to read line");
        let km: f64 = km.trim().parse().expect("Please type a number!");
        let mm = km * 1000000.0;
        println!("{} kilometers is equal to {} millimeters", km, mm);
    }
}

fn cm_dm(command: String) {
    if command == "cm into dm" {
        //if the command is cm into dm
        println!("Enter the amount of centimeters you would like to convert into decimeters:");
        let mut cm = String::new();
        io::stdin().read_line(&mut cm).expect("Failed to read line");
        let cm: f64 = cm.trim().parse().expect("Please type a number!");
        let dm = cm / 10.0;
        println!("{} centimeters is equal to {} decimeters", cm, dm);
    } else {
        //if the command is dm into cm
        println!("Enter the amount of decimeters you would like to convert to centimeters:");
        let mut dm = String::new();
        io::stdin().read_line(&mut dm).expect("Failed to read line");
        let dm: f64 = dm.trim().parse().expect("Please type a number!");
        let cm = dm * 10.0;
        println!("{} decimeters is equal to {} centimeters", dm, cm);
    }
}

fn cm_km(command: String) {
    if command == "cm into km" {
        //if the command is cm into km
        println!("Enter the amount of centimeters you would like to convert into kilometers:");
        let mut cm = String::new();
        io::stdin().read_line(&mut cm).expect("Failed to read line");
        let cm: f64 = cm.trim().parse().expect("Please type a number!");
        let km = cm / 100000.0;
        println!("{} centimeters is equal to {} kilometers", cm, km);
    } else {
        //if the command is km into cm
        println!("Enter the amount of kilometers you would like to convert to centimeters:");
        let mut km = String::new();
        io::stdin().read_line(&mut km).expect("Failed to read line");
        let km: f64 = km.trim().parse().expect("Please type a number!");
        let cm = km * 100000.0;
        println!("{} kilometers is equal to {} centimeters", km, cm);
    }
}

fn dm_km(command: String) {
    if command == "dm into km" {
        //if the command is dm into km
        println!("Enter the amount of decimeters you would like to convert into kilometers:");
        let mut dm = String::new();
        io::stdin().read_line(&mut dm).expect("Failed to read line");
        let dm: f64 = dm.trim().parse().expect("Please type a number!");
        let km = dm / 10000.0;
        println!("{} decimeters is equal to {} kilometers", dm, km);
    } else {
        //if the command is km into dm
        println!("Enter the amount of kilometers you would like to convert to decimeters:");
        let mut km = String::new();
        io::stdin().read_line(&mut km).expect("Failed to read line");
        let km: f64 = km.trim().parse().expect("Please type a number!");
        let dm = km * 10000.0;
        println!("{} kilometers is equal to {} decimeters", km, dm);
    }
}

fn mm_inch(command: String) {
    if command == "mm into inch" {
        //if the command is mm into inch
        println!("Enter the amount of millimeters you would like to convert into inches:");
        let mut mm = String::new();
        io::stdin().read_line(&mut mm).expect("Failed to read line");
        let mm: f64 = mm.trim().parse().expect("Please type a number!");
        let inch = mm / 25.4;
        println!("{} millimeters is equal to {} inches", mm, inch);
    } else {
        //if the command is inch into mm
        println!("Enter the amount of inches you would like to convert to millimeters:");
        let mut inch = String::new();
        io::stdin()
            .read_line(&mut inch)
            .expect("Failed to read line");
        let inch: f64 = inch.trim().parse().expect("Please type a number!");
        let mm = inch * 25.4;
        println!("{} inches is equal to {} millimeters", inch, mm);
    }
}

fn cm_inch(command: String) {
    if command == "cm into inch" {
        //if the command is cm into inch
        println!("Enter the amount of centimeters you would like to convert into inches:");
        let mut cm = String::new();
        io::stdin().read_line(&mut cm).expect("Failed to read line");
        let cm: f64 = cm.trim().parse().expect("Please type a number!");
        let inch = cm / 2.54;
        println!("{} centimeters is equal to {} inches", cm, inch);
    } else {
        //if the command is inch into cm
        println!("Enter the amount of inches you would like to convert to centimeters:");
        let mut inch = String::new();
        io::stdin()
            .read_line(&mut inch)
            .expect("Failed to read line");
        let inch: f64 = inch.trim().parse().expect("Please type a number!");
        let cm = inch * 2.54;
        println!("{} inches is equal to {} centimeters", inch, cm);
    }
}

fn dm_inch(command: String) {
    if command == "dm into inch" {
        //if the command is dm into inch
        println!("Enter the amount of decimeters you would like to convert into inches:");
        let mut dm = String::new();
        io::stdin().read_line(&mut dm).expect("Failed to read line");
        let dm: f64 = dm.trim().parse().expect("Please type a number!");
        let inch = dm / 0.254;
        println!("{} decimeters is equal to {} inches", dm, inch);
    } else {
        //if the command is inch into dm
        println!("Enter the amount of inches you would like to convert to decimeters:");
        let mut inch = String::new();
        io::stdin()
            .read_line(&mut inch)
            .expect("Failed to read line");
        let inch: f64 = inch.trim().parse().expect("Please type a number!");
        let dm = inch * 0.254;
        println!("{} inches is equal to {} decimeters", inch, dm);
    }
}

fn m_inch(command: String) {
    if command == "m into inch" {
        //if the command is m into inch
        println!("Enter the amount of meters you would like to convert into inches:");
        let mut m = String::new();
        io::stdin().read_line(&mut m).expect("Failed to read line");
        let m: f64 = m.trim().parse().expect("Please type a number!");
        let inch = m / 0.0254;
        println!("{} meters is equal to {} inches", m, inch);
    } else {
        //if the command is inch into m
        println!("Enter the amount of inches you would like to convert to meters:");
        let mut inch = String::new();
        io::stdin()
            .read_line(&mut inch)
            .expect("Failed to read line");
        let inch: f64 = inch.trim().parse().expect("Please type a number!");
        let m = inch * 0.0254;
        println!("{} inches is equal to {} meters", inch, m);
    }
}

fn km_inch(command: String) {
    if command == "km into inch" {
        //if the command is m into inch
        println!("Enter the amount of kilometers you would like to convert into inches:");
        let mut km = String::new();
        io::stdin().read_line(&mut km).expect("Failed to read line");
        let km: f64 = km.trim().parse().expect("Please type a number!");
        let inch = km / 0.0000254;
        println!("{} kilometers is equal to {} inches", km, inch);
    } else {
        //if the command is inch into km
        println!("Enter the amount of inches you would like to convert to kilometers:");
        let mut inch = String::new();
        io::stdin()
            .read_line(&mut inch)
            .expect("Failed to read line");
        let inch: f64 = inch.trim().parse().expect("Please type a number!");
        let km = inch * 0.0000254;
        println!("{} inches is equal to {} kilometers", inch, km);
    }
}

fn mm_foot(command: String) {
    if command == "mm into feet" {
        println!("Enter the amount of millimeters you would like to convert into feet:");
        let mut mm = String::new();
        io::stdin().read_line(&mut mm).expect("Failed to read line");
        let mm: f64 = mm.trim().parse().expect("Please type a number!");
        let foot = mm / 304.8;
        println!("{} millimeters is equal to {} feet", mm, foot);
    } else {
        println!("Enter the amount of feet you would like to convert to millimeters:");
        let mut foot = String::new();
        io::stdin()
            .read_line(&mut foot)
            .expect("Failed to read line");
        let foot: f64 = foot.trim().parse().expect("Please type a number!");
        let mm = foot * 304.8;
        println!("{} feet is equal to {} millimeters", foot, mm);
    }
}

fn cm_foot(command: String) {
    if command == "cm into feet" {
        println!("Enter the amount of centimeters you would like to convert into feet:");
        let mut cm = String::new();
        io::stdin().read_line(&mut cm).expect("Failed to read line");
        let cm: f64 = cm.trim().parse().expect("Please type a number!");
        let foot = cm / 30.48;
        println!("{} centimeters is equal to {} feet", cm, foot);
    } else {
        println!("Enter the amount of feet you would like to convert to centimeters:");
        let mut foot = String::new();
        io::stdin()
            .read_line(&mut foot)
            .expect("Failed to read line");
        let foot: f64 = foot.trim().parse().expect("Please type a number!");
        let cm = foot * 30.48;
        println!("{} feet is equal to {} centimeters", foot, cm);
    }
}

fn dm_foot(command: String) {
    if command == "dm into feet" {
        println!("Enter the amount of decimeters you would like to convert into feet:");
        let mut dm = String::new();
        io::stdin().read_line(&mut dm).expect("Failed to read line");
        let dm: f64 = dm.trim().parse().expect("Please type a number!");
        let foot = dm / 3.048;
        println!("{} decimeters is equal to {} feet", dm, foot);
    } else {
        println!("Enter the amount of feet you would like to convert to decimeters:");
        let mut foot = String::new();
        io::stdin()
            .read_line(&mut foot)
            .expect("Failed to read line");
        let foot: f64 = foot.trim().parse().expect("Please type a number!");
        let dm = foot * 3.048;
        println!("{} feet is equal to {} decimeters", foot, dm);
    }
}

fn m_foot(command: String) {
    if command == "m into feet" {
        println!("Enter the amount of meters you would like to convert into feet:");
        let mut m = String::new();
        io::stdin().read_line(&mut m).expect("Failed to read line");
        let m: f64 = m.trim().parse().expect("Please type a number!");
        let foot = m / 0.3048;
        println!("{} meters is equal to {} feet", m, foot);
    } else {
        println!("Enter the amount of feet you would like to convert to meters:");
        let mut foot = String::new();
        io::stdin()
            .read_line(&mut foot)
            .expect("Failed to read line");
        let foot: f64 = foot.trim().parse().expect("Please type a number!");
        let m = foot * 0.3048;
        println!("{} feet is equal to {} meters", foot, m);
    }
}

fn km_foot(command: String) {
    if command == "km into feet" {
        println!("Enter the amount of kilometers you would like to convert into feet:");
        let mut km = String::new();
        io::stdin().read_line(&mut km).expect("Failed to read line");
        let km: f64 = km.trim().parse().expect("Please type a number!");
        let foot = km / 0.0003048;
        println!("{} kilometers is equal to {} feet", km, foot);
    } else {
        println!("Enter the amount of feet you would like to convert to kilometers:");
        let mut foot = String::new();
        io::stdin()
            .read_line(&mut foot)
            .expect("Failed to read line");
        let foot: f64 = foot.trim().parse().expect("Please type a number!");
        let km = foot * 0.0003048;
        println!("{} feet is equal to {} kilometers", foot., km);
    }
}

fn inch_foot(command: String) {
    if command == "inch into feet" {
        println!("Enter the amount of inches you would like to convert into feet:");
        let mut inch = String::new();
        io::stdin()
            .read_line(&mut inch)
            .expect("Failed to read line");
        let inch: f64 = inch.trim().parse().expect("Please type a number!");
        let foot = inch / 12.0;
        println!("{} inches is equal to {} feet", inch, foot);
    } else {
        println!("Enter the amount of feet you would like to convert to inches:");
        let mut foot = String::new();
        io::stdin()
            .read_line(&mut foot)
            .expect("Failed to read line");
        let foot: f64 = foot.trim().parse().expect("Please type a number!");
        let inch = foot * 12.0;
        println!("{} feet is equal to {} inches", foot, inch);
    }
}

fn matchCommands(command: String, vec: &mut Vec<String>) {
    let command = command.trim();
    match command {
        "m into cm" => m_cm(String::from(command)),
        "cm into m" => m_cm(String::from(command)),

        "dm into m" => m_dm(String::from(command)),
        "m into dm" => m_dm(String::from(command)),

        "km into m" => m_km(String::from(command)),
        "m into km" => m_km(String::from(command)),

        "mm into m" => m_mm(String::from(command)),
        "m into mm" => m_mm(String::from(command)),

        "mm into cm" => mm_cm(String::from(command)),
        "cm into mm" => mm_cm(String::from(command)),

        "mm into dm" => mm_dm(String::from(command)),
        "dm into mm" => mm_dm(String::from(command)),

        "mm into km" => mm_km(String::from(command)),
        "km into mm" => mm_km(String::from(command)),

        "cm into dm" => cm_dm(String::from(command)),
        "dm into cm" => cm_dm(String::from(command)),

        "cm into km" => cm_km(String::from(command)),
        "km into cm" => cm_km(String::from(command)),

        "dm into km" => dm_km(String::from(command)),
        "km into dm" => dm_km(String::from(command)),

        "mm into inches" => mm_inch(String::from(command)),
        "inches into mm" => mm_inch(String::from(command)),

        "cm into inches" => cm_inch(String::from(command)),
        "inches into cm" => cm_inch(String::from(command)),

        "dm into inches" => dm_inch(String::from(command)),
        "inches into dm" => dm_inch(String::from(command)),

        "m into inches" => m_inch(String::from(command)),
        "inches into m" => m_inch(String::from(command)),

        "km into inches" => km_inch(String::from(command)),
        "inches into km" => km_inch(String::from(command)),

        "mm into feet" => mm_foot(String::from(command)),
        "feet into mm" => mm_foot(String::from(command)),

        "cm into feet" => cm_foot(String::from(command)),
        "feet into cm" => cm_foot(String::from(command)),

        "dm into feet" => dm_foot(String::from(command)),
        "feet into dm" => dm_foot(String::from(command)),

        "m into feet" => m_foot(String::from(command)),
        "feet into m" => m_foot(String::from(command)),

        "km into feet" => km_foot(String::from(command)),
        "feet into km" => km_foot(String::from(command)),

        "inch into feet" => inch_foot(String::from(command)),
        "feet into inch" => inch_foot(String::from(command)),

        "help" => help(),
        "quit" => quit(),
        _ => println!("---Invalid command!---"),
    }
}

fn help() {
    println!("Type: '<given> into <convertet>'");
    println!();
    println!("Please type decimal numbers with a dot as a separator!");
    println!();
}

fn quit() {
    println!("Thanks for using this unit converter!");
    println!();
    println!("Goodbye!");
    std::process::exit(0);
}

fn main() {
    // Define the main function
    println!("Welcome to the Metric Converter!");
    println!();
    println!("You can convert the following units: Millimeters, Centimeters, Decimeters, Meters, Kilometers");
    println!();
    println!("Important: Please type decimal numbers with an dot as the separator!");
    println!();
    println!("---Type help for commandinstructions---");
    println!();
    let mut vec: Vec<String> = Vec::new();
    loop {
        let mut command = String::new();
        println!("Enter command:");
        println!();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        matchCommands(command, &mut vec);
    }
}
