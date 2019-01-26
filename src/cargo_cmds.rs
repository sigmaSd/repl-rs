use std::env::temp_dir;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

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
        let _ = fs::remove_dir_all(&*self.rust_repl_playground_dir);
        Command::new("cargo")
            .current_dir(&*self.tmp_dir)
            .args(&["new", "rust_repl_playground"])
            .spawn()?
            .wait()?;
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

    pub fn cargo_add(&self, add_dep: &str) -> Result<(), io::Error> {
        let add_dep: Vec<&str> = add_dep.split(' ').collect();
        if add_dep.len() < 2 {
            println!("missing dependency for cargo add cmd");
            return Ok(());
        }
        Command::new("cargo")
            .current_dir(&*self.rust_repl_playground_dir)
            .args(&add_dep)
            .spawn()?
            .wait()?;
        Ok(())
    }
}
