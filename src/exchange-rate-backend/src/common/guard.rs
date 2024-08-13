use ic_cdk::caller;
use super::constant::{ GOVERNANCE_YINDR, GOVERNANCE_ZHOU};

pub fn admin_guard() -> Result<(), String> {
    let caller = caller().to_string();
    if GOVERNANCE_ZHOU == caller
      || GOVERNANCE_YINDR == caller {
      Ok(())
    } else {
      Err("caller not admin ".to_string())
    }
  }