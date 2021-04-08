use crate::tui::widgets::*;

pub fn edit_seq_items() -> [SeqFrame; 8] {
    [
        SeqFrame::new(
            "Key (trigger) for new command?",
            "key cannot be empty",
            |val| !val.trim().is_empty(),
        ),
        SeqFrame::new("Binary/primary command?", "", |val| !val.trim().is_empty()),
        SeqFrame::new("Arguments for command?", "", |_| true),
        SeqFrame::new("Aliases for command?", "", |_| true),
        SeqFrame::new("Encoder for output?", "", |val| {
            vec!["none", "n", "false", "json", "url", "web"].contains(&val)
        }),
        SeqFrame::new("Permissions schema?", "", |val| {
            vec!["none", "any", "dfl", "group", "root", "user"].contains(&val)
        }),
        SeqFrame::new("Query which?", "", |val| {
            vec!["y", "yes", "true", "f", "n", "no", "false"].contains(&val)
        }),
        SeqFrame::new("Query which?", "", |val| {
            vec!["y", "yes", "true", "f", "n", "no", "false"].contains(&val)
        }),
    ]
}
