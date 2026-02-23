mod cli;
mod hooks;
mod menu;
mod runner;
use hooks::HookKit;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

use crate::cli::{UserArg, UserContext};
use crate::hooks::{Hook, Pattern};

fn main() {
    let hook_file = dirs::config_dir().unwrap().join("jdoit").join("hooks.toml");
    let hook_kit = HookKit::from_file(&hook_file.to_str().unwrap().to_string()).unwrap();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let context: UserContext = UserContext::from_args(&args);
    match context.args.get(0).unwrap_or_else(|| {
        std::process::exit(1);
    }) {
        UserArg::Flag(_, _) => {
            eprintln!("TODO")
        }
        _ => {}
    }
    let finded_hooks: HashMap<String, Hook> = find_hook(hook_kit.hooks, context.clone());
    let pretty_hook_map: HashMap<String, Hook> = finded_hooks
        .iter()
        .map(|(_, h)| (format!("{} - {}", h.command, h.help), h.clone()))
        .collect();
    if finded_hooks.len() > 0 {
        menu::run(pretty_hook_map, context);
    } else {
        println!("Hooks not found :(");
    }
}

fn find_hook(hooks: HashMap<String, Hook>, ctx: UserContext) -> HashMap<String, Hook> {
    let mut out_hooks: HashMap<String, Hook> = HashMap::new();
    let main_arg = match ctx.args.get(0).unwrap() {
        UserArg::Position(_, a) => a,
        _ => "",
    };
    for (k, h) in hooks {
        if h.args.len() <= ctx.position_args().iter().len() && h.check_flags(ctx.flags()) {
            match &h.pattern {
                Pattern::Text(t) => {
                    if main_arg == t {
                        out_hooks.insert(k, h.clone());
                    } else {
                        continue;
                    }
                }
                Pattern::Regex(r) => {
                    let re = Regex::from_str(&r).unwrap();
                    if re.is_match(main_arg) {
                        out_hooks.insert(k, h.clone());
                    } else {
                        continue;
                    }
                }
            }
        }
    }
    out_hooks
}
