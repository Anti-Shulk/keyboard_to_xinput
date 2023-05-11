use std::{time::{self,}, thread};
use inputbot::{KeybdKey::*};
use vigem_client::{XButtons};

fn main() {
    	// Connect to the ViGEmBus driver
	let client = vigem_client::Client::connect().unwrap();

	// Create the virtual controller target
	let id = vigem_client::TargetId::XBOX360_WIRED;
	let mut target = vigem_client::Xbox360Wired::new(client, id);

	// Plugin the virtual controller
	target.plugin().unwrap();

	// Wait for the virtual controller to be ready to accept updates
	target.wait_ready().unwrap();

	// The input state of the virtual controller
	let mut gamepad = vigem_client::XGamepad {
		buttons: vigem_client::XButtons!(),
		..Default::default()
	};

	loop {
		let mut gamepad_value = 0x0000 as u16;

		if (AKey.is_pressed()) { gamepad_value += XButtons::A }

		if (BKey.is_pressed()) { gamepad_value += XButtons::B }

		if (XKey.is_pressed()) { gamepad_value += XButtons::X }

		if (YKey.is_pressed()) { gamepad_value += XButtons::Y }


		gamepad.buttons = vigem_client::XButtons { raw: gamepad_value };

	
		let _ = target.update(&gamepad);

		thread::sleep(time::Duration::from_millis(10));
	}
}
