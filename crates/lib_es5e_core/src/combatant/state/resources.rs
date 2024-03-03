#[derive(Clone, Debug)]
pub(super) struct Resource {
    pub(super) name: String,
    pub(super) charges: u32,
    pub(super) max_charges: u32,
}

pub(super) fn reset_charge_to_max(entries: &mut [Resource]) {
    entries
        .iter_mut()
        .for_each(|resource| resource.charges = resource.max_charges);
}
