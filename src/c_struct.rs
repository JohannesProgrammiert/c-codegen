use crate::{CArraySize, CTypedefKind, CTypeDecl};

pub struct CStruct {
    name: String,
    typedef: Option<CTypedefKind>,
    members: Vec<CTypeDecl>,
}

impl CStruct {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            typedef: None,
            members: Vec::new(),
        }
    }

    pub fn as_typedef(mut self, typedef: CTypedefKind) -> Self {
        self.typedef = Some(typedef);
        self
    }

    pub fn member(mut self, type_: impl Into<String>, name: impl Into<String>) -> Self {
        self.members.push(CTypeDecl::new(type_, name));
        self
    }

    pub fn add_member(&mut self, member: CTypeDecl) {
        self.members.push(member);
    }
}

impl std::fmt::Display for CStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(td) = &self.typedef {
            match td {
                CTypedefKind::Unnamed => writeln!(f, "typedef struct {{")?,
                CTypedefKind::Explicit(_) => writeln!(f, "typedef struct {} {{", self.name)?,
                CTypedefKind::Named => writeln!(f, "typedef struct {} {{", self.name)?,
            }
        } else {
            writeln!(f, "struct {} {{", self.name)?;
        }

        for m in &self.members {
            writeln!(f, "    {}", m)?;
        }

        if let Some(td) = &self.typedef {
            if let CTypedefKind::Explicit(e) = td {
                write!(f, "}} {};", e)
            } else {
                write!(f, "}} {};", self.name)
            }
        } else {
            write!(f, "}};")
        }
    }
}

mod test {

    #[test]
    fn test_struct() {
        use super::*;
        let expected = "typedef struct struct_foo {
    uint8_t a;
    uint16_t b[35];
    const char * name;
    const int c[];
} foo;";

        let mut struct_inst = CStruct::new("struct_foo")
            .as_typedef(CTypedefKind::explicit("foo"))
            .member("uint8_t", "a");
        let b = CTypeDecl::new("uint16_t", "b").sized_array(35);
        struct_inst.add_member(b);
        let name = CTypeDecl::new("char *", "name").const_();
        struct_inst.add_member(name);
        let c = CTypeDecl::new("int", "c").const_().unsized_array();
        struct_inst.add_member(c);
        assert_eq!(expected.to_string(), struct_inst.to_string());
    }
}
