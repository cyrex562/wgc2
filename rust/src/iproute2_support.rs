use crate::{multi_error::MultiError, utils::run_command};

pub fn ip_link_add(dev_name: &str, ifc_type: &str) -> Result<(), MultiError> {
    let _out = run_command(
        "ip",
        &vec!["link", "add", "dev", dev_name, "type", ifc_type],
        None,
    )?;
    // todo: verify that the link was created by getting a list of interfaces and making sure it is in there

    Ok(())
}

pub fn ip_link_del(dev_name: &str) -> Result<(), MultiError> {
    let _out = run_command("ip", &vec!["link", "del", "dev", dev_name], None)?;
    // todo: verify that the link was created by getting a list of interfaces and making sure it is in there

    Ok(())
}

pub fn ip_addr_add(dev_name: &str, address: &str) -> Result<(), MultiError> {
    log::debug!("adding address={} to device={}", address, dev_name);
    let _out = run_command("ip", &vec!["addr", "add", address, "dev", dev_name], None)?;
    // todo: verify that the address was added by getting a list of addresses for the interface and verifying it is contained.
    Ok(())
}

pub fn ip_link_set_up(dev_name: &str) -> Result<(), MultiError> {
    log::debug!("setting dev={} link state to \"up\"", dev_name);
    let _out = run_command("ip", &vec!["link", "set", dev_name, "up"], None)?;
    // todo: verify that the link is now up
    Ok(())
}

pub fn ip_link_set_down(dev_name: &str) -> Result<(), MultiError> {
    log::debug!("setting dev={} link state to \"down\"", dev_name);
    let _out = run_command("ip", &vec!["link", "set", dev_name, "up"], None)?;
    // todo: verify that the link is now up
    Ok(())
}
