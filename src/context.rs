use crossbeam_channel::{unbounded, Sender, Receiver};
use super::FormState;

trait Statable {
    fn get_field_by_name<T>(name: &'static str) -> T;
}

#[derive(Debug)]
pub enum FormEvent<State: FormState> {
    SetState(State),
    SetFieldValue(State::FieldValue)
}

pub struct FormContext<State: Clone + FormState> {
    pub sender: Sender<FormEvent<State>>,
    pub receiver: Receiver<FormEvent<State>>,
    pub state: State
}

impl<State: Clone + FormState> FormContext<State> {
    pub fn new(state: State) -> Self {
        let (sender, receiver) = unbounded();

        Self {
            state,
            sender,
            receiver,
        }
    }
}

impl<State: Clone + FormState> PartialEq for FormContext<State> {
    fn eq(&self, other: &Self) -> bool {
        self.state.get_form_id() == other.state.get_form_id()
    }
}

impl<State: FormState + Clone> Clone for FormContext<State> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            state: self.state.clone()
        }
    }
}
