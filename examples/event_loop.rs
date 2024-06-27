use steamdeck_controller_hidraw::*;

fn main() {
    let mut device = SteamDeckDevice::best().unwrap();

    println!("device: {:?}", device);
    let (tx, rx) = std::sync::mpsc::channel::<SteamDeckGamepadButton>();

    std::thread::spawn(move || {
        device.event_loop(tx);
    });

    let map = SteamDeckGamepadButton::value_to_display_name();

    loop {
        let value = rx.recv().unwrap();
        println!(
            "Pressed: {:?}",
            value.iter().map(|v| map.get(&v)).collect::<Vec<_>>()
        );
    }
}
