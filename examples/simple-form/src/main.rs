use yew::prelude::*;
use yewf::{Form, Field, FormState};

#[derive(Clone, Debug)]
struct MyFormState {
    form_id: String,
    foo: String,
    bar: u32,
}

impl MyFormState {
    pub fn new(form_id: &'static str) -> Self {
        Self {
            foo: "foo".to_string(),
            bar: 12,
            form_id: form_id.to_string(),
        }
    }

    pub fn from_field(&self, field: AppField) -> AppValue {
        match field {
            AppField::Foo => AppValue::Foo(self.foo.clone()),
            AppField::Bar => AppValue::Bar(self.bar.clone()),
        }
    }
}

impl FormState for MyFormState {
    type Field = AppField;

    fn get_form_id(&self) -> String {
        self.form_id.clone()
    }
}

impl PartialEq for MyFormState {
    fn eq(&self, other: &Self) -> bool {
        self.form_id == other.form_id
    }
}

enum AppValue {
    Foo(String),
    Bar(u32)
}

impl AppValue {
    pub fn validate(&self) -> bool {
        match self {
            Self::Foo(foo) => foo.len() > 5,
            Self::Bar(bar) => bar > &5,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum AppField {
    Foo,
    Bar
}

#[function_component(App)]
fn app() -> Html {
    log::debug!("application started");
    let initial_state = MyFormState::new("my_form");

    html! {
        <Form<MyFormState> id={"my_form"} {initial_state}>
            <Field<MyFormState> name={AppField::Foo} />
        </Form<MyFormState>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
