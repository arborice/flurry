use crate::{config::types::GenericUtil, prelude::*};

impl<'util> Program<'util> for GenericUtil<'util> {
    type Bin = &'util str;

    fn get_bin(&self) -> Self::Bin {
        self.bin
    }
}

impl<'util> AliasedProgram<'util, 'util> for GenericUtil<'util> {
    type Alias = &'util str;
    type Aliases = Option<Vec<Self::Alias>>;

    fn aliases(&self) -> Self::Aliases {
        self.aliases.clone()
    }

    fn is_override(&self, _over_ride: &Self::Alias) -> bool {
        false
    }
}

impl<'util> ProgramExec<'util, 'util> for GenericUtil<'util> {
    type Args = Vec<&'static str>;

    fn try_exec_override(&self, args: Self::Args, cfg: &GlobalConfig) -> Result<()> {
        Ok(())
    }
}
