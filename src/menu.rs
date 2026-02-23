use std::{collections::HashMap, io::Cursor};

use skim::prelude::*;

use crate::{cli::UserContext, hooks::Hook, runner::run_hook};

pub fn run(pretty_hook_map: HashMap<String, Hook>, context: UserContext) {
    let options = SkimOptionsBuilder::default()
        .height("50%")
        .multi(true)
        .build()
        .unwrap();
    let input: String = pretty_hook_map
        .keys()
        .map(|k| k.clone())
        .collect::<Vec<String>>()
        .join("\n");
    let hooks_reader = SkimItemReader::default();
    let hooks = hooks_reader.of_bufread(Cursor::new(input));
    let output = Skim::run_with(options, Some(hooks))
        .map(|out| out.selected_items)
        .unwrap_or_else(|_| vec![]);
    for h in output {
        run_hook(
            pretty_hook_map.get(&h.item.to_string()).unwrap(),
            context.clone(),
        )
        .unwrap();
    }
}
