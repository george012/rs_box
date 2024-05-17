use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HashRateUnitFormat {
    Hs,  // H/s 默认值
    KHs, // KH/s
    MHs, // MH/s
    GHs, // GH/s
    THs, // TH/s
    PHs, // PH/s
    EHs, // EH/s
    ZHs, // ZH/s
    YHs, // YH/s
}

impl fmt::Display for HashRateUnitFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashRateUnitFormat::Hs => write!(f, "H/s"),
            HashRateUnitFormat::KHs => write!(f, "KH/s"),
            HashRateUnitFormat::MHs => write!(f, "MH/s"),
            HashRateUnitFormat::GHs => write!(f, "GH/s"),
            HashRateUnitFormat::THs => write!(f, "TH/s"),
            HashRateUnitFormat::PHs => write!(f, "PH/s"),
            HashRateUnitFormat::EHs => write!(f, "EH/s"),
            HashRateUnitFormat::ZHs => write!(f, "ZH/s"),
            HashRateUnitFormat::YHs => write!(f, "YH/s"),
        }
    }
}

pub struct GTHashRate {
    pub value: String,
    pub unit_str: String,
    pub unit_flag: HashRateUnitFormat,
}

impl GTHashRate {
    fn new(value: String, unit_str: String, unit_flag: HashRateUnitFormat) -> Self {
        Self {
            value,
            unit_str,
            unit_flag,
        }
    }
}

/// 算力单位自动格式化  深度定制
/// base_hash_rate 以H/s 传入的 基础数值
/// to_format 想要转换成的单位类型
/// f_sed 小数点后保留位数
/// return--->GTHashRate 返回 GTHashRate 结构体
pub fn hash_rate_to_format(base_hash_rate: f64, to_format: HashRateUnitFormat, f_sed: usize) -> GTHashRate {
    let k = 1_000.0;
    let m = k * k;
    let g = m * k;
    let t = g * k;
    let p = t * k;
    let e = p * k;
    let z = e * k;
    let y = z * k;

    let (cm_hs, cmp_unit, cmp_unit_str) = match to_format {
        HashRateUnitFormat::YHs => (base_hash_rate / y, HashRateUnitFormat::YHs, HashRateUnitFormat::YHs.to_string()),
        HashRateUnitFormat::ZHs => (base_hash_rate / z, HashRateUnitFormat::ZHs, HashRateUnitFormat::ZHs.to_string()),
        HashRateUnitFormat::EHs => (base_hash_rate / e, HashRateUnitFormat::EHs, HashRateUnitFormat::EHs.to_string()),
        HashRateUnitFormat::PHs => (base_hash_rate / p, HashRateUnitFormat::PHs, HashRateUnitFormat::PHs.to_string()),
        HashRateUnitFormat::THs => (base_hash_rate / t, HashRateUnitFormat::THs, HashRateUnitFormat::THs.to_string()),
        HashRateUnitFormat::GHs => (base_hash_rate / g, HashRateUnitFormat::GHs, HashRateUnitFormat::GHs.to_string()),
        HashRateUnitFormat::MHs => (base_hash_rate / m, HashRateUnitFormat::MHs, HashRateUnitFormat::MHs.to_string()),
        HashRateUnitFormat::KHs => (base_hash_rate / k, HashRateUnitFormat::KHs, HashRateUnitFormat::KHs.to_string()),
        HashRateUnitFormat::Hs | _ => (base_hash_rate, HashRateUnitFormat::Hs, HashRateUnitFormat::Hs.to_string()),
    };

    GTHashRate::new(format!("{:.*}", f_sed, cm_hs), cmp_unit_str, cmp_unit)
}

/// 算力单位自动格式化
/// hs 以H/s 传入的 基础数值
/// f_sed 小数点后保留位数
pub fn hash_rate_format_with_sed(hs: f64, f_sed: usize) -> String {
    let k = 1_000.0;
    let m = k * k;
    let g = m * k;
    let t = g * k;
    let p = t * k;
    let e = p * k;
    let z = e * k;
    let y = z * k;

    let hsr = if hs >= y {
        hash_rate_to_format(hs, HashRateUnitFormat::YHs, f_sed)
    } else if hs >= z {
        hash_rate_to_format(hs, HashRateUnitFormat::ZHs, f_sed)
    } else if hs >= e {
        hash_rate_to_format(hs, HashRateUnitFormat::EHs, f_sed)
    } else if hs >= p {
        hash_rate_to_format(hs, HashRateUnitFormat::PHs, f_sed)
    } else if hs >= t {
        hash_rate_to_format(hs, HashRateUnitFormat::THs, f_sed)
    } else if hs >= g {
        hash_rate_to_format(hs, HashRateUnitFormat::GHs, f_sed)
    } else if hs >= m {
        hash_rate_to_format(hs, HashRateUnitFormat::MHs, f_sed)
    } else if hs >= k {
        hash_rate_to_format(hs, HashRateUnitFormat::KHs, f_sed)
    } else {
        hash_rate_to_format(hs, HashRateUnitFormat::Hs, f_sed)
    };

    format!("{} {}", hsr.value, hsr.unit_str)
}

/// 算力单位自动格式化 一键式
/// hs 以H/s 传入的 基础数值, 默认小数点后暴力3位
pub fn hash_rate_format(hs: f64) -> String {
    hash_rate_format_with_sed(hs,3)
}