`capnpc-macros` holds some Rusty macros wrapping [capnpc-rust](https://github.com/dwrensha/capnpc-rust) functionality.

For example:

    let mut message =
      capnp_new!(address_book::Builder,
        [init_people 2 =>
          [
            [set_id 123]
            [set_name "Alice"]
            [set_email "alice@example.com"]
            [init_phones 1 =>
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
            [init_phones 2 =>
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
