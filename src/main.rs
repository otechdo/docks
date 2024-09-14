use chrono::Local;
use inquire::{Confirm, Select, Text};
use is_executable::IsExecutable;
use std::collections::HashMap;
use std::env::{current_dir, set_current_dir, var};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR_STR};
use std::process::{ExitStatus, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{
    fs::read_to_string,
    io,
    io::{Error, ErrorKind},
    process::Command,
};
use toml::Value;
pub const TASKS: [&str; 30] = [
    "init",
    "build",
    "clear",
    "check",
    "cd",
    "commit",
    "os",
    "deploy",
    "enter",
    "exit",
    "edit",
    "editor",
    "ls",
    "show containers",
    "show volumes",
    "show networks",
    "login",
    "mkdir",
    "publish",
    "logout",
    "pull",
    "ps",
    "run",
    "rm",
    "start",
    "restart",
    "stop",
    "ssh",
    "touch",
    "ps",
];

const LOG_WITHOUT_SPACE: &str = "";
const LOG_WITH_SPACE: &str = " ";

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

fn mkdir() -> io::Result<()> {
    let path = Text::new("please enter the new directory name to create :")
        .prompt()
        .unwrap();
    if Path::new(path.as_str()).is_dir() {
        return Ok(());
    }
    if create_dir_all(path.as_str()).is_ok() {
        log(
            format!("Successfully created directory: {path}").as_str(),
            LOG_WITHOUT_SPACE,
        );
        return Ok(());
    }
    Err(Error::new(
        ErrorKind::InvalidInput,
        "Cannot create directory",
    ))
}
fn dirs() -> Vec<String> {
    if let Ok(working_dir) = var("DOCKS_WORKING_DIR") {
        if let Ok(public) = var("DOCKS_PUBLIC_DIR") {
            let mut dirs: Vec<String> = Vec::from([working_dir.to_string(), public]);
            let walk = ignore::WalkBuilder::new(working_dir.as_str())
                .standard_filters(true)
                .threads(4)
                .add_custom_ignore_filename("ignore.ji")
                .filter_entry(move |e| e.path().is_dir())
                .hidden(false)
                .build();
            for entry in walk.flatten() {
                let p = entry.path();
                if entry.file_type().unwrap().is_dir() {
                    if let Some(directory) = p.to_str() {
                        if directory.contains(".git").eq(&false)
                            && dirs.contains(&directory.to_string()).eq(&false)
                            && Path::new(format!("{directory}/Dockerfile").as_str()).exists()
                        {
                            dirs.push(String::from(directory));
                        }
                    }
                }
            }
            return dirs;
        }
        return Vec::new();
    }
    Vec::new()
}

fn jump() {
    loop {
        let jump = Select::new("Select a folder for jump : ", dirs())
            .prompt()
            .unwrap();
        assert!(cd(jump.as_str()).is_ok());
        log(
            format!("{jump} is the current dir").as_str(),
            LOG_WITHOUT_SPACE,
        );
        if Confirm::new("jump on an another directory ? ")
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            continue;
        }
        break;
    }
}
fn cd(dir: &str) -> io::Result<()> {
    set_current_dir(dir)
}
fn ssh_run(args: &[&str], user: &str, ip: &str) -> Result<(), Error> {
    if let Ok(mut cmd) = Command::new("ssh")
        .arg(format!("{user}@{ip}").as_str())
        .args(args)
        .spawn()
    {
        return match cmd.wait() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
    Err(Error::new(ErrorKind::NotFound, "ssh not found"))
}

fn list_networks() -> Result<(), Error> {
    docker("network", &["ls"], "/tmp")
}
fn upload_image(user: &str, ip: &str, image: &str, port: &str) -> Result<(), Error> {
    if let Ok(mut cmd) = Command::new("rsync")
        .arg("-a")
        .arg("-z")
        .arg("-e")
        .arg(format!("ssh -p {port}").as_str())
        .arg(
            format!(
                "{}/{image}/",
                var("DOCKS_PUBLIC_DIR")
                    .expect("missing DOCKS_PUBLIC_DIR")
                    .as_str()
            )
            .as_str(),
        )
        .arg(format!("{user}@{ip}:{image}").as_str())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        return match cmd.wait() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
    Err(Error::new(ErrorKind::NotFound, "rsync not found"))
}

fn clear() -> Result<(), Error> {
    if let Ok(mut child) = Command::new("clear").spawn() {
        assert!(child.wait().is_ok());
        return Ok(());
    }
    Err(Error::new(ErrorKind::NotFound, "clear failed"))
}
fn login() -> Result<(), Error> {
    let username = Text::new("Please enter your docker username : ")
        .with_default(env!("USER"))
        .prompt()
        .unwrap_or_default();
    if docker("login", &["-u", username.as_str()], "/tmp").is_ok() {
        log(format!("Logged as {username}").as_str(), LOG_WITHOUT_SPACE);
        return Ok(());
    }
    Err(Error::new(ErrorKind::NotFound, "docker username not found"))
}

fn logout() -> Result<(), Error> {
    if docker("logout", &[], "/tmp").is_ok() {
        log("Disconnected successfully", LOG_WITHOUT_SPACE);
        return Ok(());
    }
    Err(Error::new(ErrorKind::NotFound, "docker logout not found"))
}

fn servers() -> Result<Vec<String>, Error> {
    let mut servers: Vec<String> = Vec::new();
    if let Ok(config) = configuration() {
        if let Some(tables) = config.as_table() {
            for (server_name, _) in tables {
                if server_name.ne("local") {
                    servers.push(server_name.to_string());
                }
            }
            return Ok(servers);
        }
        return Err(Error::new(ErrorKind::InvalidData, "must be a table"));
    }
    Err(Error::new(ErrorKind::NotFound, "Missing configuration"))
}

fn ssh() -> Result<ExitStatus, Error> {
    let server = Text::new("Please enter the server to connect :")
        .prompt()
        .unwrap_or_default();
    let user = Text::new("Please enter the username :")
        .with_default("root")
        .prompt()
        .unwrap_or_default();
    let port = Text::new("Please enter the ssh port :")
        .with_default("22")
        .prompt()
        .unwrap_or_default();

    if let Ok(ssh) = cmd(
        "ssh",
        &[
            format!("-p {port}").as_str(),
            format!("{user}@{server}").as_str(),
        ],
    ) {
        return Ok(ssh);
    }
    Err(Error::new(ErrorKind::NotFound, "ssh not found"))
}
fn configuration() -> Result<Value, Error> {
    if let Ok(conf) = read_to_string("docks.toml") {
        if let Ok(config) = toml::from_str::<Value>(&conf) {
            return Ok(config);
        }
        return Err(Error::last_os_error());
    }
    Err(Error::last_os_error())
}
fn log(message: &str, t: &str) {
    println!(
        "{}",
        format!("\x1b[1;32m{t}\x1b[0;37m {message}\x1b[0m").as_str()
    );
}
fn cmd(program: &str, args: &[&str]) -> Result<ExitStatus, Error> {
    if let Ok(mut child) = Command::new(program).args(args).current_dir(".").spawn() {
        return child.wait();
    }
    Err(Error::new(ErrorKind::NotFound, "program not found"))
}
fn check_connexion(ip: &str, port: &str) -> Result<(), Error> {
    log(
        format!("Checking the ssh connexion on {ip}").as_str(),
        LOG_WITH_SPACE,
    );
    sleep(Duration::from_secs(1));
    if let Ok(status) = cmd("ncat", &["-z", ip, port]) {
        if status.success() {
            log(
                format!("Can communicate to the {ip} server").as_str(),
                LOG_WITH_SPACE,
            );
            return Ok(());
        }
        log(
            format!("Cannot communicate to the {ip} server").as_str(),
            LOG_WITH_SPACE,
        );
        return Err(Error::new(ErrorKind::NotFound, "ncat connexion failed"));
    }
    Err(Error::new(ErrorKind::NotFound, "ncat not founded"))
}
fn running(user: &str, ip: &str) -> Result<(), Error> {
    ssh_run(&["docker", "ps"], user, ip)
}
fn ps() -> Result<(), Error> {
    docker("ps", &["-a"], "/tmp")
}

fn build() -> Result<(), Error> {
    let tag = Text::new("Please enter the tag for the image :")
        .prompt()
        .unwrap_or_default();
    if Path::new("Dockerfile").is_file() {
        return docker("buildx", &["build", "-t", tag.as_str(), "."], ".");
    }
    Err(Error::new(ErrorKind::NotFound, "Dockerfile not found"))
}
fn list_container() -> Result<(), Error> {
    docker("container", &["ls"], "/tmp")
}

fn deploy_local() -> Result<(), Error> {
    if let Ok(docks) = configuration() {
        if let Some(table) = docks.as_table() {
            if let Some(local) = table.get("local") {
                let containers = local.get("containers").unwrap().as_array().unwrap();
                for container in containers {
                    let x = format!("./containers/{}", container.as_str().unwrap_or_default());
                    let xf = format!(
                        "./containers/{}/compose.yaml",
                        container.as_str().unwrap_or_default()
                    );
                    if Path::new(x.as_str()).is_dir() && Path::new(&xf).is_file() {
                        assert!(
                            docker("compose", &["down"], x.as_str()).is_ok(),
                            "fail to stop container"
                        );
                        assert!(
                            docker("compose", &["pull"], x.as_str()).is_ok(),
                            "fail to update container"
                        );
                        assert!(
                            docker("compose", &["up", "--remove-orphans", "-d"], x.as_str())
                                .is_ok(),
                            "fail to start container"
                        );
                    } else {
                        return Err(Error::new(
                            ErrorKind::NotFound,
                            "docker container is not a dir",
                        ));
                    }
                }
                return Ok(());
            }
            return Err(Error::new(ErrorKind::NotFound, "missing local id"));
        }
        return Err(Error::new(ErrorKind::NotFound, "docker config not valid"));
    }
    Err(Error::new(
        ErrorKind::NotFound,
        "docks.toml config not found",
    ))
}

fn server_founded(servers: usize) {
    if servers.gt(&1) {
        log(
            format!("Deploying docker containers on {servers} servers").as_str(),
            LOG_WITH_SPACE,
        );
    } else {
        log(
            format!("Deploying docker containers on {servers} server").as_str(),
            LOG_WITH_SPACE,
        );
    }
}

fn manage_remote_container(image: &str, server: &str, ip: &str, port: &str, username: &str) {
    log(
        format!("Deploying {image} docker container on {server} server").as_str(),
        LOG_WITH_SPACE,
    );
    assert!(upload_image(username, ip, image, port).is_ok());
    log(
        format!("The {image} has been deployed successfully on the {server} server").as_str(),
        LOG_WITH_SPACE,
    );
    log(
        format!("Stopping {image} before update on the {server} server").as_str(),
        LOG_WITH_SPACE,
    );
    assert!(
        ssh_run(
            &["docker", "compose", "--project-directory", image, "down",],
            username,
            ip,
        )
        .is_ok(),
        "Failed to stop container"
    );
    log(
        format!("Updating the {image} container on the {server} server").as_str(),
        LOG_WITH_SPACE,
    );
    assert!(
        ssh_run(
            &["docker", "compose", "--project-directory", image, "pull"],
            username,
            ip
        )
        .is_ok(),
        "Failed to update container"
    );
    log(
        format!("The {image} container has been updated successfully on the {server} server")
            .as_str(),
        LOG_WITH_SPACE,
    );
    log(
        format!("Restarting the {image} after upgrade on the {server} server").as_str(),
        LOG_WITH_SPACE,
    );
    assert!(
        ssh_run(
            &[
                "docker",
                "compose",
                "--project-directory",
                image,
                "--remove-orphans",
                "up",
                "-d",
            ],
            username,
            ip
        )
        .is_ok(),
        "Failed to start the container"
    );
    log(
        format!("The {image} has been restarted successfully on the {server} server").as_str(),
        LOG_WITH_SPACE,
    );
}
fn deploy_to_remote() -> Result<(), Error> {
    if let Ok(docks) = configuration() {
        if let Ok(servers) = servers() {
            server_founded(servers.len());
            for server in &servers {
                if let Some(table) = docks.as_table() {
                    if let Some(config) = table.get(server.as_str()) {
                        let username = config
                            .get("username")
                            .expect("missing username")
                            .as_str()
                            .expect("failed to parse username");
                        let port = config
                            .get("port")
                            .expect("missing port")
                            .as_str()
                            .expect("failed to parse port");
                        let ip = config
                            .get("ip")
                            .expect("missing ip")
                            .as_str()
                            .expect("missing ip");
                        let containers = config
                            .get("containers")
                            .expect("missing containers")
                            .as_array()
                            .expect("failed to parse containers");
                        if check_connexion(ip, port).is_err() {
                            log(
                                format!("Cannot communicate to the {server} server").as_str(),
                                LOG_WITH_SPACE,
                            );
                            continue;
                        }
                        for container in containers {
                            let image = container.as_str().unwrap();
                            manage_remote_container(image, server, ip, port, username);
                        }
                    }
                }
            }
        }
        return Ok(());
    }
    Err(Error::new(ErrorKind::NotFound, "docks.toml not found"))
}

fn deploy() {
    let now = Instant::now();
    let date = Local::now();
    log(
        format!("Starting deployment at {date}").as_str(),
        LOG_WITH_SPACE,
    );
    assert!(deploy_local().is_ok());
    assert!(deploy_to_remote().is_ok());
    log(
        format!("The deployment take {} secs", now.elapsed().as_secs()).as_str(),
        LOG_WITH_SPACE,
    );
}

fn editor() -> Result<(), Error> {
    if let Ok(mut child) = Command::new("ranger").arg(".").spawn() {
        assert!(child.wait().is_ok());
        return Ok(());
    }
    Err(Error::new(ErrorKind::NotFound, "Failed to run ranger"))
}
fn dock_running() -> Result<(), Error> {
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
                assert!(running(
                    username.as_str().unwrap_or_default(),
                    ip.as_str().unwrap_or_default(),
                )
                .is_ok());
            }
        }
    }
    Ok(())
}

