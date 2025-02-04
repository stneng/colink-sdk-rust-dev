use async_recursion::async_recursion;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

impl crate::application::CoLink {
    #[async_recursion]
    pub(crate) async fn _update_entry_append(
        &self,
        key_name: &str,
        payload: &[u8],
    ) -> Result<String, Error> {
        if key_name.contains('$') {
            let (string_before, macro_type, string_after) = self._parse_macro(key_name);
            match macro_type.as_str() {
                "redis" => {
                    return self
                        ._append_entry_redis(&string_before, &string_after, payload)
                        .await;
                }
                "chunk" => {
                    return self._append_entry_chunk(&string_before, payload).await;
                }
                _ => {}
            }
        }
        let lock = self.lock(key_name).await?;
        // use a closure to prevent locking forever caused by errors
        let res = async {
            let mut data = self.read_entry(key_name).await?;
            data.append(&mut payload.to_vec());
            let res = self.update_entry(key_name, &data).await?;
            Ok::<String, Error>(res)
        }
        .await;
        self.unlock(lock).await?;
        res
    }
}
