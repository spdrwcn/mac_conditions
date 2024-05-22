use mac_conditions;

fn main() {
    let (wired_mac, wireless_mac, bluetooth_mac) = mac_conditions::get_mac_addresses();
    println!("wired_mac: {:?}", wired_mac);
    println!("wireless_mac: {:?}", wireless_mac);
    println!("bluetooth_mac: {:?}", bluetooth_mac);
}
