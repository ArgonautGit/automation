use inputbot::KeybdKey;

pub fn bind_key<F>(key: KeybdKey, function: F)
where
    F: Fn() + Sync + Send + 'static,
{
    key.bind(function);
}
