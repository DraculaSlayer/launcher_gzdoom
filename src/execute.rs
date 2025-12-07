use std::process::Command;

pub struct ExecuteDoom {}

impl ExecuteDoom {
    pub fn execute(wad: String, list: Vec<String>) -> std::io::Result<()> {

        let mut args: Vec<String> = vec!("-iwad".to_string(), wad);

        if list[0] == "none" {return Ok(())}else {

            for i in list {
                let s = i.clone();

                args.push("-File".to_string());
                args.push(s.to_string());
            }
        }

        let run = Command::new("gzdoom").args(args).output().expect("failed");

        Ok(())
    }
}
