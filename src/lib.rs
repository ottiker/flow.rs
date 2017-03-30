
mod event;
mod sequence;
mod emit;

use event::Event;

fn emit <'a, A>(adapter: &'static A, sequence_id: &'a String, role: &'a String) -> Event<'a> {

	/* js code:
		const event = Event(call, options, done);

        Sequence(adapter, call, event, data)
        .then(Emit)
        .then(event.open)
        .catch(event.done);

        return event;
	*/

	Event {
		sequence: sequence_id,
		role: role,
	}
}

pub fn flow <'a, T>(adapter: &'static T) -> Box<Fn(&'a String, &'a String) -> Event<'a>> {

	/* js code:
	if (!adapter.cache || !adapter.seq || !adapter.fn) {
		throw new Error("Flow: Invalid adapter.");
	}
	*/

	Box::new(move |sequence_id, role| emit(adapter, sequence_id, role))
}
