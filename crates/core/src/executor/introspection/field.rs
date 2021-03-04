use value::ConstValue;

use super::input_value::IntrospectionInputValue;
use super::r#type::IntrospectionType;
use super::resolver::{resolve_obj, Resolver};
use crate::planner::IntrospectionSelectionSet;
use crate::schema::{ComposedSchema, MetaField};

pub struct IntrospectionField<'a>(pub &'a MetaField);

impl<'a> Resolver for IntrospectionField<'a> {
    fn resolve(
        &self,
        selection_set: &IntrospectionSelectionSet,
        schema: &ComposedSchema,
    ) -> ConstValue {
        resolve_obj(selection_set, |name, field| match name {
            "name" => ConstValue::String(self.0.name.to_string()),
            "description" => self
                .0
                .description
                .as_ref()
                .map(|description| ConstValue::String(description.clone()))
                .unwrap_or_default(),
            "isDeprecated" => ConstValue::Boolean(self.0.deprecation.is_deprecated()),
            "args" => ConstValue::List(
                self.0
                    .arguments
                    .values()
                    .map(|arg| IntrospectionInputValue(arg).resolve(&field.selection_set, schema))
                    .collect(),
            ),
            "type" => {
                IntrospectionType::new(&self.0.ty, schema).resolve(&field.selection_set, schema)
            }
            "deprecationReason" => self
                .0
                .deprecation
                .reason()
                .map(|reason| ConstValue::String(reason.to_string()))
                .unwrap_or_default(),
            _ => ConstValue::Null,
        })
    }
}
