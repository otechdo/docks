use std::{env::args, io::Error, process::Command};

use inquire::Text;

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

fn help() -> Result<(), Error> {
    println!("tux [ login|logout]");
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
            }
        }
    };
    help()
}
