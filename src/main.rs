use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use std::any::Any;
use std::error::Error;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // Get the first Bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // Start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_secs(2)).await; // Wait for peripherals to be discovered

    // Print the names of peripherals and check for Even Realities glasses
    find_even_realities_glasses(&central).await;

    Ok(())
}

async fn find_even_realities_glasses(central: &Adapter) {
    let peripherals = central.peripherals().await.unwrap();
    if peripherals.is_empty() {
        println!("No peripherals found.");
    } else {
        for p in peripherals {
            if let Some(props) = p.properties().await.unwrap() {
                if let Some(name) = props.local_name {
                    println!("Peripheral name: {}", name);
                    // Check if the name matches Even Realities glasses pattern
                    if name.contains("Even G1_21") {
                        println!("Found Even Realities glasses: {}", name);
                        // You can now interact with the glasses here
                    }
                } else {
                    println!("Peripheral with no name found. id:{:?}", props.type_id());
                }
            } else {
                println!("Unable to fetch properties for a peripheral.");
            }
        }
    }
}
