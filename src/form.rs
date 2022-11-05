use yew::prelude::*;
use super::{FormContext, FormState, context::FormEvent};

#[derive(Properties)]
pub struct Props<State: FormState> {
    #[prop_or_default]
    pub children: Children,

    pub state: State,
}

impl<State: FormState> PartialEq for Props<State> {
    fn eq(&self, other: &Self) -> bool {
        self.state.get_form_id() == other.state.get_form_id()
    }
}

#[function_component(Form)]
pub fn form<State: Clone + FormState + 'static>(props: &Props<State>) -> Html {

    let ctx = use_state(|| FormContext::new(props.state.clone()));

    {
        let ctx = ctx.clone();
        if let Err(set_state_error) = ctx.sender.send(FormEvent::SetState(ctx.state.clone())) {
            log::error!("fail setting initial state in form {}, {}",
                props.state.get_form_id(), set_state_error);
        }

        for field_value in ctx.state.get_fields_value() {
            if let Err(field_value_error) = ctx.sender.send(FormEvent::SetFieldValue(field_value.clone())) {
                log::error!("fail sending field value {:?}, {}", field_value.clone(), field_value_error);
            }
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
