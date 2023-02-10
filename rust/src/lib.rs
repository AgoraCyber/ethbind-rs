mod binding;
mod contract;

pub use binding::*;

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{read_to_string, remove_file, File},
        io::Write,
        path::PathBuf,
        process::Command,
    };

    use sha3::{Digest, Keccak256};

    use ethbind_core::{BindingBuilder, SerdeTypeMapping};

    use super::RustBinding;

    #[test]
    fn test_gen_rust() {
        _ = pretty_env_logger::try_init();

        let types_mapping: SerdeTypeMapping = include_str!("../../tests/mapping.json")
            .parse()
            .expect("Load types mapping data");

        let codes = BindingBuilder::new(RustBinding::new(types_mapping))
            .bind_hardhat("test", include_str!("../../data/abi.json"))
            .finalize()
            .expect("Generate codes")
            .to_string()
            .expect("Generate codes");

        let rust_fmt_path =
            PathBuf::from(env::var("CARGO_HOME").expect("Get CARGO_HOME")).join("bin/rustfmt");

        let temp_file_name = format!(
            "{:x}",
            Keccak256::new().chain_update(codes.as_bytes()).finalize()
        );

        let path = env::temp_dir().join(temp_file_name);

        if path.exists() {
            remove_file(path.clone()).expect("Remove exists generate file");
        }

        let mut file = File::create(path.clone()).expect("Open tmp file");

        file.write_all(codes.as_bytes()).expect("Write tmp file");

        // Call rustfmt to fmt tmp file
        let mut child = Command::new(&rust_fmt_path)
            .args([path.to_str().unwrap()])
            .spawn()
            .expect("failed to execute child");

        child.wait().expect("failed to wait on child");

        let formated = read_to_string(path).expect("Read formated codes");

        log::debug!("generated: \n{}", formated);
    }
}
