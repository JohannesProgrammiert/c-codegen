use crate::{CArraySize, CStorageClass};

/// Variable declaration.
pub struct CVar {
    decl: CVarDecl,
    storage: Option<CStorageClass>,
    init: Option<String>,
}

impl From<CVarDecl> for CVar {
    fn from(value: CVarDecl) -> Self {
        Self {
            decl: value,
            storage: None,
            init: None,
        }
    }
}

impl CVar {
    pub fn extern_(mut self) -> Self {
        self.storage = Some(CStorageClass::Extern);
        self
    }
    pub fn static_(mut self) -> Self {
        self.storage = Some(CStorageClass::Static);
        self
    }
    pub fn init(mut self, init: impl Into<String>) -> Self {
        self.init = Some(init.into());
        self
    }
}

impl std::fmt::Display for CVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = &self.storage {
            write!(f, "{} ", s)?;
        }
        write!(f, "{}", self.decl)?;
        if let Some(i) = &self.init {
            write!(f, " = {}", i)?;
        }
        Ok(())
    }
}

/// Variable declaration such as `const uint32_t mytype`, used in struct members and function args.
///
/// Use `CVar` to specify storage and class and/or init value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CVarDecl {
    name: String,
    type_: String,
    array: Option<CArraySize>,
    const_: bool,
}

impl CVarDecl {
    pub fn new(type_: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_: type_.into(),
            array: None,
            const_: false,
        }
    }

    pub fn const_(mut self) -> Self {
        self.const_ = true;
        self
    }

    pub fn sized_array(mut self, arraysize: usize) -> Self {
        self.array = Some(CArraySize::Sized(arraysize));
        self
    }

    pub fn unsized_array(mut self) -> Self {
        self.array = Some(CArraySize::Unsized);
        self
    }
}

impl From<(&str, &str)> for CVarDecl {
    fn from(value: (&str, &str)) -> Self {
        Self {
            name: value.1.into(),
            type_: value.0.into(),
            array: None,
            const_: false,
        }
    }
}

impl std::fmt::Display for CVarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.const_ {
            write!(f, "const ")?;
        }

        write!(f, "{} {}", self.type_, self.name)?;

        if let Some(a) = &self.array {
            write!(f, "{}", a)?;
        }

        Ok(())
    }
}

mod test {

    #[test]
    fn test_var() {
        use super::*;
        let expected = "static const uint8_t foo[2] = {0, 0}";

        let var_decl = CVarDecl::new("uint8_t", "foo").const_().sized_array(2);
        let var_def = CVar::from(var_decl).static_().init("{0, 0}");
        assert_eq!(expected.to_string(), var_def.to_string());
    }
}
