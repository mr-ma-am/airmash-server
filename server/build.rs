
extern crate phf_codegen;

mod flags {
    use std::env;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::Path;
    use phf_codegen;

    const KEY_TABLE: [[&'static str; 2]; 125] = [
        [ "SY", "FlagCode::SyrianArabRepublic" ],
        [ "TH", "FlagCode::Thailand" ],
        [ "TM", "FlagCode::Turkmenistan" ],
        [ "TN", "FlagCode::Tunisia" ],
        [ "TR", "FlagCode::Turkey" ],
        [ "TT", "FlagCode::TrinidadandTobago" ],
        [ "TW", "FlagCode::Taiwan" ],
        [ "TZ", "FlagCode::Tanzania" ],
        [ "UA", "FlagCode::Ukraine" ],
        [ "UN", "FlagCode::UnitedNations" ],
        [ "US", "FlagCode::UnitedStates" ],
        [ "UY", "FlagCode::Uruguay" ],
        [ "UZ", "FlagCode::Uzbekistan" ],
        [ "VE", "FlagCode::Venezuela" ],
        [ "VN", "FlagCode::VietNam" ],
        [ "PR", "FlagCode::PuertoRico" ],
        [ "PT", "FlagCode::Portugal" ],
        [ "PY", "FlagCode::Paraguay" ],
        [ "QA", "FlagCode::Qatar" ],
        [ "RAINBOW", "FlagCode::Rainbow" ],
        [ "RO", "FlagCode::Romania" ],
        [ "RS", "FlagCode::Serbia" ],
        [ "RU", "FlagCode::RussianFederation" ],
        [ "SA", "FlagCode::SaudiArabia" ],
        [ "SE", "FlagCode::Sweden" ],
        [ "SG", "FlagCode::Singapore" ],
        [ "SI", "FlagCode::Slovenia" ],
        [ "SK", "FlagCode::Slovakia" ],
        [ "SM", "FlagCode::SanMarino" ],
        [ "MK", "FlagCode::Macedonia" ],
        [ "MO", "FlagCode::Macao" ],
        [ "MT", "FlagCode::Malta" ],
        [ "MX", "FlagCode::Mexico" ],
        [ "MY", "FlagCode::Malaysia" ],
        [ "NG", "FlagCode::Nigeria" ],
        [ "NL", "FlagCode::Netherlands" ],
        [ "NO", "FlagCode::Norway" ],
        [ "NP", "FlagCode::Nepal" ],
        [ "NZ", "FlagCode::NewZealand" ],
        [ "OM", "FlagCode::Oman" ],
        [ "PA", "FlagCode::Panama" ],
        [ "PE", "FlagCode::Peru" ],
        [ "JP", "FlagCode::Japan" ],
        [ "KP", "FlagCode::DPRK" ],
        [ "KR", "FlagCode::SouthKorea" ],
        [ "KW", "FlagCode::Kuwait" ],
        [ "KZ", "FlagCode::Kazakhstan" ],
        [ "LB", "FlagCode::Lebanon" ],
        [ "LI", "FlagCode::Liechtenstein" ],
        [ "LK", "FlagCode::SriLanka" ],
        [ "LT", "FlagCode::Lithuania" ],
        [ "LU", "FlagCode::Luxembourg" ],
        [ "LV", "FlagCode::Latvia" ],
        [ "HN", "FlagCode::Honduras" ],
        [ "HR", "FlagCode::Croatia" ],
        [ "HU", "FlagCode::Hungary" ],
        [ "ID", "FlagCode::Indonesia" ],
        [ "IE", "FlagCode::Ireland" ],
        [ "IL", "FlagCode::Israel" ],
        [ "IM", "FlagCode::IsleofMan" ],
        [ "IMPERIAL", "FlagCode::ImperialJapan" ],
        [ "IN", "FlagCode::India" ],
        [ "IQ", "FlagCode::Iraq" ],
        [ "DE", "FlagCode::Germany" ],
        [ "DK", "FlagCode::Denmark" ],
        [ "DO", "FlagCode::DominicanRepublic" ],
        [ "DZ", "FlagCode::Algeria" ],
        [ "EC", "FlagCode::Ecuador" ],
        [ "EE", "FlagCode::Estonia" ],
        [ "EG", "FlagCode::Egypt" ],
        [ "ES", "FlagCode::Spain" ],
        [ "EU", "FlagCode::EuropeanUnion" ],
        [ "BH", "FlagCode::Bahrain" ],
        [ "BO", "FlagCode::Bolivia" ],
        [ "BR", "FlagCode::Brazil" ],
        [ "BT", "FlagCode::Bhutan" ],
        [ "BY", "FlagCode::Belarus" ],
        [ "CA", "FlagCode::Canada" ],
        [ "CH", "FlagCode::Switzerland" ],
        [ "AD", "FlagCode::Andorra" ],
        [ "AE", "FlagCode::UnitedArabEmirates" ],
        [ "AL", "FlagCode::Albania" ],
        [ "AM", "FlagCode::Armenia" ],
        [ "CL", "FlagCode::Chile" ],
        [ "AQ", "FlagCode::Antarctica" ],
        [ "CN", "FlagCode::China" ],
        [ "AR", "FlagCode::Argentina" ],
        [ "FI", "FlagCode::Finland" ],
        [ "CO", "FlagCode::Colombia" ],
        [ "AT", "FlagCode::Austria" ],
        [ "IR", "FlagCode::Iran" ],
        [ "FR", "FlagCode::France" ],
        [ "COMMUNIST", "FlagCode::Communist" ],
        [ "AU", "FlagCode::Australia" ],
        [ "LY", "FlagCode::LibyanArabJamahiriya" ],
        [ "IS", "FlagCode::Iceland" ],
        [ "GB", "FlagCode::UnitedKingdom" ],
        [ "CONFEDERATE", "FlagCode::Confederate" ], 
        [ "AZ", "FlagCode::Azerbaijan" ],
        [ "MA", "FlagCode::Morocco" ],
        [ "IT", "FlagCode::Italy" ],
        [ "GE", "FlagCode::Georgia" ],
        [ "CR", "FlagCode::CostaRica" ],
        [ "BA", "FlagCode::BosniaandHerzegovina" ],
        [ "PH", "FlagCode::Philippines" ],
        [ "MC", "FlagCode::Monaco" ],
        [ "JM", "FlagCode::Jamaica" ],
        [ "GR", "FlagCode::Greece" ],
        [ "CU", "FlagCode::Cuba" ],
        [ "BD", "FlagCode::Bangladesh" ],
        [ "SO", "FlagCode::Somalia" ],
        [ "PK", "FlagCode::Pakistan" ],
        [ "MD", "FlagCode::Moldova" ],
        [ "JO", "FlagCode::Jordan" ],
        [ "GT", "FlagCode::Guatemala" ],
        [ "CY", "FlagCode::Cyprus" ],
        [ "BE", "FlagCode::Belgium" ],
        [ "ZA", "FlagCode::SouthAfrica" ],
        [ "SV", "FlagCode::ElSalvador" ],
        [ "PL", "FlagCode::Poland" ],
        [ "ME", "FlagCode::Montenegro" ],
        [ "JOLLY", "FlagCode::JollyRogers" ],
        [ "HK", "FlagCode::HongKong" ],
        [ "CZ", "FlagCode::CzechRepublic" ],
        [ "BG", "FlagCode::Bulgaria" ],
    ];	

    pub fn write() {
        let path = Path::new(&env::var("OUT_DIR").unwrap()).join("flags-phf.rs");
        let mut file = BufWriter::new(File::create(&path).unwrap());

        write!(&mut file, "use phf;").unwrap();
        write!(&mut file, "static FLAG_MAP: phf::Map<&'static str, FlagCode> =").unwrap();
        let mut map = phf_codegen::Map::new();

        for entry in KEY_TABLE.iter() {
            map.entry(entry[0], entry[1]);
        }

        map.build(&mut file).unwrap();
        write!(&mut file, ";\n").unwrap();
    }
}

mod protocol {
    
}

fn main() {
    flags::write();
}
    