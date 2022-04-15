#[derive(Debug, PartialEq)]
pub enum Message {
    AppClose,
    EditStart,
    EditSubmit(String),
    EditApproved,
    EditRejected,
    SaveChanges,
    DiscardChanges,
    SearchStart,
    SearchChanged(String),
    SearchEnd
}
