
#[derive(Debug, PartialEq, Eq)]
/// As três diferentes possibilidades de uma comparação
pub enum Comparision {
    Equal,
    More,
    Less,
    NEqual, // Usada em Strings
}

use value::Value;

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
    use value::Value::*;
    match left {
        Str(v1) => {
            match right {
                Str(v2) => compare_str(*v1, *v2),
                Number(v2) => compare_str(*v1, v2.to_string()),
            }
        }
        Number(v1) => {
            match right {
                Number(v2) => compare_num(v1, v2),
                Str(v2) => {
                    compare_num(v1,
                                v2.parse::<f64>()
                                    .expect("Erro na conversão de String pra Número"))
                }
            }
        }
    }
}
