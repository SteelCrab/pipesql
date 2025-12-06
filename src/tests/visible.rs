#[cfg(test)]
mod tests {
    use crate::mvcc::{Row, RowVersion, Value, visible};
    use std::collections::HashSet;

    fn create_test_row() -> Row {
        Row {
            values: vec![Value::Integer(1), Value::String("test".to_string())],
        }
    }

    fn create_version(id: u64, t_created: u64, t_deleted: Option<u64>) -> RowVersion {
        RowVersion {
            id,
            row: create_test_row(),
            t_created,
            t_deleted,
        }
    }

    #[test]
    fn test_visible_created_by_current_tx() {
        let version = create_version(1, 10, None);
        let active_txs = HashSet::new();

        assert!(visible(&version, 10, &active_txs));
    }

    #[test]
    fn test_not_visible_created_and_deleted_by_current_tx() {
        let version = create_version(1, 10, Some(10));
        let active_txs = HashSet::new();

        assert!(!visible(&version, 10, &active_txs));
    }

    #[test]
    fn test_not_visible_created_by_uncommitted_tx() {
        let version = create_version(1, 5, None);
        let mut active_txs = HashSet::new();
        active_txs.insert(5);

        assert!(!visible(&version, 10, &active_txs));
    }

    #[test]
    fn test_not_visible_created_by_future_tx() {
        let version = create_version(1, 15, None);
        let active_txs = HashSet::new();

        assert!(!visible(&version, 10, &active_txs));
    }

    #[test]
    fn test_visible_created_by_committed_past_tx() {
        let version = create_version(1, 5, None);
        let active_txs = HashSet::new();

        assert!(visible(&version, 10, &active_txs));
    }

    // 현재 트랜잭션이 삭제한 row는 안 보여야 함
    #[test]
    fn test_not_visible_deleted_by_current_tx() {
        let version = create_version(1, 5, Some(10));
        let active_txs = HashSet::new();

        assert!(!visible(&version, 10, &active_txs));
    }

    // 커밋된 과거 트랜잭션이 삭제한 row는 안 보여야 함
    #[test]
    fn test_not_visible_deleted_by_committed_past_tx() {
        let version = create_version(1, 3, Some(8));
        let active_txs = HashSet::new();

        assert!(!visible(&version, 10, &active_txs));
    }

    // 아직 커밋되지 않은 트랜잭션이 삭제한 row는 보여야 함
    #[test]
    fn test_visible_deleted_by_uncommitted_tx() {
        let version = create_version(1, 3, Some(8));
        let mut active_txs = HashSet::new();
        active_txs.insert(8); // tx 8이 아직 활성 상태 (삭제가 커밋되지 않음)

        assert!(visible(&version, 10, &active_txs));
    }

    #[test]
    fn test_visible_deleted_by_future_tx() {
        let version = create_version(1, 3, Some(15));
        let active_txs = HashSet::new();

        assert!(visible(&version, 10, &active_txs));
    }

    #[test]
    fn test_visible_with_multiple_active_txs() {
        let version = create_version(1, 3, None);
        let mut active_txs = HashSet::new();
        active_txs.insert(5);
        active_txs.insert(7);
        active_txs.insert(12);

        assert!(visible(&version, 10, &active_txs));
    }

    #[test]
    fn test_visible_created_by_current_deleted_by_other() {
        let version = create_version(1, 10, Some(15));
        let active_txs = HashSet::new();

        assert!(visible(&version, 10, &active_txs));
    }

    #[test]
    fn test_visible_created_by_current_deleted_by_other2() {
        let version = create_version(1, 10, Some(10));
        let mut active_txs = HashSet::new();
        active_txs.insert(10);

        assert!(!visible(&version, 10, &active_txs));
    }
}
