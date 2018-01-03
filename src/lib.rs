#![allow(dead_code)]
extern crate futures;

use std::error::Error;
use std::collections::HashMap;
use futures::prelude::*;
use futures::future::ok;
use futures::future::FutureResult;

#[derive(Debug)]
pub struct Flow {
    sequences: HashMap<String, Sequence>,
    handlers: HashMap<String, Handler>,
}

impl Flow {

    // emit a sequence, if the sequence doesn't exists it will be built and then emitted.
    fn emit(&mut self, sequence_uri: String) -> FutureResult<Sequence, Box<Error>> {

        // TODO execute this future on reactor
        (match self.sequences.get(&sequence_uri) {

            // sequence exists in cache
            Some(sequence) => ok(sequence),

            // sequence doesn't exists in the cache, thus build the sequence
            None => {

                // TODO load sequence load(sequence_uri).then
                // TODO this is async!
                let sequence = Sequence::build(sequence_uri);
                self.sequences.insert(sequence_uri, sequence);
                ok(sequence)
            }
        })//.and_then(|sequence| {
            //sequence.start()
        //})//.wait()
    }
}

#[derive(Debug)]
struct Sequence {
    uri: String,
    done: bool,
    //seq: Vec<Handler>,
    //err: Box<Sequence>,
}

impl Sequence {

    pub fn start<T> (&self) -> FutureResult<String, Box<Error>>  {
        // TODO call handlers2
        // TODO return result from last handler
        ok("result".to_string())
    }

    fn build (uri: String) -> Sequence {

        println!("Build sequence.....");

        Sequence {
            uri: uri,
            done: false
        }

        // Tipp: let v = vec![wfi_1, wfi_2]; let sel = join_all(v); ..will join all futures, like promise.all([])

        // TODO: Load sequence assets
        // - Arguments: parse json to argument type
        // - Handler dependencies: shared libraries
        // - Handlers: shared libraries -> Vec<Fn>

    }
}

impl Future for Sequence {
    type Item = String;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        // TODO if all handler resolved return ready with result from last handler -> Ok(Async::Ready())
        // TODO unpark when sequence is Ready

        // TODO if one of the handlers rejected -> Error
        // TODO if one of the handlers crashed -> Error

        // TODO else -> Ok(Async::NotReady())

        Ok(Async::Ready("sequence_ready".to_string()))
    }
}

#[derive(Debug)]
struct Handler {
    uri: String
}

impl Handler {

    fn call() -> FutureResult<String, Box<Error>> {
        // Call handler
        ok("handler_result".to_string())
    }

    fn build (uri: String) -> Handler {

        println!("Build handler.....");

        Handler {
            uri: uri
        }
    }
}

impl Future for Handler {
    type Item = String;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        // TODO if handler resolved called -> Ok(Async::Ready())
        // TODO notify sequence to call next handler

        // TODO if handler rejected -> Error
        // TODO if handler crashed -> Error
        // TODO else -> Ok(Async::NotReady())

        Ok(Async::Ready("handler_ready".to_string()))
    }
}

pub fn new () -> Flow {

    // TODO add sequence loader
    // TODO add handler loader
    Flow {
        sequences: HashMap::new(),
        handlers: HashMap::new(),
        //load_sequence: Fn (),
        //load_handler: Fn ()
    }
}
