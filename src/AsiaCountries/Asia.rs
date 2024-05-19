#[derive(Debug)]
pub enum Asia {
    CentralAsia(CentralAsia),
    EasternAsia(EasternAsia),
    SouthernAsia(SouthernAsia),
    SouthEasternAsia(SouthEasternAsia),
    WesternAsia(WesternAsia),
}

#[derive(Debug)]
pub enum CentralAsia {
    Kazakhstan,
    Kyrgzstan,
    Tajikistan,
    Turkmenistan,
    Uzbekistan,
}

#[derive(Debug)]
pub enum EasternAsia {
    China,
    ChinaHongKong,
    ChinaMacao,
    Japan,
    Mongolia,
    NorthKorea,
    SouthKorea,
}

#[derive(Debug)]
pub enum SouthernAsia {
    Afghanistan,
    Bangladesh,
    Bhutan,
    India,
    Iran,
    Maldives,
    Nepal,
    Pakistan,
    SriLanka,
}

#[derive(Debug)]
pub enum SouthEasternAsia {
    Brunei,
    Cambodia,
    Indonesia,
    Laos,
    Malaysia,
    Myanmar,
    Philippines,
    Singapore,
    Thailand,
    TimorLeste,
    VietNam,
}

#[derive(Debug)]
pub enum WesternAsia {
    Armenia,
    Azerbaijan,
    Bahrain,
    Cyprus,
    Georgia,
    Iraq,
    Israel,
    Jordan,
    Kuwait,
    Lebanon,
    Oman,
    Qatar,
    SaudiArabia,
    Palestine,
    SyrianArabRepublic,
    Turkey,
    UnitedArabEmirates,
}