fn remove() -> Result<(), Error> {
    loop {
        assert!(clear().is_ok());
        assert!(list_images().is_ok());
        let image = Text::new("please enter the name or the id of the image to remove : ")
            .prompt()
            .unwrap_or_default();
        if docker("image", &["rm", "-f", image.as_str()], "/tmp").is_ok() {
            if Confirm::new("remove an other image ? :")
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                continue;
            }
            break;
        }
        return Err(Error::new(ErrorKind::NotFound, "Failed to remove image"));
    }
    Ok(())
}
fn stop() -> Result<(), Error> {
    loop {
        assert!(clear().is_ok());
        assert!(list_container().is_ok());
        let image = Text::new("please enter the name or the id of the container to stop : ")
            .prompt()
            .unwrap_or_default();
        if docker("stop", &[image.as_str()], "/tmp").is_ok() {
            log(
                format!("The container {image} has been stopped successfully").as_str(),
                LOG_WITHOUT_SPACE,
            );
            if Confirm::new("stop an other container ? :")
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                continue;
            }
            break;
        }
        return Err(Error::new(
            ErrorKind::NotFound,
            "Failed to stop the container",
        ));
    }
    Ok(())
}
fn start() -> Result<(), Error> {
    loop {
        assert!(clear().is_ok());
        assert!(list_container().is_ok());
        let image = Text::new("please enter the name or the id of the image to run : ")
            .prompt()
            .unwrap();
        let host_port = Text::new("please enter the host port  : ")
            .prompt()
            .unwrap();
        let container_port = Text::new("please enter the container port  : ")
            .prompt()
            .unwrap();
        if docker(
            "run",
            &[
                "-d",
                "-p",
                format!("{host_port}:{container_port}").as_str(),
                image.as_str(),
            ],
            "/tmp",
        )
        .is_ok()
        {
            log(
                format!("The container {image} has been started in the foreground successfully")
                    .as_str(),
                LOG_WITHOUT_SPACE,
            );
            assert!(list_container().is_ok());
            if Confirm::new("run an other container ? :")
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                continue;
            }
            break;
        }
        return Err(Error::new(
            ErrorKind::NotFound,
            "Failed to run the container",
        ));
    }
    Ok(())
}

