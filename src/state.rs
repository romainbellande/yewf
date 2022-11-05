use std::fmt::Debug;

pub trait FormState: Clone + PartialEq {
    type Field: Clone + PartialEq + Debug;
    type FieldValue: Clone + PartialEq + Debug;

    fn get_form_id(&self) -> String;

    fn get_fields_value(&self) -> Vec<Self::FieldValue>;

    fn set_field(self, field_value: Self::FieldValue);

    fn from_field(&self, field: Self::Field) -> Self::FieldValue;
}

