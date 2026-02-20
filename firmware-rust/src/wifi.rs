use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::WifiWait;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi};
use log::{error, info};

pub struct WifiManager {
    _wifi: EspWifi<'static>,
    connected: bool,
}

impl WifiManager {
    pub fn new(
        event_loop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
        ssid: &str,
        password: &str,
    ) -> Result<Self> {
        info!("Initializing WiFi...");

        let mut wifi = EspWifi::new(event_loop, nvs)?;

        info!("Configuring WiFi with SSID: {}", ssid);

        let config = Configuration::Client(ClientConfiguration {
            ssid: ssid
                .try_into()
                .map_err(|_| anyhow::anyhow!("SSID too long"))?,
            password: password
                .try_into()
                .map_err(|_| anyhow::anyhow!("Password too long"))?,
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        });

        wifi.set_configuration(&config)?;
        wifi.start()?;
        wifi.connect()?;

        info!("Waiting for WiFi connection...");

        // Wait for connection
        if !WifiWait::new(&event_loop)?
            .wait_with_timeout(std::time::Duration::from_secs(30), || {
                wifi.is_connected().unwrap_or(false)
            })
        {
            return Err(anyhow::anyhow!("Failed to connect to WiFi"));
        }

        let ip_info = wifi.sta_netif().get_ip_info()?;
        info!("WiFi connected! IP: {}", ip_info.ip);

        Ok(WifiManager {
            _wifi: wifi,
            connected: true,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
