#![allow(dead_code)]
extern crate futures;

use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use futures::prelude::*;
use futures::future::ok;
use futures::future::FutureResult;

#[derive(Debug)]
pub struct FlowError<'e> {
    message: &'e str
}

impl <'e>FlowError<'e> {
    fn new (message: &'e str) -> FlowError {
        FlowError {
            message: message
        }
    }
}

impl <'e>fmt::Display for FlowError<'e> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flow error:")
    }
}

impl <'e>Error for FlowError<'e> {
    fn description(&self) -> &str {
        self.message
    }
}

#[derive(Debug)]
pub struct Flow<'k> {
    sequences: HashMap<&'k str, Sequence>,
    handlers: HashMap<&'k str, Handler>,
}

impl <'k>Flow<'k> {

    pub fn emit (&mut self, uri: &'k str) -> Result<&Sequence, FlowError> {

        // get sequence (FutureResult)
        //  - in cache future::ok(sequence)
        if self.sequences.contains_key(uri) {
            return match self.sequences.get(uri) {
                Some(sequence) => Ok(sequence),
                None => Err(FlowError::new("hu?")),
            };
        }

        println!("Sequence {:?} not in cache.", uri);
        // TODO load sequence load(sequence_uri).then

        //let key = uri.to_owned();
        let name = uri.to_owned();
        match self.sequences.insert(uri, Sequence::build(name)) {
            Some(sequence) => {
                // this is the replaced sequence! error?
                Err(FlowError::new("Sequence was already in cache!"))
            },

            None => {

                match self.sequences.get(uri) {
                    Some(sequence) => Ok(sequence),
                    None => {
                        // error
                        Err(FlowError::new("Sequence not in cache."))
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Sequence {
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

pub fn new<'k> () -> Flow<'k> {

    // TODO add sequence loader
    // TODO add handler loader
    Flow {
        sequences: HashMap::new(),
        handlers: HashMap::new(),
        //load_sequence: Fn (),
        //load_handler: Fn ()
    }
}
