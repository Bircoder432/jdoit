#[derive(Clone, Debug)]
pub enum UserArg {
    Position(usize, String),
    Flag(String, String),
}

#[derive(Clone)]
pub struct UserContext {
    pub args: Vec<UserArg>,
}

impl UserContext {
    pub fn from_args(args: &[String]) -> Self {
        let mut out_args: Vec<UserArg> = Vec::new();
        let mut is_flag = false;
        let mut flag = "";
        let mut pos = 0;
        for arg in args {
            if arg.starts_with("-") {
                if is_flag {
                    out_args.push(UserArg::Flag(flag.to_string(), "".to_string()));
                }
                flag = &arg;
                is_flag = true;
            } else {
                if is_flag {
                    out_args.push(UserArg::Flag(flag.to_string(), arg.to_string()));
                    flag = "";
                    is_flag = false;
                } else {
                    out_args.push(UserArg::Position(pos, arg.to_string()));
                    pos += 1;
                }
            }
        }
        if is_flag {
            out_args.push(UserArg::Flag(flag.to_string(), "".to_string()));
        }
        Self { args: out_args }
    }
    pub fn position_args(&self) -> Vec<UserArg> {
        let mut pos_args: Vec<UserArg> = Vec::new();
        for a in &self.args {
            match a {
                UserArg::Position(_, _) => {
                    pos_args.push(a.clone());
                }
                UserArg::Flag(_, _) => {}
            }
        }
        pos_args
    }

    pub fn flags(&self) -> Vec<UserArg> {
        let mut flags: Vec<UserArg> = Vec::new();
        for f in &self.args {
            match f {
                UserArg::Position(_, _) => {}
                UserArg::Flag(_, _) => {
                    flags.push(f.clone());
                }
            }
        }
        flags
    }
}
