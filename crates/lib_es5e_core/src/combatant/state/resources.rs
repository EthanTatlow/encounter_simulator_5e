use std::collections::BTreeMap;

use super::{resource_config::ResourceKey, ResourceCfg};

pub(super) type Resources = BTreeMap<ResourceKey, Resource>;

#[derive(Clone, Debug)]
pub(super) struct Resource {
    pub(super) charges: u32,
    pub(super) max_charges: u32,
}

impl From<ResourceCfg> for Resource {
    fn from(value: ResourceCfg) -> Self {
        Self {
            charges: value.charges,
            max_charges: value.max_charges,
        }
    }
}

pub(super) fn reset_charge_to_max(entries: &mut Resources) {
    entries
        .iter_mut()
        .for_each(|(_, resource)| resource.charges = resource.max_charges);
}
