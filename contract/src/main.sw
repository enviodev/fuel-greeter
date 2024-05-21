contract;

use std::storage::storage_api::{read, write, clear};

// Workaround for the following error: `str` or a type containing `str` on `configurables` is not allowed.
// But might be not needed, since we use str[8] now
const GREETING_KEY: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

struct Greeting {
    value: str[8]
}

enum Error {
    InvalidContractSender: (),
    ToThrow: (),
}

pub struct NewGreeting {
    user: Address,
    greeting: Greeting,
}

pub struct ClearGreeting {
    user: Address,
}

abi Greeter {
    #[storage(read)]
    fn greet() -> Option<Greeting>;
 
    #[storage(write)]
    fn set_greeting(greeting: str[8]);

    #[storage(write)]
    fn clear_greeting();

    fn throw_error();
}


impl Greeter for Contract {
    #[storage(read)]
    fn greet() -> Option<Greeting> {
        read::<Greeting>(GREETING_KEY, 0)  
    }

    #[storage(write)]
    fn set_greeting(greeting: str[8]) {
        let sender = msg_sender().unwrap();
        if let Identity::Address(address) = sender {
            let g = Greeting{
                value: greeting,
            };
            write(GREETING_KEY, 0, g);
            log(NewGreeting {
              user: address,
              greeting: g,
            });
        } else {
            require(false, Error::InvalidContractSender);
        }
    }

    #[storage(write)]
    fn clear_greeting() {
        let sender = msg_sender().unwrap();
        if let Identity::Address(address) = sender {
            let _ = clear::<Greeting>(GREETING_KEY, 0);
            log(ClearGreeting {
              user: address,
            });
        } else {
            require(false, Error::InvalidContractSender);
        }
    }

    fn throw_error() {
        require(false, Error::ToThrow);
    }
}
