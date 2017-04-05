
mod event;
pub mod sequence;
mod emit;

use event::Event;
use sequence::{Sequence, Handler, State};

pub trait Adapter<A> {
	fn set<'a, T>(&self, key: &str, val: &'a T) -> &'a T;
	fn get<'a>(&self, key: &str);
	fn del<'a>(&self, key: &str);
	fn seq(key: &str);
	fn fnc(key: &str);
}

pub fn Flow<'a, 'b, A, T: Adapter<A> + 'a>(adapter: Box<T>) -> Box<Fn(&'b str, &'b str) -> Event<'b> + 'a> {

	/* js code:
	if (!adapter.cache || !adapter.seq || !adapter.fn) {
		throw new Error("Flow: Invalid adapter.");
	}
	*/

	Box::new(move |sequence_id, role| emit(&adapter, sequence_id, role))
}

fn emit<'a, 'b, A, T: Adapter<A>>(adapter: &'a Box<T>, sequence_id: &'b str, role: &'b str) -> Event<'b> {

	/* js code:
		const event = Event(call, options, done);

		Sequence(adapter, call, event, data)
		.then(Emit)
		.then(event.open)
		.catch(event.done);

		return event;
	*/

	// adpater call test
	adapter.get(sequence_id);

	Event {
		sequence: sequence_id,
		role: role,
	}
}
