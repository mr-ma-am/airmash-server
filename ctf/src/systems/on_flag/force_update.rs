use component::*;
use server::*;
use specs::*;

use server::component::flag::ForcePlayerUpdate;
use server::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct ForceUpdate;

#[derive(SystemData)]
pub struct ForceUpdateData<'a> {
	force: WriteStorage<'a, ForcePlayerUpdate>,
	carriers: WriteStorage<'a, FlagCarrier>,
}

impl EventHandlerTypeProvider for ForceUpdate {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for ForceUpdate {
	type SystemData = ForceUpdateData<'a>;

	fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
		let subject = try_get!(evt.flag, data.carriers)
			.0
			.map(Some)
			.unwrap_or(evt.player);

		// In case there's still not an entity that can be
		// found, don't force an update. This may technically
		// be a bug, but dropping an update packet on the
		// floor occasionally is really not that bad.
		let subject = match subject {
			Some(x) => x,
			None => return,
		};

		data.force.insert(subject, ForcePlayerUpdate).unwrap();
	}
}

use systems::PickupFlagSystem;

impl SystemInfo for ForceUpdate {
	type Dependencies = (
		PickupFlagSystem,
		super::KnownEventSources,
		super::SendFlagMessage,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
