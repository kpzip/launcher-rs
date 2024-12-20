use std::borrow::Borrow;
use std::path::{Path, PathBuf};

pub fn unpack_option<T, S, F>(double: Option<S>, func: F) -> Option<T>
where
    F: Fn(S) -> Option<T>
{
    func(double?)
}

pub fn ref_comparison<T>(lhs: &T, rhs: &T) -> bool {
    lhs as *const T == rhs as *const T
}

pub fn flip_result_option<T, E>(res: Result<Option<T>, E>) -> Option<Result<T, E>> {
    match res {
        Ok(t) => match t {
            None => None,
            Some(t) => Some(Ok(t)),
        }
        Err(e) => Some(Err(e)),
    }
}

pub fn option_comparison<Rhs, Lhs>(lhs: Option<Lhs>, rhs: Option<Rhs>) -> bool
where
    Lhs: PartialEq<Rhs>
{
    match lhs {
        None => rhs.is_none(),
        Some(lhs) => {
            match rhs {
                None => false,
                Some(rhs) => lhs == rhs
            }
        }
    }
}

// Copied from stack overflow https://stackoverflow.com/questions/50322817/how-do-i-remove-the-prefix-from-a-canonical-windows-path
// Allows us to obliterate the garbage that is added to the beginning of windows paths and it unsupported by the jvm
pub trait StripCanonicalization
where
    Self: AsRef<Path>,
{
    #[cfg(not(target_os = "windows"))]
    fn strip_canonicalization(&self) -> PathBuf {
        self.as_ref().to_path_buf()
    }

    #[cfg(target_os = "windows")]
    fn strip_canonicalization(&self) -> PathBuf {
        const VERBATIM_PREFIX: &str = r#"\\?\"#;
        let p = self.as_ref().display().to_string();
        if p.starts_with(VERBATIM_PREFIX) {
            PathBuf::from(&p[VERBATIM_PREFIX.len()..])
        } else {
            self.as_ref().to_path_buf()
        }
    }
}

impl<T> StripCanonicalization for T where T: AsRef<Path> {}