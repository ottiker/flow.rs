
mod event;
pub mod sequence;
mod emit;

use event::Event;
use sequence::{Sequence, Handler, State};

pub struct Adapter {
	_lru_size: usize
}
pub trait TAdapter {
	fn set<'a, T>(key: &str, val: &'a T) -> &'a T;
	fn get<'a, TAdapter>(adapter: &'static Adapter, key: &str);
	fn del<'a>(key: &str);
	fn seq(key: &str);
	fn fnc(key: &str);
}

pub fn Flow<'a, T: TAdapter>(adapter: &'static Adapter) -> Box<Fn(&'a str, &'a str) -> Event<'a>> {

	/* js code:
	if (!adapter.cache || !adapter.seq || !adapter.fn) {
		throw new Error("Flow: Invalid adapter.");
	}
	*/
	// TODO how to make adapter available in the closure?

	Box::new(move |sequence_id, role| emit(adapter, sequence_id, role))
}

fn emit <'a>(adapter: &'static Adapter, sequence_id: &'a str, role: &'a str) -> Event<'a> {

	/* js code:
		const event = Event(call, options, done);

		Sequence(adapter, call, event, data)
		.then(Emit)
		.then(event.open)
		.catch(event.done);

		return event;
	*/

	// TODO how to get the reference to the adapter
	<Adapter as TAdapter>::get(&adapter, sequence_id);


	Event {
		sequence: sequence_id,
		role: role,
	}
}
