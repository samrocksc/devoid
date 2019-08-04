pub mod decompose {
    use std::process::Command;
    pub fn run_image(image_name: String) {
        println!("testing run_image {}", image_name);
        Command::new("sh")
            .arg("-c")
            .arg(image_name)
            .spawn()
            .expect("failed to execute process");
    }
}

pub mod checks {
    pub fn for_os(image_name: String) {
        if cfg!(target_os = "windows") {
            return println!("pffthhhhh windows");
        } else {
            for_image(image_name);
        }
    }

    // Checks for the image supplied from the command line flag
    pub fn for_image(image_name: String) {
        use super::decompose::run_image;
        match String::from(image_name).as_ref() {
            "blue" => println!("blue"),
            "postgres" => {
                println!("Starting Postgres");
                run_image(String::from("docker-compose run --service-ports postgres"));
            }
            "mysql56" => {
                println!("Starting MySQL5.6");
                run_image(String::from("docker-compose run --service-ports mysql56"));
            }
            "stop" => run_image(String::from("docker-compose down")),
            _ => println!("It's nothing"),
        }
    }
}
