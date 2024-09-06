use std::{
    env::args,
    fs::read_to_string,
    io::{Error, ErrorKind},
    process::Command,
};

use inquire::Text;
use toml::Value;

fn login() -> Result<(), Error> {
    if let Ok(username) = Text::new("Please enter your docker username : ")
        .with_default(env!("USER"))
        .prompt()
    {
        if let Ok(mut child) = Command::new("docker")
            .arg("login")
            .arg("-u")
            .arg(username.as_str())
            .spawn()
        {
            if child.wait().is_ok() {
                return Ok(());
            } else {
                return Err(Error::last_os_error());
            }
        }
    }
    Err(Error::last_os_error())
}

fn logout() -> Result<(), Error> {
    if let Ok(mut child) = Command::new("docker").arg("logout").spawn() {
        if child.wait().is_ok() {
            return Ok(());
        } else {
            return Err(Error::last_os_error());
        }
    }
    Err(Error::last_os_error())
}

fn deploy() -> Result<(), Error> {
    if let Ok(config) = read_to_string("tux.toml") {
        if let Ok(tux) = toml::from_str::<Value>(config.as_str()) {
            if let Ok(server) = Text::new("Please enter the server name to deploy : ").prompt() {
                if let Some(ip) = tux.get(server.as_str()).and_then(|lab| lab.get("ip")) {
                    if let Some(services) =
                        tux.get(server.as_str()).and_then(|lab| lab.get("services"))
                    {
                        if let Some(to_deploy) = services.as_array() {
                            for service in to_deploy {
                                if let Some(address) = ip.as_str() {
                                    if let Some(x) = service.as_str() {
                                        if let Ok(mut config) = Command::new("docker")
                                            .arg("compose")
                                            .arg("config")
                                            .current_dir(format!("services/{x}").as_str())
                                            .spawn()
                                        {
                                            if config.wait().is_err() {
                                                println!("{x} skipped compose.yaml no valid");
                                                continue;
                                            }
                                        }

                                        if let Ok(mut build) = Command::new("docker")
                                            .arg("compose")
                                            .arg("build")
                                            .arg("--no-cache")
                                            .current_dir(format!("services/{x}").as_str())
                                            .spawn()
                                        {
                                            if build.wait().is_err() {
                                                println!("{x} skipped compose.yaml no valid");
                                                continue;
                                            }
                                        }

                                        if let Ok(mut pull) = Command::new("docker")
                                            .arg("compose")
                                            .arg("pull")
                                            .current_dir(format!("services/{x}").as_str())
                                            .spawn()
                                        {
                                            if let Ok(code) = pull.wait() {
                                                if code.success().eq(&false) {
                                                    continue;
                                                }
                                            }
                                        }

                                        if let Ok(mut rsync) = Command::new("rsync")
                                            .arg("-a")
                                            .arg("-z")
                                            .arg("-e")
                                            .arg("ssh")
                                            .arg(format!("./services/{x}/"))
                                            .arg(format!("root@{address}:{x}").as_str())
                                            .spawn()
                                        {
                                            if rsync.wait().is_ok() {
                                                if let Ok(mut docker) = Command::new("ssh")
                                                            .arg(format!("root@{address}").as_str())
                                                            .arg(format!("docker compose --project-directory {x} up -d").as_str())                                                        .spawn()
                                                        {
                                                            if docker.wait().is_ok() {
                                                                println!("{}",format!("service {x} started successfully").as_str()); 
                                                            } else {
                                                                println!(
                                                                    "{}",
                                                                    format!("failed to start {x}")
                                                                        .as_str()
                                                                );
                                                            }
                                                        }
                                            } else {
                                                println!(
                                                    "{}",
                                                    format!("failed to deploy {x}").as_str()
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            return Ok(());
                        }
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Failed to parse services",
                        ));
                    }
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Failed to get server services",
                    ));
                }
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Failed to get server address",
                ));
            }
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to get server name",
            ));
        }
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to parse the config",
        ));
    }
    Err(Error::last_os_error())
}

fn help() -> Result<(), Error> {
    println!("tux [ login|logout|deploy]");
    Ok(())
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
            }
        }
    };
    help()
}
