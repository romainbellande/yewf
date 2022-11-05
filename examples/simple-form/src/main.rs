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
}

impl FormState for MyFormState {
    type Field = AppField;
    type FieldValue = AppFieldValue;

    fn get_form_id(&self) -> String {
        self.form_id.clone()
    }

    fn get_fields_value(&self) -> Vec<Self::FieldValue> {
        vec![
            self.from_field(Self::Field::Foo),
            self.from_field(Self::Field::Bar),
        ]
    }

    fn set_field(mut self, field_value: Self::FieldValue) {
        match field_value {
           Self::FieldValue::Foo(value) => {
               self.foo = value;
           },
           Self::FieldValue::Bar(value) => {
               self.bar = value;
           }
        }
    }

    fn from_field(&self, field: Self::Field) -> Self::FieldValue {
        match field {
            Self::Field::Foo => Self::FieldValue::Foo(self.foo.clone()),
            Self::Field::Bar => Self::FieldValue::Bar(self.bar.clone()),
        }
    }
}

impl PartialEq for MyFormState {
    fn eq(&self, other: &Self) -> bool {
        self.form_id == other.form_id
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    let state = MyFormState::new("my_form");

    html! {
        <Form<MyFormState> {state}>
            <Field<MyFormState> name={AppField::Foo} />
        </Form<MyFormState>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
