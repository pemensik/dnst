use crate::error::Error;

#[derive(Clone, Debug, clap::Args)]
pub struct Help {
    #[arg(value_name = "COMMAND")]
    command: Option<String>,
}

impl Help {
    pub fn execute(self) -> Result<(), Error> {
        Ok(())
    }
}
