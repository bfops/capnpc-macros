// Copyright (c) 2013-2014 Sandstorm Development Group, Inc. and contributors
// Licensed under the MIT License:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![crate_type = "bin"]
#![feature(core, io)]

extern crate capnp;
pub mod addressbook_capnp {
  include!(concat!(env!("OUT_DIR"), "/addressbook_capnp.rs"));
}

#[macro_use]
extern crate "capnpc-macros" as capnpc_macros;

pub mod addressbook {
    use addressbook_capnp::{address_book, person};
    use capnp::serialize_packed;
    use capnp::{MessageBuilder, MessageReader, ReaderOptions, MallocMessageBuilder};

    pub fn write_address_book() -> ::std::io::Result<()> {
        let mut message =
            capnpc_new!(
                address_book::Builder =>
                [array init_people 2 =>
                    [
                        [set_id 123]
                        [set_name "Alice"]
                        [set_email "alice@example.com"]
                        [array init_phones 1 =>
                          [
                              [set_number "555-1212"]
                              [set_type {person::phone_number::Type::Mobile}]
                          ]
                        ]
                        [init_employment => [set_school "MIT"]]
                    ]

                    [
                        [set_id 456]
                        [set_name "Bob"]
                        [set_email "bob@example.com"]
                        [array init_phones 2 =>
                            [
                                [set_number "555-4567"]
                                [set_type {person::phone_number::Type::Home}]
                            ]
                            [
                                [set_number "555-7654"]
                                [set_type {person::phone_number::Type::Work}]
                            ]
                        ]
                        [init_employment => [set_unemployed ()]]
                    ]
                ]
            );

        serialize_packed::write_packed_message_unbuffered(
            &mut ::capnp::io::WriteOutputStream::new(::std::io::stdout()), &mut message)
    }

    pub fn print_address_book() -> ::std::io::Result<()> {

        let message_reader = try!(serialize_packed::new_reader_unbuffered(
            ::capnp::io::ReadInputStream::new(::std::io::stdin()), ReaderOptions::new()));
        let address_book = message_reader.get_root::<address_book::Reader>();

        for person in address_book.get_people().iter() {
            println!("{}: {}", person.get_name(), person.get_email());
            for phone in person.get_phones().iter() {
                let type_name = match phone.get_type() {
                    Some(person::phone_number::Type::Mobile) => {"mobile"}
                    Some(person::phone_number::Type::Home) => {"home"}
                    Some(person::phone_number::Type::Work) => {"work"}
                    None => {"UNKNOWN"}
                };
                println!("  {} phone: {}", type_name, phone.get_number());
            }
            match person.get_employment().which() {
                Some(person::employment::Unemployed(())) => {
                    println!("  unemployed");
                }
                Some(person::employment::Employer(employer)) => {
                    println!("  employer: {}", employer);
                }
                Some(person::employment::School(school)) => {
                    println!("  student at: {}", school);
                }
                Some(person::employment::SelfEmployed(())) => {
                    println!("  self-employed");
                }
                None => { }
            }
        }
        Ok(())
    }
}

pub fn main() {

    let args : Vec<String> = ::std::env::args().collect();
    if args.len() < 2 {
        println!("usage: $ {} [write | read]", args[0]);
    } else {
        match args[1].as_slice() {
            "write" => addressbook::write_address_book().unwrap(),
            "read" =>  addressbook::print_address_book().unwrap(),
            _ => {println!("unrecognized argument") }
        }
    }

}
