use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

type FilterCallback = Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any> + Send + Sync>;

struct Filter {
    id: u64,
    priority: i32,
    callback: FilterCallback,
    type_id: TypeId,
}

static FILTERS: Lazy<RwLock<HashMap<String, Vec<Filter>>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

static FILTER_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Registers a filter callback for the given hook name.
/// Returns an ID that can be used to remove the filter.
pub fn add_filter<T: 'static + Send + Sync>(
    hook: &str,
    priority: i32,
    callback: impl Fn(T) -> T + 'static + Send + Sync,
) -> u64 {
    let id = FILTER_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let filter = Filter {
        id,
        priority,
        callback: Box::new(move |value: Box<dyn Any>| {
            let value = *value.downcast::<T>().expect("Type mismatch in filter");
            let new_value = callback(value);
            Box::new(new_value)
        }),
        type_id: TypeId::of::<T>(),
    };

    let mut filters = FILTERS.write().unwrap();
    let entry = filters.entry(hook.to_string()).or_insert_with(Vec::new);
    entry.push(filter);
    entry.sort_by_key(|f| f.priority);

    id
}

/// Applies all filter callbacks registered for the given hook to `value`.
pub fn apply_filters<T: 'static + Send + Sync>(hook: &str, value: T) -> T {
    let filters = FILTERS.read().unwrap();
    let filter_list = match filters.get(hook) {
        Some(list) => list,
        None => return value,
    };

    let mut result: Box<dyn Any> = Box::new(value);
    for filter in filter_list {
        if filter.type_id == TypeId::of::<T>() {
            result = (filter.callback)(result);
        } else {
            panic!("Type mismatch for filter hook '{}'", hook);
        }
    }

    *result.downcast::<T>().expect("Type mismatch in final value")
}

/// Removes the filter with the specified ID from the given hook.
/// Returns `true` if a filter was removed.
pub fn remove_filter(hook: &str, id: u64) -> bool {
    let mut filters = FILTERS.write().unwrap();
    if let Some(list) = filters.get_mut(hook) {
        let orig_len = list.len();
        list.retain(|f| f.id != id);

        return list.len() != orig_len;
    }

    false
}

/// Removes all filters for the given hook.
pub fn remove_all_filters(hook: &str) {
    let mut filters = FILTERS.write().unwrap();
    filters.remove(hook);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filters_i32() {
        let hook = "modify_int";
        add_filter(hook, 10, |v: i32| v + 1);
        add_filter(hook, 20, |v: i32| v * 3);
        let result = apply_filters(hook, 4);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_filters_string() {
        let hook = "modify_string";
        add_filter(hook, 10, |s: String| format!("Hello, {}", s));
        add_filter(hook, 20, |s: String| s.to_uppercase());
        let result = apply_filters(hook, "world".to_string());
        assert_eq!(result, "HELLO, WORLD");
    }
}
