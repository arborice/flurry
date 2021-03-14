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

    fn is_override(&self, over_ride: &Self::Alias) -> bool {
        if let Some(aliases) = &self.aliases {
            aliases.contains(over_ride)
        } else {
            false
        }
    }
}

impl GenericUtil<'_> {
    pub fn try_exec(&self) -> Result<()> {
        let GenericUtil {
            bin,
            args,
            aliases,
            permissions,
            query_which,
            sanitizer,
            scan_dir,
        } = self;

        Ok(())
    }
}
