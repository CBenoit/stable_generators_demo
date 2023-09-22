use stable_generators_demo::generator::GeneratorState;
use stable_generators_demo::something::{combine_both, Event, UserResponse};

// Example of usage of our generator-based library, no "async" nor "await" in view

fn main() {
    dbg!("Entering main");

    let input_url = "devolutions.net";
    let mut do_something_generator = combine_both(input_url);

    dbg!("Drive the generator");

    // Start the generator
    let mut do_something_state = do_something_generator.start();

    let out = loop {
        let response = match do_something_state {
            // The generator is suspended, handle the yielded value
            GeneratorState::Suspended(event) => {
                // How the events are actually handled is up to the caller (could perform I/O with or without async)
                match event {
                    Event::HttpRequest { url } => {
                        assert_eq!(url, input_url);
                        UserResponse::Payload(vec![1, 2, 3])
                    }
                    Event::PayloadLen(len) => {
                        assert!(len == 3 || len == 500);
                        UserResponse::SomeValue(u32::try_from(len).unwrap())
                    }
                }
            }
            // The generator is in its final state, break out the execution loop
            GeneratorState::Completed(out) => break dbg!(out),
        };

        // Resume the generator
        do_something_state = do_something_generator.resume(dbg!(response));
    };

    assert!(out == 6 || out == 1000);
}
