use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsAnyFunction, JsExportDefaultFunctionClause};

impl ToFormatElement for JsExportDefaultFunctionClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let default_token = formatter.format_token(&self.default_token()?)?;
        let class = JsAnyFunction::from(self.clone()).to_format_element(formatter)?;

        Ok(format_elements![default_token, space_token(), class])
    }
}