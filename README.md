# `mac_conditions`


Supported platforms: Any version of Windows that supports WMIC.
## Example

```yaml
use mac;

fn main() {
    let (wired_mac, wireless_mac, bluetooth_mac) = mac::get_mac_addresses();
    println!("wired_mac: {:?}", wired_mac);
    println!("wireless_mac: {:?}", wireless_mac);
    println!("bluetooth_mac: {:?}", bluetooth_mac);
}
```

## conditions.yaml

```yaml
conditions:  
  - adapter_type: wired  
    keywords:  
      - ["gbe", "true"]  
  - adapter_type: wireless  
    keywords:  
      - ["wi-fi", "true"]  
      - ["wi-fi", "ax"]  
      - ["wireless", "true"]  
  - adapter_type: bluetooth  
    keywords:  
      - ["bluetooth", "true"]
```

## License

`mac_conditions` is licensed under both MIT and Apache 2.0
