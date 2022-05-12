use clap::Parser;

use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;

/// == Gradle Profiler Director ==
/// A simple program to generate gradle profiler scenarios
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Min Xmx
    #[clap(long, default_value_t = 2)]
    min: u8,

    /// Max Xmx
    #[clap(long, default_value_t = 8)]
    max: u8,

    /// Step from min to max
    #[clap(short, long, default_value_t = 1)]
    step: u8,

    /// Template pathw
    #[clap(short)]
    template: Option<std::path::PathBuf>,

    /// Gradle task
    #[clap(long, default_value = "assembleRelease")]
    task: String,
}

const DEFAULT_TEMPLATE: &str = r#"
{{task_name}} {
    title = "{{task_name}}-Xmx:{{max}}-Xms:{{min}}"
    tasks = ["{{task}}"]
    jvm-args = ["-Xmx{{max}}g", "-Xmx{{min}}g"]
}
"#;

fn main() {
    let args: CliArgs = CliArgs::parse();

    let mut reg = Handlebars::new();
    let mut data = HashMap::new();

    match reg.register_template_string("template", DEFAULT_TEMPLATE) {
        Err(why) => println!("Error registering template {}", why),
        _ => {}
    };

    let mut current_xmx = args.min;
    let mut scenario_list: Vec<String> = Vec::new();

    while current_xmx <= args.max {
        let task = args.task.to_owned();
        data.insert("task", task.to_owned());
        data.insert("task_name", format!("{}", task));
        data.insert("max", current_xmx.to_string());
        data.insert("min", args.min.to_string());

        scenario_list.push(create_scenario(&reg, &data));

        current_xmx += args.step
    }

    let all_scenario = scenario_list.join("\n");
    let result = fs::write("generated.scenarios", all_scenario);

    match result {
        Ok(_) => println!("Scenarios generated!"),
        Err(why) => println!("Error: {}", why)
    }
}

fn create_scenario(
    reg: &Handlebars,
    data: &HashMap<&str, String>,
) -> String {
    let result = reg.render("template", &data);
    return result.unwrap();
}
