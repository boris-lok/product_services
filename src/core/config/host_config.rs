#[derive(Debug)]
pub struct HostConfig {
    host_ip: String,
    host_port: u16,
}

impl HostConfig {
    pub fn new() -> Self {
        let host_ip = dotenv::var("HOST_IP").expect("Can read the host ip from env.");

        let host_port = dotenv::var("HOST_PORT")
            .expect("Can read the host port from env.")
            .parse::<u16>()
            .expect("Can parse the host port to u16");

        Self { host_ip, host_port }
    }
}
