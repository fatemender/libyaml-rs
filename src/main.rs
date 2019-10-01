use libyaml::*;

fn main() {
    let mut parser = Parser::new(std::io::stdin()).unwrap();

    loop {
        let event = parser.parse().unwrap();
        println!("{:?}", event.type_());

        if event.type_() == Some(EventType::StreamEnd) {
            break;
        }
    }
}
