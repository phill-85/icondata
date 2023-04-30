
use leptos_icons::*;
use strum::IntoEnumIterator;

fn all_icons() -> impl Iterator<Item = IconData> {
    [
        AiIcon::iter(),
        FaIcon::iter(),
        WiIcon::iter(),
        FiIcon::iter(),
        VsIcon::iter(),
        BsIcon::iter(),
        BiIcon::iter(),
        ImIcon::iter(),
        IoIcon::iter(),
        RiIcon::iter(),
        SiIcon::iter(),
        TiIcon::iter(),
        HiIcon::iter(),
        CgIcon::iter(),
        TbIcon::iter(),
        OcIcon::iter(),
        LuIcon::iter(),
    ]
}
