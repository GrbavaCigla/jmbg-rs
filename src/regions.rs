pub fn country<'a>(code: u8) -> &'a str {
    match code / 10 {
        1 => "Bosnia and Herzegovina",
        2 => "Montenegro",
        3 => "Croatia",
        4 => "North Macedonia",
        5 => "Slovenia",
        7 => "Central Serbia",
        8 => "Vojvodina",
        9 => "Kosovo",
        _ => ""
    }
}

pub fn region<'a>(code: u8) -> &'a str {
    match code {
        0 => "naturalized citizen which had no republican citizenship",
        1 => "foreigner in Bosnia and Herzegovina",
        2 => "foreigner in Montenegro",
        3 => "foreigner in Croatia",
        4 => "foreigner in Macedonia",
        5 => "foreigner in Slovenia",
        6 => "foreigner in Central Serbia",
        7 => "foreigner in Vojvodina",
        8 => "foreigner in Kosovo",
        9 => "naturalized citizen which had no republican citizenship",
        10 => "Banja Luka",
        11 => "Bihać",
        12 => "Doboj",
        13 => "Goražde",
        14 => "Livno",
        15 => "Mostar",
        16 => "Prijedor",
        17 => "Sarajevo",
        18 => "Tuzla",
        19 => "Zenica",
        21 => "Podgorica, Danilovgrad, Kolašin",
        22 => "Bar, Ulcinj",
        23 => "Budva, Kotor, Tivat",
        24 => "Herceg Novi",
        25 => "Cetinje",
        26 => "Nikšić, Plužine, Šavnik",
        27 => "Berane, Rožaje, Plav, Andrijevica",
        28 => "Bijelo Polje, Mojkovac",
        29 => "Pljevlja, Žabljak",
        30 => "Osijek, Slavonia",
        31 => "Bjelovar, Virovitica, Koprivnica, Pakrac, Podravina",
        32 => "Varaždin, Međimurje",
        33 => "Zagreb",
        34 => "Karlovac, Kordun",
        35 => "Gospić, Lika",
        36 => "Rijeka, Pula, Gorski kotar, Istria and Croatian Littoral",
        37 => "Sisak, Banovina",
        38 => "Split, Zadar, Šibenik, Dubrovnik, Dalmatia",
        39 => "Hrvatsko Zagorje",
        41 => "Bitola",
        42 => "Kumanovo",
        43 => "Ohrid",
        44 => "Prilep",
        45 => "Skopje",
        46 => "Strumica",
        47 => "Tetovo",
        48 => "Veles",
        49 => "Štip",
        70 => "Serbian citizen registered abroad at a Serbian diplomatic/consular post",
        71 => "Belgrade",
        72 => "Šumadija and Pomoravlje",
        73 => "Niš",
        74 => "Southern Morava",
        75 => "Zaječar",
        76 => "Podunavlje",
        77 => "Podrinje and Kolubara",
        78 => "Kraljevo",
        79 => "Užice",
        80 => "Novi Sad",
        81 => "Sombor",
        82 => "Subotica",
        84 => "Kikinda",
        85 => "Zrenjanin",
        86 => "Pančevo",
        87 => "Vršac",
        88 => "Ruma",
        89 => "Sremska Mitrovica",
        91 => "Priština",
        92 => "Kosovska Mitrovica",
        93 => "Peć",
        94 => "Đakovica",
        95 => "Prizren",
        96 => "Gnjilane",
        _ => "",
    }
}
