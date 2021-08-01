use proc_macro2::TokenStream;
use quote::quote;

pub fn get_read_string_input_rust_struct() -> TokenStream {
    return quote! {
        #[derive(InputObject)]
        struct ReadStringInput {
            eq: MaybeUndefined<String>,
            gt: MaybeUndefined<String>,
            gte: MaybeUndefined<String>,
            lt: MaybeUndefined<String>,
            lte: MaybeUndefined<String>,
            contains: MaybeUndefined<String>,
            startsWith: MaybeUndefined<String>,
            endsWith: MaybeUndefined<String>
        }

        impl ReadStringInput {
            fn get_read_inputs(
                &self,
                field_name: String
            ) -> Vec<ReadInput> {
                let fields = [
                    (
                        &self.eq,
                        ReadInputOperation::Equals
                    ),
                    (
                        &self.gt,
                        ReadInputOperation::GreaterThan
                    ),
                    (
                        &self.gte,
                        ReadInputOperation::GreaterThanOrEqualTo
                    ),
                    (
                        &self.lt,
                        ReadInputOperation::LessThan
                    ),
                    (
                        &self.lte,
                        ReadInputOperation::LessThanOrEqualTo
                    ),
                    (
                        &self.contains,
                        ReadInputOperation::Contains
                    ),
                    (
                        &self.startsWith,
                        ReadInputOperation::StartsWith
                    ),
                    (
                        &self.endsWith,
                        ReadInputOperation::EndsWith
                    )
                ];

                let read_inputs = fields.iter().filter_map(|(field, read_input_operation)| {
                    match field {
                        MaybeUndefined::Value(field_value) => {
                            return Some(ReadInput {
                                input_type: ReadInputType::Scalar,
                                input_operation: read_input_operation.clone(),
                                field_name: String::from(&field_name),
                                field_value: field_value.sudo_serialize(),
                                relation_object_type_name: String::from(""),
                                relation_read_inputs: vec![],
                                and: vec![],
                                or: vec![]
                            });
                        },
                        MaybeUndefined::Null => {
                            return Some(ReadInput {
                                input_type: ReadInputType::Scalar,
                                input_operation: read_input_operation.clone(),
                                field_name: String::from(&field_name),
                                field_value: FieldValue::Scalar(None),
                                relation_object_type_name: String::from(""),
                                relation_read_inputs: vec![],
                                and: vec![],
                                or: vec![]
                            });
                        },
                        MaybeUndefined::Undefined => {
                            return None;
                        }
                    }
                }).collect();

                return read_inputs;
            }
        }
    };
}