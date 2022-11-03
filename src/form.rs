use yew::prelude::*;
use super::{FormContext, context::FormEvent};
use std::fmt::Debug;

pub trait FormState: Clone + PartialEq {
    type Field: Clone + PartialEq + Debug;

    fn get_form_id(&self) -> String;
}

#[derive(Properties)]
pub struct Props<State> {
    #[prop_or_default]
    pub children: Children,

    pub initial_state: State,

    pub id: &'static str,
}

impl<State> PartialEq for Props<State> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[function_component(Form)]
pub fn form<State: Clone + FormState + 'static>(props: &Props<State>) -> Html {

    let ctx = use_state(|| FormContext::new(props.id, props.initial_state.clone()));

    {
        let ctx = ctx.clone();
        if let Err(set_state_error) = ctx.sender.send(FormEvent::SetState(ctx.initial_state.clone())) {
            log::error!("fail setting initial state in form {}, {}", props.id, set_state_error);
        }
    }


    html! {
        <ContextProvider<FormContext<State>> context={(*ctx).clone()}>
            <form>
                { for props.children.iter() }
            </form>
        </ContextProvider<FormContext<State>>>
    }
}
