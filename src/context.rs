use crossbeam_channel::{unbounded, Sender, Receiver};

trait Statable {
    fn get_field_by_name<T>(name: &'static str) -> T;
}

#[derive(Debug)]
pub enum FormEvent<State, Field> {
    SetState(State),
    SetField(Field)
}

pub struct FormContext<State: Clone, Field: Clone> {
    pub id: String,
    pub sender: Sender<FormEvent<State, Field>>,
    pub receiver: Receiver<FormEvent<State, Field>>,
    pub initial_state: State
}

impl<State: Clone, Field: Clone> FormContext<State, Field> {
    pub fn new(id: &'static str, initial_state: State) -> Self {
        let (sender, receiver) = unbounded();

        Self {
            initial_state,
            sender,
            receiver,
            id: id.to_string()
        }
    }
}

impl<State: Clone, Field: Clone> PartialEq for FormContext<State, Field> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<State: Clone, Field: Clone> Clone for FormContext<State, Field> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            initial_state: self.initial_state.clone()
        }
    }
}
