use std::{convert::Infallible, path::Path, str::FromStr};

use tokio::fs::{read_to_string, write};
use tracing::error;

use crate::result::{AftmanError, AftmanResult};

/**
    Loads the given type from the file at the given path.

    Will return an error if the file does not exist or could not be parsed.
*/
pub(crate) async fn load_from_file_fallible<P, T, E>(path: P) -> AftmanResult<T>
where
    P: AsRef<Path>,
    T: FromStr<Err = E>,
    E: Into<AftmanError>,
{
    let path = path.as_ref();
    match read_to_string(path).await {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(AftmanError::FileNotFound(path.into()))
        }
        Err(e) => Err(e.into()),
        Ok(s) => match s.parse() {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into()),
        },
    }
}

/**
    Loads the given type from the file at the given path.

    If the file does not exist, it will be created with
    the default stringified contents of the type.
*/
pub(crate) async fn load_from_file<P, T>(path: P) -> AftmanResult<T>
where
    P: AsRef<Path>,
    T: Default + FromStr<Err = Infallible> + ToString,
{
    let path = path.as_ref();
    match read_to_string(path).await {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let new: T = Default::default();
            write(path, new.to_string()).await?;
            Ok(new)
        }
        Err(e) => Err(e.into()),
        Ok(s) => Ok(s.parse().unwrap()),
    }
}

/**
    Saves the given data, stringified, to the file at the given path.
*/
pub(crate) async fn save_to_file<P, T>(path: P, data: T) -> AftmanResult<()>
where
    P: AsRef<Path>,
    T: Clone + ToString,
{
    let path = path.as_ref();
    write(path, data.to_string()).await?;
    Ok(())
}

/**
    Writes the given contents to the file at the
    given path, and adds executable permissions to it.
*/
pub async fn write_executable_file(
    path: impl AsRef<Path>,
    contents: impl AsRef<[u8]>,
) -> AftmanResult<()> {
    let path = path.as_ref();

    if let Err(e) = write(path, contents).await {
        error!("Failed to write executable to {path:?}:\n{e}");
        return Err(e.into());
    }

    add_executable_permissions(path).await?;

    Ok(())
}

/**
    Writes a symlink at the given link path to the given
    target path, and sets the symlink to be executable.

    # Panics

    This function will panic if called on a non-unix system.
*/
#[cfg(unix)]
pub async fn write_executable_link(
    link_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
) -> AftmanResult<()> {
    use tokio::fs::{remove_file, symlink};

    let link_path = link_path.as_ref();
    let target_path = target_path.as_ref();

    // NOTE: If a symlink already exists, we may need to remove it
    // for the new symlink to be created successfully - the only error we
    // should be able to get here is if the file doesn't exist, which is fine.
    remove_file(link_path).await.ok();

    if let Err(e) = symlink(target_path, link_path).await {
        error!("Failed to create symlink at {link_path:?}:\n{e}");
        return Err(e.into());
    }

    // NOTE: We set the permissions of the symlink itself only on macOS
    // since that is the only supported OS where symlink permissions matter
    #[cfg(target_os = "macos")]
    {
        add_executable_permissions(link_path).await?;
    }

    Ok(())
}

/**
    Writes a symlink at the given link path to the given
    target path, and sets the symlink to be executable.

    # Panics

    This function will panic if called on a non-unix system.
*/
#[cfg(not(unix))]
pub async fn write_executable_link(
    _link_path: impl AsRef<Path>,
    _target_path: impl AsRef<Path>,
) -> AftmanResult<()> {
    panic!("write_executable_link should only be called on unix systems");
}

#[cfg(unix)]
async fn add_executable_permissions(path: impl AsRef<Path>) -> AftmanResult<()> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;
    use tokio::fs::set_permissions;

    let path = path.as_ref();
    if let Err(e) = set_permissions(path, Permissions::from_mode(0o755)).await {
        error!("Failed to set executable permissions on {path:?}:\n{e}");
        return Err(e.into());
    }

    Ok(())
}

#[cfg(not(unix))]
async fn set_executable_permissions(_path: impl AsRef<Path>) -> AftmanResult<()> {
    Ok(())
}