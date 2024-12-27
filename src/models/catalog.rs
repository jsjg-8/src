/*
 * Kill Bill
 *
 * Kill Bill is an open-source billing and payments platform
 *
 * The version of the OpenAPI document: 0.24.10
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "effectiveDate", skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "currencies", skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<Currencies>>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<models::Unit>>,
    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<models::Product>>,
    #[serde(rename = "priceLists", skip_serializing_if = "Option::is_none")]
    pub price_lists: Option<Vec<models::PriceList>>,
}

impl Catalog {
    pub fn new() -> Catalog {
        Catalog {
            name: None,
            effective_date: None,
            currencies: None,
            units: None,
            products: None,
            price_lists: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currencies {
    #[serde(rename = "AED")]
    Aed,
    #[serde(rename = "AFN")]
    Afn,
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "AMD")]
    Amd,
    #[serde(rename = "ANG")]
    Ang,
    #[serde(rename = "AOA")]
    Aoa,
    #[serde(rename = "ARS")]
    Ars,
    #[serde(rename = "AUD")]
    Aud,
    #[serde(rename = "AWG")]
    Awg,
    #[serde(rename = "AZN")]
    Azn,
    #[serde(rename = "BAM")]
    Bam,
    #[serde(rename = "BBD")]
    Bbd,
    #[serde(rename = "BDT")]
    Bdt,
    #[serde(rename = "BGN")]
    Bgn,
    #[serde(rename = "BHD")]
    Bhd,
    #[serde(rename = "BIF")]
    Bif,
    #[serde(rename = "BMD")]
    Bmd,
    #[serde(rename = "BND")]
    Bnd,
    #[serde(rename = "BOB")]
    Bob,
    #[serde(rename = "BRL")]
    Brl,
    #[serde(rename = "BSD")]
    Bsd,
    #[serde(rename = "BTN")]
    Btn,
    #[serde(rename = "BWP")]
    Bwp,
    #[serde(rename = "BYR")]
    Byr,
    #[serde(rename = "BZD")]
    Bzd,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "CDF")]
    Cdf,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "CLP")]
    Clp,
    #[serde(rename = "CNY")]
    Cny,
    #[serde(rename = "COP")]
    Cop,
    #[serde(rename = "CRC")]
    Crc,
    #[serde(rename = "CUC")]
    Cuc,
    #[serde(rename = "CUP")]
    Cup,
    #[serde(rename = "CVE")]
    Cve,
    #[serde(rename = "CZK")]
    Czk,
    #[serde(rename = "DJF")]
    Djf,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "DOP")]
    Dop,
    #[serde(rename = "DZD")]
    Dzd,
    #[serde(rename = "EGP")]
    Egp,
    #[serde(rename = "ERN")]
    Ern,
    #[serde(rename = "ETB")]
    Etb,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "FJD")]
    Fjd,
    #[serde(rename = "FKP")]
    Fkp,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "GEL")]
    Gel,
    #[serde(rename = "GGP")]
    Ggp,
    #[serde(rename = "GHS")]
    Ghs,
    #[serde(rename = "GIP")]
    Gip,
    #[serde(rename = "GMD")]
    Gmd,
    #[serde(rename = "GNF")]
    Gnf,
    #[serde(rename = "GTQ")]
    Gtq,
    #[serde(rename = "GYD")]
    Gyd,
    #[serde(rename = "HKD")]
    Hkd,
    #[serde(rename = "HNL")]
    Hnl,
    #[serde(rename = "HRK")]
    Hrk,
    #[serde(rename = "HTG")]
    Htg,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "IDR")]
    Idr,
    #[serde(rename = "ILS")]
    Ils,
    #[serde(rename = "IMP")]
    Imp,
    #[serde(rename = "INR")]
    Inr,
    #[serde(rename = "IQD")]
    Iqd,
    #[serde(rename = "IRR")]
    Irr,
    #[serde(rename = "ISK")]
    Isk,
    #[serde(rename = "JEP")]
    Jep,
    #[serde(rename = "JMD")]
    Jmd,
    #[serde(rename = "JOD")]
    Jod,
    #[serde(rename = "JPY")]
    Jpy,
    #[serde(rename = "KES")]
    Kes,
    #[serde(rename = "KGS")]
    Kgs,
    #[serde(rename = "KHR")]
    Khr,
    #[serde(rename = "KMF")]
    Kmf,
    #[serde(rename = "KPW")]
    Kpw,
    #[serde(rename = "KRW")]
    Krw,
    #[serde(rename = "KWD")]
    Kwd,
    #[serde(rename = "KYD")]
    Kyd,
    #[serde(rename = "KZT")]
    Kzt,
    #[serde(rename = "LAK")]
    Lak,
    #[serde(rename = "LBP")]
    Lbp,
    #[serde(rename = "LKR")]
    Lkr,
    #[serde(rename = "LRD")]
    Lrd,
    #[serde(rename = "LSL")]
    Lsl,
    #[serde(rename = "LTL")]
    Ltl,
    #[serde(rename = "LVL")]
    Lvl,
    #[serde(rename = "LYD")]
    Lyd,
    #[serde(rename = "MAD")]
    Mad,
    #[serde(rename = "MDL")]
    Mdl,
    #[serde(rename = "MGA")]
    Mga,
    #[serde(rename = "MKD")]
    Mkd,
    #[serde(rename = "MMK")]
    Mmk,
    #[serde(rename = "MNT")]
    Mnt,
    #[serde(rename = "MOP")]
    Mop,
    #[serde(rename = "MRO")]
    Mro,
    #[serde(rename = "MUR")]
    Mur,
    #[serde(rename = "MVR")]
    Mvr,
    #[serde(rename = "MWK")]
    Mwk,
    #[serde(rename = "MXN")]
    Mxn,
    #[serde(rename = "MYR")]
    Myr,
    #[serde(rename = "MZN")]
    Mzn,
    #[serde(rename = "NAD")]
    Nad,
    #[serde(rename = "NGN")]
    Ngn,
    #[serde(rename = "NIO")]
    Nio,
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "NPR")]
    Npr,
    #[serde(rename = "NZD")]
    Nzd,
    #[serde(rename = "OMR")]
    Omr,
    #[serde(rename = "PAB")]
    Pab,
    #[serde(rename = "PEN")]
    Pen,
    #[serde(rename = "PGK")]
    Pgk,
    #[serde(rename = "PHP")]
    Php,
    #[serde(rename = "PKR")]
    Pkr,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "PYG")]
    Pyg,
    #[serde(rename = "QAR")]
    Qar,
    #[serde(rename = "RON")]
    Ron,
    #[serde(rename = "RSD")]
    Rsd,
    #[serde(rename = "RUB")]
    Rub,
    #[serde(rename = "RWF")]
    Rwf,
    #[serde(rename = "SAR")]
    Sar,
    #[serde(rename = "SBD")]
    Sbd,
    #[serde(rename = "SCR")]
    Scr,
    #[serde(rename = "SDG")]
    Sdg,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "SGD")]
    Sgd,
    #[serde(rename = "SHP")]
    Shp,
    #[serde(rename = "SLL")]
    Sll,
    #[serde(rename = "SOS")]
    Sos,
    #[serde(rename = "SPL")]
    Spl,
    #[serde(rename = "SRD")]
    Srd,
    #[serde(rename = "STD")]
    Std,
    #[serde(rename = "SVC")]
    Svc,
    #[serde(rename = "SYP")]
    Syp,
    #[serde(rename = "SZL")]
    Szl,
    #[serde(rename = "THB")]
    Thb,
    #[serde(rename = "TJS")]
    Tjs,
    #[serde(rename = "TMT")]
    Tmt,
    #[serde(rename = "TND")]
    Tnd,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "TRY")]
    Try,
    #[serde(rename = "TTD")]
    Ttd,
    #[serde(rename = "TVD")]
    Tvd,
    #[serde(rename = "TWD")]
    Twd,
    #[serde(rename = "TZS")]
    Tzs,
    #[serde(rename = "UAH")]
    Uah,
    #[serde(rename = "UGX")]
    Ugx,
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "UYU")]
    Uyu,
    #[serde(rename = "UZS")]
    Uzs,
    #[serde(rename = "VEF")]
    Vef,
    #[serde(rename = "VND")]
    Vnd,
    #[serde(rename = "VUV")]
    Vuv,
    #[serde(rename = "WST")]
    Wst,
    #[serde(rename = "XAF")]
    Xaf,
    #[serde(rename = "XCD")]
    Xcd,
    #[serde(rename = "XDR")]
    Xdr,
    #[serde(rename = "XOF")]
    Xof,
    #[serde(rename = "XPF")]
    Xpf,
    #[serde(rename = "YER")]
    Yer,
    #[serde(rename = "ZAR")]
    Zar,
    #[serde(rename = "ZMW")]
    Zmw,
    #[serde(rename = "ZWD")]
    Zwd,
    #[serde(rename = "BTC")]
    Btc,
}

impl Default for Currencies {
    fn default() -> Currencies {
        Self::Aed
    }
}

