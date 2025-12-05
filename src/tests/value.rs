#[cfg(test)]
mod tests {
    use crate::mvcc::Value;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    // PartialEq 테스트
    #[test]
    fn test_value_equality() {
        assert_eq!(Value::Integer(42), Value::Integer(42));
        assert_eq!(Value::Float(3.14), Value::Float(3.14));
        assert_eq!(
            Value::String("hello".to_string()),
            Value::String("hello".to_string())
        );
        assert_eq!(Value::Boolean(true), Value::Boolean(true));
        assert_eq!(Value::Null, Value::Null);
    }

    // Hash 테스트 - 동일한 값
    #[test]
    fn test_hash_integer_same() {
        let h1 = calculate_hash(&Value::Integer(42));
        let h2 = calculate_hash(&Value::Integer(42));
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_integer_different() {
        let h1 = calculate_hash(&Value::Integer(42));
        let h2 = calculate_hash(&Value::Integer(100));
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_float_same() {
        let h1 = calculate_hash(&Value::Float(3.14));
        let h2 = calculate_hash(&Value::Float(3.14));
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_float_nan() {
        let h1 = calculate_hash(&Value::Float(f64::NAN));
        let h2 = calculate_hash(&Value::Float(f64::NAN));
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_float_zero() {
        let h1 = calculate_hash(&Value::Float(0.0));
        let h2 = calculate_hash(&Value::Float(-0.0));
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_string_same() {
        let h1 = calculate_hash(&Value::String("hello".to_string()));
        let h2 = calculate_hash(&Value::String("hello".to_string()));
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_string_different() {
        let h1 = calculate_hash(&Value::String("hello".to_string()));
        let h2 = calculate_hash(&Value::String("world".to_string()));
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_boolean() {
        let h_true1 = calculate_hash(&Value::Boolean(true));
        let h_true2 = calculate_hash(&Value::Boolean(true));
        let h_false = calculate_hash(&Value::Boolean(false));

        assert_eq!(h_true1, h_true2);
        assert_ne!(h_true1, h_false);
    }

    #[test]
    fn test_hash_null() {
        let h1 = calculate_hash(&Value::Null);
        let h2 = calculate_hash(&Value::Null);
        assert_eq!(h1, h2);
    }

    // 서로 다른 타입은 다른 해시
    #[test]
    fn test_hash_different_types() {
        let int_hash = calculate_hash(&Value::Integer(1));
        let float_hash = calculate_hash(&Value::Float(1.0));
        let str_hash = calculate_hash(&Value::String("1".to_string()));
        let bool_hash = calculate_hash(&Value::Boolean(true));
        let null_hash = calculate_hash(&Value::Null);

        assert_ne!(int_hash, float_hash);
        assert_ne!(int_hash, str_hash);
        assert_ne!(int_hash, bool_hash);
        assert_ne!(int_hash, null_hash);
        assert_ne!(float_hash, str_hash);
    }

    #[test]
    fn test_hash_zero_vs_null() {
        let int_zero = calculate_hash(&Value::Integer(0));
        let null_hash = calculate_hash(&Value::Null);
        assert_ne!(int_zero, null_hash);
    }
}