fn restart() -> Result<(), Error> {
    loop {
        assert!(clear().is_ok());
        assert!(list_container().is_ok());
        let image = Text::new("please enter the name or the id of the image to restart : ")
            .prompt()
            .unwrap();

        if docker("restart", &[image.as_str()], "/tmp").is_ok() {
            log(
                format!("The container {image} has been restarted successfully").as_str(),
                LOG_WITHOUT_SPACE,
            );
            assert!(list_container().is_ok());
            if Confirm::new("restart an other container ? :")
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                continue;
            }
            break;
        }
        return Err(Error::new(
            ErrorKind::NotFound,
            "Failed to restart the container",
        ));
    }
    Ok(())
}

fn edit() -> Result<(), Error> {
    let filename = Select::new(
        "Select a filename to edit",
        vec!["docks.toml", "compose.yaml", "Dockerfile"],
    )
    .prompt()
    .unwrap();
    if let Ok(mut child) = Command::new("vim").arg(filename).current_dir(".").spawn() {
        if child.wait().is_ok() {
            return Ok(());
        }
        return Err(Error::new(ErrorKind::NotFound, "Failed to edit filename"));
    }
    Err(Error::new(ErrorKind::NotFound, "vim not found"))
}
fn touch() -> Result<(), Error> {
    if Confirm::new("create a Dockerfile")
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true)
    {
        if let Ok(mut child) = Command::new("touch")
            .arg("Dockerfile")
            .current_dir(".")
            .spawn()
        {
            if child.wait().is_ok() {
                return Ok(());
            }
            return Err(Error::new(
                ErrorKind::NotFound,
                "Failed to create Dockerfile",
            ));
        }
        return Err(Error::new(ErrorKind::NotFound, "touch not found"));
    }
    if let Ok(mut child) = Command::new("touch")
        .arg("compose.yaml")
        .current_dir(".")
        .spawn()
    {
        if child.wait().is_ok() {
            return Ok(());
        }
        return Err(Error::new(
            ErrorKind::NotFound,
            "Failed to create compose.yaml",
        ));
    }
    Err(Error::new(ErrorKind::NotFound, "touch not found"))
}
fn pull() {
    loop {
        assert!(clear().is_ok());
        assert!(list_images().is_ok());
        let image = Text::new("please enter the name or the id of the image to pull : ")
            .prompt()
            .unwrap();

        let tag = Text::new("please enter the image tag to pull : ")
            .with_default("latest")
            .prompt()
            .unwrap();
        if docker(
            "pull",
            &[format!("{}:{}", image.as_str(), tag.as_str()).as_str()],
            "/tmp",
        )
        .is_ok()
        {
            log(
                format!("The container {image} has been updated successfully").as_str(),
                LOG_WITHOUT_SPACE,
            );

            assert!(ps().is_ok());
            if Confirm::new("pull an other image ? :")
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                continue;
            }
            break;
        }
    }
}

