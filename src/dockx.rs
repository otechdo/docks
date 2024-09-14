use std::collections::HashMap;
use std::env::{set_current_dir, var};
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use std::process::{Command, ExitCode};
use toml::Value;

fn docker(verb: &str, args: &[&str], path: &str) -> Result<(), Error> {
    if let Ok(mut child) = Command::new("docker")
        .arg(verb)
        .args(args)
        .current_dir(path)
        .spawn()
    {
        if let Ok(status) = child.wait() {
            if status.success() {
                return Ok(());
            }
            return Err(Error::new(
                ErrorKind::Other,
                "Docker exited with status no 0",
            ));
        }
    }
    Err(Error::new(ErrorKind::NotFound, "docker not found"))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("dockx");
        return ExitCode::FAILURE;
    }
    if args[1] == "--version" || args[1] == "-v"
    {
        println!("dockx {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    if args[1] == "-h" || args[1] == "--help" {
        return ExitCode::SUCCESS;
    }
    if args[1] == "--publish" {
        return publish();
    }
    ExitCode::FAILURE
}

fn publish() -> ExitCode {
    set_current_dir(var("DOCKX_ROOT").expect("DOCKX_ROOT not set").as_str()).expect("failed to checkout on directory");

    let h = read_to_string("hub/hub.toml").expect("No hub.toml");

    let config: Value = toml::from_str(h.as_str()).expect("Invalid hub.toml");
    let mut tg: HashMap<String, Vec<String>> = HashMap::new();
    let conf = config.as_table().expect("Invalid config");
    let hub = conf.get("hub").expect("Missing hub").as_table().expect("Invalid config");
    let username = hub.get("username").expect("Missing username").as_str().expect("Invalid config");
    let images = hub.get("images").expect("Missing images").as_array().expect("Invalid config");
    let tags = hub.get("tags").expect("Missing tags").as_table().expect("Invalid config");
    for (name, tags) in tags {
        tg.insert(name.to_string(), tags.as_array().expect("Invalid config").into_iter().map(|v| v.as_str().expect("Invalid config").to_string()).collect::<Vec<String>>());
    }
    for image in images {
        let name = image.get("name").expect("Missing name").as_str().expect("Invalid config").to_string();
        let path = image.get("path").expect("Missing path").as_str().expect("Invalid config").to_string();
        let mut images_tags: Vec<String> = Vec::new();
        let tags = image.get("tags").expect("Missing tags").as_array().expect("Invalid config");
        for tag in tags {
            let x = tag.as_str().expect("Invalid config").to_string();
            let t = tg.get(&x).expect("invalid config");
            for current in t {
                images_tags.push(current.to_string());
            }
        }
        for images_tag in &images_tags {
            docker("buildx", &["build", "-t", format!("{username}/{name}:{images_tag}").as_str()], &path.as_str()).expect("docker buildx");
            docker("push", &[format!("{username}/{name}:{images_tag}").as_str()], &path.as_str()).expect("docker push");
        }

        images_tags.clear();
    }
    dbg!(images);
    ExitCode::SUCCESS
}
