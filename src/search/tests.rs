#[cfg(test)]
mod tests {
    use paste::paste;
    macro_rules! test_engine {
        ($func:ident, $query:literal, map) => {
            paste! {
                #[tokio::test]
                async fn [<test_ $func>]() {
                    let map = HashMap::default();
                    $func($query, &map).await.unwrap();
                    assert!(map.len() > 0);
                }
            }
        };
        ($func:ident, $query:literal) => {
            paste! {
                #[tokio::test]
                async fn [<test_ $func>]() {
                    let results = $func($query).await.unwrap();
                    assert!(results.len() > 0);
                }
            }
        };
        ($func:ident) => {
            test_engine!($func, "shark");
        };
    }

    use crate::search::brave::brave;
    use crate::search::duckduckgo::duckduckgo;
    use crate::search::img::qwant;
    use crate::search::vids::indivious;
    use scc::HashMap;

    test_engine!(qwant);
    test_engine!(indivious);

    test_engine!(duckduckgo, "schizophreina", map);
    test_engine!(brave, "schizophrenia", map);
}
