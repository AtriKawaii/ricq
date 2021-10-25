//手机设备信息
pub struct DeviceInfo {
    pub display: String,
    pub product: String,
    pub device: String,
    pub board: String,
    pub model: String,
    pub finger_print: String,
    pub imei: String,
    pub brand: String,
    pub bootloader: String,
    pub base_band: String,
    pub version: Version,
    pub sim_info: String,
    pub os_type: String,
    pub mac_address: String,
    pub ip_address: Vec<u8>,
    pub wifi_bssid: String,
    pub wifi_ssid: String,
    pub imsi_md5: Vec<u8>,
    pub android_id: String,
    pub apn: String,
    pub vendor_name: String,
    pub vendor_os_name: String,
    pub guid: Vec<u8>,
}

//系统版本
pub struct Version {
    pub incremental: String,
    pub release: String,
    pub codename: String,
    pub sdk: u32,
}