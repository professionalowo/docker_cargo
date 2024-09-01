#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum CreateAction {
    Create,
    Run,
}
impl Into<String> for CreateAction {
    fn into(self) -> String {
        match self {
            CreateAction::Create => "create",
            CreateAction::Run => "run",
        }
        .into()
    }
}
