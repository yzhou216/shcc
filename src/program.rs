use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use derive_builder::Builder;
use directories::BaseDirs;

#[derive(Builder)]
pub struct Program {
    path: PathBuf,
}

impl Program {
    pub fn run(&self) -> io::Result<String> {
        self.compile()?;
        Ok(String::from_utf8_lossy(&Command::new(self.exe_cache()?).output()?.stdout).into_owned())
    }

    fn compile(&self) -> io::Result<()> {
        let mut child = Command::new("cc")
            .args(["-x", "c", "-o"])
            .arg(&self.exe_cache()?)
            .arg("-")
            .stdin(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        child
            .stdin
            .take()
            .ok_or(io::Error::new(io::ErrorKind::BrokenPipe, "No stdin"))?
            .write_all(&{
                let source = self.source()?;
                source
                    .starts_with(b"#!") // Remove shebang line
                    .then(|| source.iter().position(|&byte| byte == b'\n'))
                    .flatten()
                    .map(|position| source[position + 1..].to_vec())
                    .unwrap_or(source)
            })?;

        (child.wait()?.success())
            .then_some(())
            .ok_or_else(|| io::Error::other("Compilation failed"))?;

        Ok(())
    }

    fn source(&self) -> io::Result<Vec<u8>> {
        fs::read(&self.path)
    }

    // TODO: Make cache actually caching
    fn exe_cache(&self) -> io::Result<PathBuf> {
        let cache_dir = BaseDirs::new()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No base dirs"))?
            .cache_dir()
            .join(env!("CARGO_PKG_NAME"));
        fs::create_dir_all(&cache_dir)?;
        Ok(cache_dir.join(self.stem()?))
    }

    fn stem(&self) -> io::Result<&str> {
        self.path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid file stem"))
    }
}
