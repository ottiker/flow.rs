
pub mod sequence;
//mod event;
//mod emit;

//use event::Event;
//use sequence::{Sequence, Handler, State};

pub trait Adapter<A, K: Sized, V> {
	fn set<'a>(&mut self, key: K, val: V) -> Option<V>;
	fn get(&self, key: &K);
	fn del(&self, key: K);
	fn seq(key: &str);
	fn fnc(key: &str);
}

pub fn flow<'a, 'b, A, K, V, T: Adapter<A, K, V> + 'a>(adapter: Box<T>) -> Box<Fn(K, &'b str) + 'a> {

	/* js code:
	if (!adapter.cache || !adapter.seq || !adapter.fn) {
		throw new Error("Flow: Invalid adapter.");
	}
	*/

	Box::new(move |sequence_id, role| emit(&adapter, &sequence_id, &role))
}

fn emit<'a, 'b, A, K, V, T: Adapter<A, K, V>>(adapter: &Box<T>, sequence_id: &K, role: &'b str)/* -> Event<'a, K>*/ {

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

	//Event {
	//	sequence: sequence_id
	//}
}
