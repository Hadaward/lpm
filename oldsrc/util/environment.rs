use std::error::Error;

#[cfg(target_os = "windows")]
pub fn set_env_var(name: &str, value: &str) -> Result<(), Box<dyn Error>> {
    use winreg::{enums::*, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap();
    env.set_value(name, &value).unwrap();
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn get_env_var(name: &str) -> Result<String, Box<dyn Error>> {
    use winreg::{enums::*, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap();
    let value = env.get_value(name);

    if value.is_err() {
        let error = value.unwrap_err();
        return Err(Box::new(error));
    }
    Ok(value.unwrap())
}
