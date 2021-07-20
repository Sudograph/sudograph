use graphql_parser::schema::ObjectType;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote
};

pub fn generate_update_mutation_resolvers(object_types: &Vec<ObjectType<String>>) -> Vec<TokenStream> {
    let generated_query_resolvers = object_types.iter().map(|object_type| {
        let object_type_name = &object_type.name;
        
        let object_type_rust_type = format_ident!(
            "{}",
            object_type_name
        );

        let update_function_name = format_ident!(
            "{}",
            String::from("update") + object_type_name
        );

        let update_input_type = format_ident!(
            "{}",
            String::from("Update") + object_type_name + "Input"
        );

        return quote! {
            async fn #update_function_name(
                &self,
                context: &sudograph::async_graphql::Context<'_>,
                input: #update_input_type
            ) -> std::result::Result<Vec<#object_type_rust_type>, sudograph::async_graphql::Error> {
                let object_store = storage::get_mut::<ObjectTypeStore>();
                
                let update_result = update(
                    object_store,
                    #object_type_name,
                    &input.id.to_string(), // TODO we might want to get rid of this?
                    &input.get_update_field_inputs(),
                    &convert_selection_field_to_selection_set(
                        #object_type_name,
                        context.field(),
                        SelectionSet(None)
                    )
                );
                    
                match update_result {
                    Ok(strings) => {
                        let deserialized_strings: Vec<#object_type_rust_type> = strings.iter().map(|string| {
                            return from_str(string).unwrap();
                        }).collect();

                        return Ok(deserialized_strings);
                    },
                    Err(error) => {
                        // TODO we need to panic here to ensure all state changes are undone
                        // TODO unfortunately this means that for now we will not be able to return
                        // TODO a nice JSON result
                        // TODO see this forum post for a way to possibly get around this in the future: https://forum.dfinity.org/t/discard-state-changes-from-update-call-without-trap/5862
                        return Err(sudograph::async_graphql::Error::from(error));
                    }
                };
            }
        };
    }).collect();

    return generated_query_resolvers;
}