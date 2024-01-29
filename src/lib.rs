use bindings::{Guest, Options};
mod bindings;

struct Component;

impl Guest for Component {
    fn transform(_component: Vec<u8>, _options: Options) -> Result<Vec<u8>, String> {
        todo!("transform")
    }
}