fn list_volumes() -> Result<(), Error> {
    docker("volume", &["ls"], "/tmp")
}
fn list_images() -> Result<(), Error> {
    docker("images", &[], "/tmp")
}
fn main() {
    assert!(clear().is_ok());
    assert!(Path::new("/usr/bin/ranger").is_executable());
    if let Ok(dir) = var("DOCKS_WORKING_DIR") {
        assert!(set_current_dir(dir).is_ok());
        loop {
            let project = current_dir().map_or_else(
                |_| String::from("."),
                |d| {
                    let parts = d
                        .to_str()
                        .unwrap()
                        .split(MAIN_SEPARATOR_STR)
                        .collect::<Vec<&str>>();
                    parts
                        .last()
                        .map_or_else(|| String::from("unknown"), |p| (*p).to_string())
                },
            );
            let selected = Select::new(
                format!("\x1b[1;34mWhat you want to do in the \x1b[1;36m{project}\x1b[1;34m project :\x1b[0m").as_str(),
                TASKS.to_vec(),
            )
                .prompt()
                .unwrap_or_default();
            match selected {
                "init" => assert!(init().is_ok()),
                "login" => assert!(login().is_ok()),
                "logout" => assert!(logout().is_ok()),
                "clear" => assert!(clear().is_ok()),
                "deploy" => deploy(),
                "check" => assert!(dock_running().is_ok()),
                "cd" => jump(),
                "edit" => assert!(edit().is_ok()),
                "enter" => enter(),
                "ssh" => assert!(ssh().is_ok()),
                "stop" => assert!(stop().is_ok()),
                "mkdir" => assert!(mkdir().is_ok()),
                "logs" => logs(),
                "commit" => assert!(commit().is_ok()),
                "show containers" => assert!(list_container().is_ok()),
                "show volumes" => assert!(list_volumes().is_ok()),
                "show networks" => assert!(list_networks().is_ok()),
                "ls" => ls(),
                "os" => os(),
                "start" => assert!(start().is_ok()),
                "restart" => assert!(restart().is_ok()),
                "rm" => assert!(remove().is_ok()),
                "touch" => assert!(touch().is_ok()),
                "ps" => assert!(ps().is_ok()),
                "pull" => pull(),
                "build" => assert!(build().is_ok()),
                "publish" => publish(),
                "editor" => assert!(editor().is_ok()),
                "exit" => break,
                _ => continue,
            }
        }
    } else {
        log("$DOCKS_WORKING_DIR not founded", LOG_WITHOUT_SPACE);
    }
    log("Bye", LOG_WITHOUT_SPACE);
}

