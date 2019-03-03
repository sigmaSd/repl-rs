use std::env::temp_dir;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

#[derive(Clone)]
pub struct CargoCmds {
    tmp_dir: PathBuf,
    rust_repl_playground_dir: PathBuf,
    main_file: PathBuf,
}
impl Default for CargoCmds {
    fn default() -> Self {
        let tmp_dir = temp_dir();
        let rust_repl_playground_dir = {
            let mut dir = tmp_dir.clone();
            dir.push("rust_repl_playground");
            dir
        };
        let main_file = {
            let mut dir = rust_repl_playground_dir.clone();
            dir.push("src/main.rs");
            dir
        };
        Self {
            tmp_dir,
            rust_repl_playground_dir,
            main_file,
        }
    }
}
impl CargoCmds {
    pub fn cargo_new(&self) -> Result<(), io::Error> {
        self.clean_toml();
        if !Path::new(&self.rust_repl_playground_dir).exists() {
            Command::new("cargo")
                .current_dir(&*self.tmp_dir)
                .args(&["new", "rust_repl_playground"])
                .spawn()?
                .wait()?;
        }
        self.cargo_build()?;
        Ok(())
    }

    pub fn cargo_run(&self, code: String) -> Result<String, io::Error> {
        let mut main = File::create(&*self.main_file)?;
        write!(main, "{}", code)?;
        let out = Command::new("cargo")
            .current_dir(&*self.rust_repl_playground_dir)
            .arg("run")
            .output()?;

        let stdout = String::from_utf8(out.stdout).expect("Invalid input (Not Utf-8)");
        let stderr = String::from_utf8(out.stderr).expect("Invalid input (Not Utf-8)");
        if stdout.is_empty() {
            Ok(stderr)
        } else {
            Ok(stdout)
        }
    }

    pub fn cargo_add(&self, add_dep: &str) -> Result<Child, io::Error> {
        let add_dep: Vec<&str> = add_dep.split(' ').collect();
        if add_dep.len() < 2 {
            //TODO: Better fix this
            println!("missing dependency for cargo add cmd");
            return Err(io::Error::from_raw_os_error(1));
        }

        Command::new("cargo")
            .current_dir(&*self.rust_repl_playground_dir)
            .args(&add_dep)
            .spawn()?
            .wait()?;

        Ok(Command::new("cargo")
            .current_dir(&*self.rust_repl_playground_dir)
            .arg("build")
            .spawn()?)
    }

    fn cargo_build(&self) -> Result<(), io::Error> {
        Command::new("cargo")
            .current_dir(&*self.rust_repl_playground_dir)
            .arg("build")
            .spawn()?
            .wait()?;
        Ok(())
    }

    fn clean_toml(&self) {
        use std::fs::File;
        use std::io::Read;

        let mut clean = String::new();

        let toml_file = {
            let mut f = self.rust_repl_playground_dir.clone();
            f.push("Cargo.toml");
            f
        };

        if !Path::exists(&toml_file) {
            return;
        }

        let mut toml_read = File::open(&toml_file).unwrap();

        let toml_contents = {
            let mut c = String::new();
            toml_read.read_to_string(&mut c).unwrap();
            c
        };

        for line in toml_contents.lines() {
            clean.push_str(line);
            if line.contains("[dependencies]") {
                break;
            }
            clean.push('\n')
        }

        let mut toml_write = File::create(&toml_file).unwrap();
        write!(toml_write, "{}", clean).unwrap();
    }
}
