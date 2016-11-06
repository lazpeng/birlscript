
#[derive(Debug, PartialEq, Eq)]
/// As três diferentes possibilidades de uma comparação
pub enum Comparision {
    Equal,
    More,
    Less,
    NEqual, // Usada em Strings
}

use eval::Value;

/// Tenta comparar dois numeros
fn compare_num(left: f64, right: f64) -> Comparision {
    if left < right {
        Comparision::Less
    } else if left == right {
        Comparision::Equal
    } else {
        Comparision::More
    }
}

fn compare_str(left: String, right: String) -> Comparision {
    let (len1, len2) = (left.len(), right.len());
    if len1 < len2 {
        Comparision::Less
    } else if len1 > len2 {
        Comparision::More
    } else {
        if left == right {
            Comparision::Equal
        } else {
            Comparision::NEqual
        }
    }
}

/// Tenta comparar dois valores
pub fn compare(left: Value, right: Value) -> Comparision {
    use eval::Value::*;
    match left {
        Str(v1) => {
            match right {
                Str(v2) => compare_str(v1, v2),
                Num(v2) => compare_str(v1, v2.to_string()),
                NullOrEmpty => Comparision::NEqual,
            }
        }
        Num(v1) => {
            match right {
                Num(v2) => compare_num(v1, v2),
                Str(v2) => {
                    compare_num(v1,
                                v2.parse::<f64>()
                                    .expect("Erro na conversão de String pra Número"))
                }
                NullOrEmpty => Comparision::NEqual,
            }
        }
        NullOrEmpty => {
            match right {
                NullOrEmpty => Comparision::Equal,
                _ => Comparision::NEqual,
            }
        }
    }
}
