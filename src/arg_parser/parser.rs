pub enum Query {
    RNDebugger,
}

pub struct QueryAndArgs {
    pub query: Query,
    pub args: Vec<String>,
}

pub fn parse_args(args: Vec<String>) -> Result<QueryAndArgs, String> {
    if args.len() < 2 {
        return Err("Please provide a command".to_string());
    }

    let command = &args[1];
    let args = args[2..].to_vec();

    match command.as_str() {
        "rn-debugger" => {
            return Ok(QueryAndArgs {
                query: Query::RNDebugger,
                args,
            });
        },
        _ => {
            return Err("Command not found".to_string());
        }
    }
}
