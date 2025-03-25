use crate::{CStorageClass, CTypeDecl};

pub struct CFnDecl {
    name: String,
    ret: String,
    storage: Option<CStorageClass>,
    args: Vec<CTypeDecl>,
}

impl CFnDecl {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ret: "void".into(),
            storage: None,
            args: Vec::new(),
        }
    }

    pub fn ret(mut self, rettype: impl Into<String>) -> Self {
        self.ret = rettype.into();
        self
    }

    pub fn static_(mut self) -> Self {
        self.storage = Some(CStorageClass::Static);
        self
    }

    pub fn extern_(mut self) -> Self {
        self.storage = Some(CStorageClass::Extern);
        self
    }

    pub fn arg(mut self, arg: CTypeDecl) -> Self {
        self.args.push(arg);
        self
    }
}

impl std::fmt::Display for CFnDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = self.storage {
            write!(f, "{} ", s)?;
        }
        write!(f, "{} {} (", self.ret, self.name)?;
        for (idx, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;
            if idx < self.args.len() - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, ")")
    }
}

pub struct CFnImpl {
    decl: CFnDecl,
    body: Vec<String>,
}

impl CFnImpl {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            decl: CFnDecl::new(name),
            body: Vec::new(),
        }
    }

    pub fn add_line(mut self, line: String) -> Self {
        self.body.push(line);
        self
    }
}

impl From<CFnDecl> for CFnImpl {
    fn from(value: CFnDecl) -> Self {
        Self {
            decl: value,
            body: Vec::new(),
        }
    }
}
