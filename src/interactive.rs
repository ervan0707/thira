use crate::templates::ProjectTemplate;
use colored::*;
use inquire::{Confirm, Select};

pub struct SetupWizard;

impl SetupWizard {
    pub fn run() -> Result<ProjectTemplate, Box<dyn std::error::Error>> {
        println!("{}", "🎉 Welcome to Thira Setup Wizard!".cyan().bold());
        println!(
            "{}",
            "Let's configure your Git hooks for this project.".white()
        );
        println!();

        // Project type selection
        let templates = ProjectTemplate::all();
        let options: Vec<String> = templates
            .iter()
            .map(|(_, name, description)| format!("{:<15} - {}", name, description))
            .collect();

        let selection = Select::new("What type of project are you setting up?", options)
            .with_help_message(
                "This will configure appropriate hooks and scripts for your project type",
            )
            .prompt()?;

        let selected_index = templates
            .iter()
            .enumerate()
            .find(|(_, (_, name, _))| selection.starts_with(name))
            .map(|(index, _)| index)
            .unwrap_or(0);

        let (template, template_name, _) = &templates[selected_index];

        println!();
        println!("{}", format!("✓ Selected: {}", template_name).green());

        // Show what will be configured
        Self::preview_configuration(template);

        // Confirmation
        let confirm = Confirm::new("Do you want to proceed with this configuration?")
            .with_default(true)
            .prompt()?;

        if !confirm {
            println!("{}", "Setup cancelled.".yellow());
            std::process::exit(0);
        }

        Ok(template.clone())
    }

    fn preview_configuration(template: &ProjectTemplate) {
        println!();
        println!("{}", "📋 Configuration Preview:".blue().bold());

        let config = template.to_config();

        // Show hooks
        println!("{}", "   Git Hooks:".yellow());
        for (hook_name, hooks) in &config.hooks {
            println!("     • {}", hook_name.cyan());
            for hook in hooks {
                if hook.command == "${thira}" {
                    println!("       └─ Commit message validation");
                } else {
                    let args_str = if hook.args.is_empty() {
                        String::new()
                    } else {
                        format!(" {}", hook.args.join(" "))
                    };
                    println!("       └─ {}{}", hook.command, args_str);
                }
            }
        }

        // Show scripts
        if !config.scripts.is_empty() {
            println!("{}", "   Custom Scripts:".yellow());
            for (script_name, _) in &config.scripts {
                println!("     • {}", script_name.cyan());
            }
        }

        // Show linter config
        println!("{}", "   Commit Message Linting:".yellow());
        println!("     • Conventional Commits enforced");
        println!("     • {} allowed types", config.lint.types.len());
        println!("     • {} predefined scopes", config.lint.scopes.len());

        println!();
    }

    pub fn show_success_message(template_name: &str) {
        println!();
        println!("{}", "🎉 Setup Complete!".green().bold());
        println!();
        println!(
            "{}",
            format!("✓ {} configuration created successfully", template_name).green()
        );
        println!("{}", "✓ hooks.yaml file generated".green());
        println!();
        println!("{}", "Next steps:".blue().bold());
        println!("  1. Review the generated {} file", "hooks.yaml".cyan());
        println!(
            "  2. Run {} to install the hooks",
            "thira hooks install".cyan()
        );
        println!(
            "  3. Run {} to see available scripts",
            "thira scripts list".cyan()
        );
        println!();
        println!(
            "{}",
            "💡 Tip: You can customize the configuration by editing hooks.yaml".yellow()
        );
    }
}
