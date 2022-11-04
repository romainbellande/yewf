use yew::prelude::*;
use yewf::{Form, Field, FormState};

#[derive(Clone, Debug, Default)]
struct MyFormState {
    form_id: String,
    foo: String,
    bar: u32,
}

impl MyFormState {
    pub fn new(form_id: &'static str) -> Self {
        Self {
            form_id: form_id.to_string(),
            ..Default::default()
        }
    }

    pub fn set_field(mut self, field_value: AppFieldValue) {
        match field_value {
           AppFieldValue::Foo(value) => {
               self.foo = value;
           },
           AppFieldValue::Bar(value) => {
               self.bar = value;
           }
        }
    }

    pub fn from_field(&self, field: AppField) -> AppFieldValue {
        match field {
            AppField::Foo => AppFieldValue::Foo(self.foo.clone()),
            AppField::Bar => AppFieldValue::Bar(self.bar.clone()),
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

enum AppFieldValue {
    Foo(String),
    Bar(u32)
}

impl AppFieldValue {
    pub fn validate(&self) -> Result<(), &'static str> {
        match self {
            Self::Foo(foo) => {
                if foo.len() > 5 {
                    Ok(())
                } else {
                    Err("foo must be at least 5 characters long")
                }
            },
            Self::Bar(bar) => {
                if bar > &5 {
                    Ok(())
                } else {
                    Err("bar must be greater than 5")
                }
            },
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
        <Form<MyFormState> {initial_state}>
            <Field<MyFormState> name={AppField::Foo} />
        </Form<MyFormState>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
