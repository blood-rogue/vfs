use std::io::Write;
mod datetime;
use crate::datetime::DateTime;

mod commands;
use crate::commands::Commands;

mod vfs;
use vfs::Vfs;

#[inline(always)]
fn get(lst: &Vec<String>, idx: usize) -> String {
    match lst.get(idx) {
        Some(s) => s.clone(),
        None => String::new()
    }
}

fn main() {
    let mut vfs = Vfs::new("test.vfs".to_string());
    if let Err(err) = vfs.init() {
        return println!("{err}");
    };

    let err = loop {
        let cmd = scan!(">> ");
        let cmds: Vec<String> = cmd.trim_end().split(' ').map(String::from).collect();
        let res = match Commands::from(get(&cmds, 0)) {
            Commands::LS => vfs.ls(),
            Commands::PWD => vfs.pwd(),
            Commands::HELP => Vfs::help(),
            Commands::EXIT => break vfs.exit(),
            Commands::RESET => break vfs.reset(),
            Commands::DEFRAG => Err("Not implemented".into()),

            Commands::RM => if cmds.len() == 2 {
                vfs.rm(get(&cmds, 1))
            } else {
                Err(format!("Usage: rm [path]").into())
            },

            Commands::CD => if cmds.len() == 2 {
                vfs.cd(get(&cmds, 1))
            } else {
                Err(format!("Usage: cd [path]").into())
            },

            Commands::CAT => if cmds.len() == 2 {
                vfs.cat(get(&cmds, 1))
            } else {
                Err(format!("Usage: cat [path]").into())
            },

            Commands::NANO => if cmds.len() == 2 {
                vfs.nano(get(&cmds, 1))
            } else {
                Err(format!("Usage: nano [path]").into())
            },

            Commands::TOUCH => if cmds.len() == 2 {
                vfs.touch(get(&cmds, 1))
            } else {
                Err(format!("Usage: touch [path]").into())
            },

            Commands::MKDIR => if cmds.len() == 2 {
                vfs.mkdir(get(&cmds, 1))
            } else {
                Err(format!("Usage: mkdir [path]").into())
            },

            Commands::RMDIR => if cmds.len() == 2 {
                vfs.rmdir(get(&cmds, 1))
            } else {
                Err(format!("Usage: rmdir [path]").into())
            },

            Commands::CP => if cmds.len() == 3 {
                vfs.cp(get(&cmds, 1), get(&cmds, 2))
            } else {
                Err(format!("Usage: cp [from] [to]").into())
            },

            Commands::MV => if cmds.len() == 3 {
                vfs.mv(get(&cmds, 1), get(&cmds, 2))
            } else {
                Err(format!("Usage: mv [from] [to]").into())
            },

            Commands::IMPORT => if cmds.len() == 3 {
                vfs.import(get(&cmds, 1), get(&cmds, 2))
            } else {
                Err(format!("Usage: import [from] [to]").into())
            },

            Commands::EXPORT => if cmds.len() == 3 {
                vfs.export(get(&cmds, 1), get(&cmds, 2))
            } else {
                Err(format!("Usage: export [from] [to]").into())
            },

            Commands::INVALID => Ok(println!("Invalid command entered `{}`", cmds[0])),
        };

        if let Err(err) = res {
            println!("An error occured:\n\t{err}")
        }
    };

    if let Err(err) = err {
        println!("Error occurred while exiting: {err}\n\tCheck entries before reusing.")
    }
}