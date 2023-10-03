use std::{fs, path::Path};

use anyhow::{Context, Result};
use inquire::{required, Text};
use path_absolutize::Absolutize;
use path_slash::PathExt;
use relative_path::PathExt as RelativePathExt;

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

    for (name, (to_path, from_path)) in links {
        current += 1;

        let to_path = expand_tilde(to_path).unwrap();
        let from_path = {
            let resolved_path = expand_tilde(from_path).unwrap();
            ctx.dotfiles_path
                .join(resolved_path)
                .absolutize()?
                .to_path_buf()
        };

        fs::create_dir_all(to_path.parent().unwrap())?;
        let result = util::link(from_path, to_path);

        match result {
            Ok(_) => {
                println!(
                    "[{}/{}] ['{}']['{}'] is linked!",
                    current, total, args.profile, name
                );
            }
            Err(err) => {
                println!(
                    "[{}/{}] ['{}']['{}']: {}",
                    current, total, args.profile, name, err
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
        path.absolutize()?.to_path_buf()
    };
    let filepath = {
        let resolved_path = expand_tilde(&args.file_path).unwrap();
        resolved_path.absolutize()?.to_path_buf()
    };
    let filename = filepath.file_name().unwrap().to_str().unwrap();
    let filepath = filepath.parent().unwrap().join(filename);
    let savepath = savepath.join(filename);

    let filepath_str = filepath.to_slash().unwrap();
    let savepath_str = savepath.relative_to(&ctx.dotfiles_path)?.to_string();

    fs::create_dir_all(savepath.parent().unwrap())?;
    fs::rename(&filepath, &savepath)?;
    util::link(&savepath, &filepath)?;

    let home_dir = home::home_dir().unwrap().to_slash().unwrap().to_string();
    ctx.add_link(
        &profile,
        &dot_name,
        &filepath_str.replace(&home_dir, "~"),
        &savepath_str,
    );
    ctx.save()?;

    Ok(())
}
