use crate::{multi_error::MultiError, utils::run_command};

pub fn ip_link_add(dev_name: &String, ifc_type: &String) -> Result<(), MultiError> {
    let _out = run_command(
        "ip",
        &vec!["link", "add", "dev", dev_name, "type", ifc_type],
        None,
    )?;
    // todo: verify that the link was created by getting a list of interfaces and making sure it is in there

    Ok(())
}

pub fn ip_addr_add(dev_name: &String, address: &String) -> Result<(), MultiError> {
    let _out = run_command("ip", &vec!["addr", "add", address, "dev", dev_name], None)?;
    // todo: verify that the address was added by getting a list of addresses for the interface and verifying it is contained.
    Ok(())
}

pub fn ip_link_set_up(dev_name: &String) -> Result<(), MultiError> {
    let _out = run_command("ip", &vec!["link", "set", dev_name, "up"], None)?;
    // todo: verify that the link is now up
    Ok(())
}
