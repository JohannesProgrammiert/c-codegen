use crate::{CStorageClass, CVarDecl};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFnDecl {
    pub(crate) name: String,
    ret: String,
    storage: Option<CStorageClass>,
    inline: bool,
    args: Vec<CVarDecl>,
}

impl CFnDecl {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ret: "void".into(),
            storage: None,
            inline: false,
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

    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }

    pub fn arg(mut self, arg: impl Into<CVarDecl>) -> Self {
        self.args.push(arg.into());
        self
    }
}

impl std::fmt::Display for CFnDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = self.storage {
            write!(f, "{} ", s)?;
        }
        if self.inline {
            write!(f, "inline ")?;
        }
        write!(f, "{} {}(", self.ret, self.name)?;
        for (idx, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;
            if idx < self.args.len() - 1 {
                write!(f, ", ")?;
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
    pub fn add_line(mut self, line: impl Into<String>) -> Self {
        self.body.push(line.into());
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

impl std::fmt::Display for CFnImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {{", self.decl)?;
        for line in &self.body {
            writeln!(f, "    {}", line)?;
        }
        write!(f, "}}")
    }
}

mod test {

    #[test]
    fn test_decl() {
        use super::*;
        let expected = "extern uint32_t my_func(void * a, const uint8_t * buf, size_t buflen)";

        let decl_inst = CFnDecl::new("my_func")
            .extern_()
            .ret("uint32_t")
            .arg(("void *", "a"))
            .arg(CVarDecl::new("uint8_t *", "buf").const_())
            .arg(("size_t", "buflen"));
        assert_eq!(expected.to_string(), decl_inst.to_string());
    }

    #[test]
    fn test_impl() {
        use super::*;
        let expected = "static inline void my_func(void * a, const uint8_t buf[], size_t buflen) {
    printf(\"Hello World\");
}";

        let decl_inst = CFnDecl::new("my_func")
            .static_()
            .inline()
            .arg(("void *", "a"))
            .arg(CVarDecl::new("uint8_t", "buf").const_().unsized_array())
            .arg(("size_t", "buflen"));
        let impl_inst = CFnImpl::from(decl_inst).add_line("printf(\"Hello World\");");
        assert_eq!(expected.to_string(), impl_inst.to_string());
    }
}