fn init() -> io::Result<()> {
    let mut f = File::create("docks.toml")?;
    writeln!(f, "# Docker user information (optional)\n[docker]\nusername = \"\"\nemail = \"\"\n\n# Private registry information (if you are using one)\n[registry]\n# url = \"your_registry_url\"  # Uncomment and fill in if necessary\n# username = \"your_username\"  # Uncomment and fill in if necessary\n# password = \"your_password\"  # Uncomment and fill in if necessary\n\n# SSH settings for remote deployment\n[ssh]\nport = 22\nuser = \"root\"\n\n# Docker image tag configuration\n[hub]\ntags = [[\"version\", [\"stable\", \"beta\", \"nightly\", \"latest\"]],[\"env\", [\"staging\", \"dev\", \"prod\"]],[\"schedule\", [\"hourly\", \"daily\", \"weekly\", \"monthly\"]]]\n\n# List of Docker images to build\n[[hub.images]]\nname = \"\"\ntags = []\npath = \"./rlang\" # Path relative to the configuration file directory\npath = \"\"\n\n[deploy]\nlocal = []\n\n[deploy.remotes]\n")
}

fn enter() {
    assert!(list_container().is_ok());
    let image = Text::new("please enter the image to enter :")
        .prompt()
        .unwrap();
    let _ = docker("run", &["-i", "-t", image.as_str()], "/tmp");
}

