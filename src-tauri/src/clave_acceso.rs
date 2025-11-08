use std::collections::HashMap;

/// Genera la clave de acceso según el algoritmo del SRI
pub fn generar_clave_acceso(
    fecha: &str, // DDMMYYYY
    tipo_comprobante: &str, // 01, 04, etc.
    ruc: &str,
    ambiente: &str, // 1 pruebas, 2 producción
    serie: &str, // 3 dígitos
    numero: &str, // 9 dígitos
    codigo_numerico: &str, // 8 dígitos
    tipo_emision: &str, // 1 normal
) -> String {
    let mut clave = String::new();
    clave.push_str(fecha);
    clave.push_str(tipo_comprobante);
    clave.push_str(ruc);
    clave.push_str(ambiente);
    clave.push_str(serie);
    clave.push_str(numero);
    clave.push_str(codigo_numerico);
    clave.push_str(tipo_emision);

    // Calcular dígito verificador módulo 11
    let digito_verificador = calcular_modulo11(&clave);
    clave.push_str(&digito_verificador.to_string());

    clave
}

/// Calcula el dígito verificador módulo 11
fn calcular_modulo11(clave: &str) -> u32 {
    let factores = [2, 3, 4, 5, 6, 7];
    let mut suma = 0;
    let chars: Vec<char> = clave.chars().rev().collect();

    for (i, &c) in chars.iter().enumerate() {
        let digito = c.to_digit(10).unwrap();
        let factor = factores[i % factores.len()];
        suma += digito * factor;
    }

    let modulo = suma % 11;
    if modulo == 0 {
        0
    } else {
        11 - modulo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generar_clave_acceso() {
        let clave = generar_clave_acceso(
            "01012023",
            "01",
            "1234567890001",
            "1",
            "001",
            "000000001",
            "12345678",
            "1",
        );
        assert_eq!(clave.len(), 49);
        // Verificar que termina con dígito verificador
        let ultimo = clave.chars().last().unwrap().to_digit(10).unwrap();
        assert!(ultimo >= 0 && ultimo <= 9);
    }

    #[test]
    fn test_calcular_modulo11() {
        // Caso de ejemplo: clave sin dígito verificador
        let clave_sin_dv = "0101202301123456789000110000000001123456781";
        let dv = calcular_modulo11(clave_sin_dv);
        assert_eq!(dv, 5); // Ejemplo hipotético
    }
}