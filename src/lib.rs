pub use byteordered::Endianness;
mod botw;
// mod cli;
pub mod model;
mod subcommand;
mod util;

pub use crate::model::Msyt;
pub type Result<T> = std::result::Result<T, failure::Error>;

#[cfg(test)]
mod tests {
    use crate::subcommand::find_files;
    use crate::{Endianness, Msyt};
    use msbt::Msbt;
    use rayon::prelude::*;
    use std::{
        convert::TryFrom,
        fs::{read_to_string, write, File},
        io::BufReader,
    };
    #[test]
    fn stock_roundtrip() {
        find_files(["test/SW"].iter().map(|s| s.to_owned()), "msyt")
            .unwrap()
            .into_par_iter()
            // .take(22)
            .try_for_each(|path| -> crate::Result<()> {
                let msyt_text = read_to_string(&path).unwrap();
                let msyt: Msyt = serde_yaml::from_str(&msyt_text).unwrap();
                let binary = msyt.clone().into_msbt_bytes(Endianness::Big).unwrap();
                let msyt2 = Msyt::from_msbt_bytes(&binary).unwrap();
                assert_eq!(msyt, msyt2);
                write(path.with_extension("msbt"), &binary).unwrap();
                Ok(())
            })
            .unwrap();
    }
    #[test]
    fn sw_roundtrip() {
        find_files(["test/SW"].iter().map(|s| s.to_owned()), "msyt")
            .unwrap()
            .into_par_iter()
            .for_each(|path| {
                let msyt_file = File::open(&path).unwrap();
                let msyt: Msyt = serde_yaml::from_reader(BufReader::new(msyt_file)).unwrap();
                let binary = msyt.clone().try_into_msbt(Endianness::Big).unwrap();
                assert_eq!(msyt, Msyt::try_from(binary).unwrap())
            });
    }
}
