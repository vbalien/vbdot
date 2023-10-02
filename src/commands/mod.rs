use std::{
    fs,
    path::{self, Path},
};

use anyhow::{Context, Result};
use inquire::{required, Text};
use relative_path::PathExt;

use crate::{
    cli::{AddArgs, LinkArgs},
    config::Config,
    ctx::Ctx,
    util::{self, expand_tilde},
};

pub fn link(args: &LinkArgs, ctx: &Ctx) -> Result<()> {
    let config: Config = toml_edit::de::from_document(ctx.config.clone())?;
    let profile = config
        .get(&args.profile)
        .context(format!("Profile '{}' not found!", args.profile))?;

    let links = profile.link.iter();
    let mut current = 0;
    let total = links.clone().count();

    for (filename, (to_path, from_path)) in links {
        current += 1;

        let to_path = expand_tilde(to_path).unwrap();
        let from_path = {
            let resolved_path = expand_tilde(from_path).unwrap();
            path::absolute(ctx.dotfiles_path.join(resolved_path))?
        };

        let result = util::link(from_path, to_path);

        match result {
            Ok(_) => {
                println!(
                    "[{}/{}] ['{}']['{}'] is linked!",
                    current, total, args.profile, filename
                );
            }
            Err(err) => {
                println!(
                    "[{}/{}] ['{}']['{}']: {}",
                    current, total, args.profile, filename, err
                );
            }
        }
    }

    Ok(())
}

pub fn add(args: &AddArgs, ctx: &mut Ctx) -> Result<()> {
    let profile = Text::new("Which profile?")
        .with_validator(required!("No empty!"))
        .prompt()?;
    let dot_name = Text::new("name?")
        .with_validator(required!("No empty!"))
        .with_default(
            Path::new(&args.file_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .prompt()?;
    let savepath = {
        let path = Text::new("Where do you want to save it in the dotfile?")
            .with_default(&format!("/{}", profile))
            .prompt()?;
        let path = ctx
            .dotfiles_path
            .join(path.strip_prefix('/').unwrap_or(&path));
        path::absolute(path)?.join(dot_name)
    };
    let filepath = {
        let resolved_path = expand_tilde(&args.file_path).unwrap();
        path::absolute(resolved_path)?
    };
    let filename = filepath.file_name().unwrap().to_str().unwrap();
    let filepath = filepath.parent().unwrap().join(filename);

    let filepath_str = filepath.to_str().unwrap().to_owned();
    let savepath_str = savepath.relative_to(&ctx.dotfiles_path)?.to_string();

    fs::create_dir_all(savepath.parent().unwrap()).context("create_dir_all")?;
    fs::rename(&filepath, &savepath).context("rename")?;
    util::link(&savepath, &filepath).context("link")?;

    ctx.add_link(
        &profile,
        filename,
        &filepath_str.replace(home::home_dir().unwrap().to_str().unwrap(), "~"),
        &savepath_str,
    );
    ctx.save()?;

    Ok(())
}
