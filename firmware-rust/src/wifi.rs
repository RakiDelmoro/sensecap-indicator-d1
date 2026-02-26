use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use log::{error, info};

pub struct WifiManager {
    _wifi: BlockingWifi<EspWifi<'static>>,
}

impl WifiManager {
    pub fn new(
        event_loop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
        ssid: &str,
        password: &str,
    ) -> Result<Self> {
        info!("Initializing WiFi...");

        let wifi = EspWifi::new(event_loop, nvs)?;
        let mut wifi = BlockingWifi::wrap(wifi)?;

        info!("Configuring WiFi with SSID: {}", ssid);

        let config = Configuration::Client(ClientConfiguration {
            ssid: ssid
                .try_into()
                .map_err(|_| anyhow::anyhow!("SSID too long"))?,
            password: password
                .try_into()
                .map_err(|_| anyhow::anyhow!("Password too long"))?,
            ..Default::default()
        });

        wifi.set_configuration(&config)?;
        wifi.start()?;

        info!("Connecting to WiFi...");
        wifi.connect()?;

        info!("Waiting for DHCP...");
        wifi.wait_netif_up()?;

        let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
        info!("WiFi connected! IP: {}", ip_info.ip);

        Ok(WifiManager { _wifi: wifi })
    }

    pub fn is_connected(&self) -> bool {
        true
    }
}
