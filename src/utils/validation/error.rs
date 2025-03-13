use std::{borrow::Cow, collections::HashMap};

type DefaultMessageParams<'a> = HashMap<Cow<'a, str>, serde_json::Value>;
fn format_default_length_message(params: &DefaultMessageParams) -> String {
    let _min = params.get("min");
    let _max = params.get("max");
    let _value = params.get("value");

    "invalid string length".to_string()
}
fn format_default_email_message(_params: &DefaultMessageParams) -> String {
    "invalid email format".to_string()
}
fn format_default_message(_params: &DefaultMessageParams) -> String {
    "default message is not implemented for this validation kind".to_string()
}
fn format_validation_error(validation_error: &validator::ValidationError) -> String {
    let message = validation_error.message.as_ref();

    if let Some(message) = message {
        message.to_string()
    } else {
        let code = validation_error.code.as_ref();
        let params = &validation_error.params;

        let format = match code {
            "length" => format_default_length_message,
            "email" => format_default_email_message,
            _ => format_default_message,
        };
        format(params)
    }
}

type FieldErrors<'a> = (Cow<'a, str>, &'a Vec<validator::ValidationError>);
fn format_field_errors((field_key, field_values): FieldErrors) -> (Cow<str>, Vec<String>) {
    let collected_values = field_values
        .into_iter()
        .map(format_validation_error)
        .collect::<Vec<_>>();

    (field_key, collected_values)
}
pub fn format_validation_errors(
    validation_errors: &validator::ValidationErrors,
) -> HashMap<Cow<str>, Vec<String>> {
    validation_errors
        .field_errors()
        .into_iter()
        .map(format_field_errors)
        .collect::<HashMap<_, Vec<_>>>()
}
