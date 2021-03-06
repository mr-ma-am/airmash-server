use specs::*;

use component::event::*;
use component::flag::ForcePlayerUpdate;
use systems::missile::MissileFireHandler;
use systems::PositionUpdate;
use types::systemdata::*;
use types::*;
use SystemInfo;

use utils::{EventHandler, EventHandlerTypeProvider};

use protocol::server::EventStealth;

#[derive(Default)]
pub struct DestealthOnFire;

#[derive(SystemData)]
pub struct DestealthOnFireData<'a> {
	conns: SendToPlayer<'a>,

	keystate: WriteStorage<'a, KeyState>,
	plane: ReadStorage<'a, Plane>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
	force: WriteStorage<'a, ForcePlayerUpdate>,
}

impl EventHandlerTypeProvider for DestealthOnFire {
	type Event = MissileFire;
}

impl<'a> EventHandler<'a> for DestealthOnFire {
	type SystemData = DestealthOnFireData<'a>;

	fn on_event(&mut self, evt: &MissileFire, data: &mut Self::SystemData) {
		if *try_get!(evt.player, data.plane) != Plane::Prowler {
			return;
		}

		try_get!(evt.player, mut data.keystate).stealthed = false;
		data.force.insert(evt.player, ForcePlayerUpdate).unwrap();

		let packet = EventStealth {
			id: evt.player.into(),
			state: false,
			energy: *try_get!(evt.player, data.energy),
			energy_regen: *try_get!(evt.player, data.energy_regen),
		};

		data.conns.send_to_player(evt.player, packet);
	}
}

impl SystemInfo for DestealthOnFire {
	type Dependencies = (MissileFireHandler, PositionUpdate);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
