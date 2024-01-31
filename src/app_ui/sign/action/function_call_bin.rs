use crate::app_ui::fields_writer::FieldsWriter;
use crate::utils::types::hex_display::HexDisplay;

pub struct FieldsContext {
    pub args_display_buf: [u8; 20],
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            args_display_buf: [0u8; 20],
        }
    }
}
pub fn format<'b, 'a: 'b>(
    args: &'b HexDisplay<500>,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 7>,
) {
    let args_fields = args.ui_fields("Args Binary", &mut field_context.args_display_buf);

    writer.push_fields(args_fields).unwrap();
}