fn commit() -> Result<(), Error> {
    assert!(list_container().is_ok());
    let id = Text::new("please enter the id of the container to commit :")
        .prompt()
        .unwrap();
    let image = Text::new("please enter the name of the new image :")
        .prompt()
        .unwrap();
    if docker("commit", &[id.as_str(), image.as_str()], "/tmp").is_ok() {
        log(
            format!("The image {image} has been created").as_str(),
            LOG_WITHOUT_SPACE,
        );
        return Ok(());
    }
    Err(Error::new(
        ErrorKind::InvalidInput,
        "Failed to commit image",
    ))
}
fn publish() {
    let username = Text::new("username : ")
        .with_default(var("USER").unwrap().as_str())
        .prompt()
        .unwrap();
    if let Ok(public) = var("DOCKS_PUBLIC_DIR") {
        for (image, tags) in &to_publish() {
            for tag in tags {
                assert!(docker(
                    "buildx",
                    &[
                        "build",
                        "-t",
                        format!("{username}/{image}:{tag}").as_str(),
                        format!("{public}/{image}/{tag}").as_str()
                    ],
                    "."
                )
                .is_ok());
                assert!(
                    docker("push", &[format!("{username}/{image}:{tag}").as_str()], ".").is_ok()
                );
            }
        }
    }
    assert!(clear().is_ok());
    log("all images are published successfully", LOG_WITHOUT_SPACE);
}
fn to_publish() -> HashMap<String, Vec<String>> {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(docks) = configuration() {
        if let Some(repositories) = docks.get("hub") {
            if let Some(repos) = repositories.as_table() {
                if let Some(i) = repos.get("images") {
                    if let Some(images) = i.as_array() {
                        for image in images {
                            if let Some(x) = image.as_array() {
                                if let Some(name) = x.first() {
                                    if let Some(tag) = x.get(1) {
                                        if let Some(tags) = tag.as_array() {
                                            let mut x: Vec<String> = Vec::new();
                                            for tag in tags {
                                                if let Some(name) = tag.as_str() {
                                                    x.push(name.to_string());
                                                }
                                            }
                                            if let Some(image) = name.as_str() {
                                                data.insert(image.to_string(), x);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    data
}
fn os() {
    loop {
        let image = Select::new(
            "please enter the os to download :",
            vec![
                "ubuntu",
                "alpine",
                "photon",
                "clearlinux",
                "almalinux",
                "rockylinux",
                "kalilinux/kali-rolling",
                "kalilinux/kali-last-release",
                "kalilinux/kali-dev",
                "kalilinux/kali-experimental",
                "kalilinux/kali-bleeding-edge",
                "archlinux",
                "debian",
                "amazonlinux",
                "oraclelinux",
                "fedora",
                "kali",
                "centos",
                "mageia",
            ],
        )
        .prompt()
        .unwrap();
        let tag = Text::new("Please enter the tag for the image :")
            .with_default("latest")
            .prompt()
            .unwrap();
        assert!(docker("pull", &[format!("{image}:{tag}").as_str()], "/tmp").is_ok());
        log(
            format!("{image}:{tag} has been downloaded successfully").as_str(),
            LOG_WITHOUT_SPACE,
        );
        if Confirm::new("download an other operating system ?")
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            continue;
        }
        break;
    }
}
fn ls() {
    if let Ok(mut child) = Command::new("eza")
        .arg("--git")
        .arg("--git-ignore")
        .arg("--tree")
        .arg("--level")
        .arg("7")
        .arg("--group-directories-first")
        .arg("--color")
        .arg("always")
        .arg("--icons")
        .arg("always")
        .arg("-l")
        .arg("-g")
        .arg("--total-size")
        .current_dir(".")
        .spawn()
    {
        assert!(child.wait().is_ok());
    }
}

fn logs() {
    loop {
        assert!(clear().is_ok());
        assert!(list_images().is_ok());
        let image = Text::new("please enter the name or the id of the image to show logs : ")
            .prompt()
            .unwrap();
        if docker("logs", &[image.as_str()], "/tmp").is_ok() {
            assert!(ps().is_ok());
            if Confirm::new("show logs of another image ? :")
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                continue;
            }
            break;
        }
    }
}
