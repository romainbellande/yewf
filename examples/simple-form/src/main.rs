use yew::prelude::*;
use yewf::{Form, Field};

#[derive(Clone, Debug)]
struct MyFormState {
    foo: String
}

impl MyFormState {
    pub fn new() -> Self {
        Self {
            foo: "bar".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum AppField {
    Foo(String)
}

impl AppField {
    pub fn validate(&self) -> bool {
        match self {
            Self::Foo(foo) => foo.len() > 5
        }
    }
}


#[function_component(App)]
fn app() -> Html {
    log::debug!("application started");
    let initial_state = MyFormState::new();

    html! {
        <Form<MyFormState, AppField> id={"my_form"} {initial_state}>
            <Field<MyFormState, AppField> name={AppField::Foo("".to_string())} />
        </Form<MyFormState, AppField>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
