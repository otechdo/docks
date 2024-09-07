use inquire::Text;
use std::{
    env::args,
    fs::read_to_string,
    io::{Error, ErrorKind},
    process::Command,
};
use toml::Value;

fn docker(verb: &str, args: &[&str], path: &str) -> Result<(), Error> {
    if let Ok(mut child) = Command::new("docker")
        .arg(verb)
        .args(args)
        .current_dir(path)
        .spawn()
    {
        return match child.wait() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
    Err(Error::new(ErrorKind::NotFound, "docker not found"))
}
fn ssh_run(args: &[&str], user: &str, ip: &str) -> Result<(), Error> {
    if let Ok(mut cmd) = Command::new("ssh")
        .arg(format!("{user}@{ip}").as_str())
        .args(args)
        .spawn()
    {
        return match cmd.wait() {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        };
    }
    Err(Error::new(ErrorKind::NotFound, "ssh not found"))
}

fn upload_image(user: &str, ip: &str, s: &str) -> Result<(), Error> {
    println!("\x1b[1;32m    Tux\x1b[1;37m Sending {s} image to the server\x1b[0m");
    if let Ok(mut cmd) = Command::new("rsync")
        .arg("-a")
        .arg("-z")
        .arg("-e")
        .arg("ssh")
        .arg(format!("./services/{s}/").as_str())
        .arg(format!("{user}@{ip}:{s}").as_str())
        .spawn()
    {
        return match cmd.wait() {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        };
    }
    Err(Error::new(ErrorKind::NotFound, "rsync not found"))
}

fn login() -> Result<(), Error> {
    let username = Text::new("Please enter your docker username : ")
        .with_default(env!("USER"))
        .prompt()
        .unwrap_or_default();
    docker("login", &["-u", username.as_str()], "/tmp")
}

fn logout() -> Result<(), Error> {
    docker("logout", &[], "/tmp")
}

fn servers() -> Result<Vec<String>, Error> {
    let mut servers: Vec<String> = Vec::new();
    if let Ok(config) = configuration() {
        if let Some(tables) = config.as_table() {
            for (server_name, _) in tables {
                servers.push(server_name.to_string());
            }
            return Ok(servers);
        }
        return Err(Error::new(ErrorKind::InvalidData, "must be a table"));
    }
    Err(Error::new(ErrorKind::NotFound, "Missing configuration"))
}
fn configuration() -> Result<Value, Error> {
    if let Ok(conf) = read_to_string("tux.toml") {
        if let Ok(config) = toml::from_str::<Value>(&conf) {
            return Ok(config);
        }
        return Err(Error::last_os_error());
    }
    Err(Error::last_os_error())
}

fn running(user: &str, ip: &str) -> Result<(), Error>
{
    assert!(ssh_run(
        &[
            "docker",
            "ps",
        ],
        user,
        ip
    )
        .is_ok());
    Ok(())
}
fn deploy() -> Result<(), Error> {
    let tux = configuration()?;
    let servers: Vec<String> = servers()?;
    for server in servers {
        if let Some(username) = tux
            .get(server.as_str())
            .and_then(|value: &Value| value.get("username"))
        {
            if let Some(ip) = tux
                .get(server.as_str())
                .and_then(|value: &Value| value.get("ip"))
            {
                if let Some(services) = tux
                    .get(server.as_str())
                    .and_then(|value: &Value| value.get("services"))
                {
                    if let Some(images) = services.as_array() {
                        for image in images {
                            println!("\x1b[1;32m    Tux\x1b[1;37m Upload {} on {}\x1b[0m", image.as_str().unwrap_or_default(), ip.as_str().unwrap_or_default(), );
                            assert!(upload_image(
                                username.as_str().unwrap_or_default(),
                                ip.as_str().unwrap_or_default(),
                                image.as_str().unwrap_or_default()
                            )
                                .is_ok());
                            println!("\x1b[1;32m    Tux\x1b[1;37m {} uploded successfully\x1b[0m", image.as_str().unwrap_or_default());
                            println!("\x1b[1;32m    Tux\x1b[1;37m Stop {} container before upgrade\x1b[0m", image.as_str().unwrap_or_default());
                            assert!(ssh_run(
                                &[
                                    "docker",
                                    "compose",
                                    "--project-directory",
                                    image.as_str().unwrap_or_default(),
                                    "down",
                                ],
                                username.as_str().unwrap_or_default(),
                                ip.as_str().unwrap_or_default(),
                            )
                                .is_ok());
                            println!("\x1b[1;32m    Tux\x1b[1;37m {} container stoped successfully\x1b[0m", image.as_str().unwrap_or_default());
                            println!("\x1b[1;32m    Tux\x1b[1;37m Start update of the {} container\x1b[0m", image.as_str().unwrap_or_default());
                            assert!(ssh_run(
                                &[
                                    "docker",
                                    "compose",
                                    "--project-directory",
                                    image.as_str().unwrap_or_default(),
                                    "pull",
                                ],
                                username.as_str().unwrap_or_default(),
                                ip.as_str().unwrap_or_default()
                            )
                                .is_ok());
                            println!("\x1b[1;32m    Tux\x1b[1;37m The {} container has been updated successfully\x1b[0m", image.as_str().unwrap_or_default());
                            println!("\x1b[1;32m    Tux\x1b[1;37m Start the {} container\x1b[0m", image.as_str().unwrap_or_default());

                            assert!(ssh_run(
                                &[
                                    "docker",
                                    "compose",
                                    "--project-directory",
                                    image.as_str().unwrap_or_default(),
                                    "up",
                                    "-d",
                                ],
                                username.as_str().unwrap_or_default(),
                                ip.as_str().unwrap_or_default(),
                            )
                                .is_ok());
                            println!("\x1b[1;32m    Tux\x1b[1;37m The container {} is now uploded on the {} server\x1b[0m", image.as_str().unwrap_or_default(), ip.as_str().unwrap_or_default());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn help() {
    println!("tux [ login|logout|deploy ]");
}
fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if args.len().eq(&2) {
        if let Some(x) = args.get(1) {
            if x.eq("login") {
                return login();
            } else if x.eq("logout") {
                return logout();
            } else if x.eq("deploy") {
                return deploy();
            } else if x.eq("running") {
                let tux = configuration()?;
                let servers = servers()?;
                for server in &servers {
                    if let Some(username) = tux
                        .get(server.as_str())
                        .and_then(|value: &Value| value.get("username"))
                    {
                        if let Some(ip) = tux
                            .get(server.as_str())
                            .and_then(|value: &Value| value.get("ip"))
                        {
                            return running(username.as_str().unwrap_or_default(), ip.as_str().unwrap_or_default());
                        }
                    }
                }
            }
        }
    };
    help();
    Ok(())
}
