use crate::{
    cli::{UserArg, UserContext},
    hooks::Hook,
};

pub fn run_hook(hook: &Hook, ctx: UserContext) -> Result<(), String> {
    let expected_pos = hook.args.len();
    let mut flags: Vec<String> = Vec::new();
    let mut extra: Vec<String> = Vec::new();
    let mut arged_command = hook.command.clone();
    for arg in &ctx.args {
        match arg {
            UserArg::Flag(f, v) => {
                flags.push(format!("{} {}", f, v));
            }
            UserArg::Position(p, v) => {
                if *p < expected_pos {
                    arged_command = arged_command.replace(&format!("{{{}}}", p), v);
                } else {
                    extra.push(v.clone());
                }
            }
        }
    }

    println!("{}", extra.join(" "));
    arged_command = arged_command.replace("{@}", &extra.join(" "));
    arged_command = arged_command.replace("{flags}", &flags.join(" "));
    println!("{}", arged_command.clone());
    exec(arged_command);
    Ok(())
}

fn exec(cmd: String) {
    match std::process::Command::new("sh").arg("-c").arg(cmd).spawn() {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(e) => {
            eprintln!("Error execute: {}", e);
        }
    }
}
