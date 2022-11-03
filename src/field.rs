use yew::{prelude::*,Properties, use_context};
use crate::form::FormState;

use super::{FormContext, context::FormEvent};
use std::fmt::Debug;

#[derive(PartialEq)]
pub enum FieldType {
    Text,
    Email,
    Password,
    Date
}

impl FieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Date => "date"
        }
    }
}

impl Default for FieldType {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Properties, PartialEq)]
pub struct Props<MyField: Clone + PartialEq> {
    #[prop_or_default]
    pub ty: FieldType,

    pub name: MyField,
}

#[function_component(Field)]
pub fn field<State: Clone + FormState + 'static + Debug>(props: &Props<State::Field>) -> Html {
    let context = use_context::<FormContext<State>>().expect("no context found");
    
    match context.receiver.recv().unwrap() {
        FormEvent::SetState(initial_state) => {
            log::debug!("initial state: {:?}", initial_state);
        },
        FormEvent::SetField(name) => {
            log::debug!("set field {:?}", name);
        }
    };


    html ! {
        <div>
            <input
                type={props.ty.as_str()} 
            />
        </div>
    }
}
