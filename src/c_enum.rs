use crate::CTypedefKind;

pub struct CEnum {
    name: String,
    typedef: Option<CTypedefKind>,
    // TODO verify values (ordering, fitting to base value)
    members: Vec<(String, Option<i128>)>,
}

impl CEnum {
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

    pub fn member(mut self, name: impl Into<String>, def: Option<i128>) -> Self {
        self.members.push((name.into(), def));
        self
    }
}

impl std::fmt::Display for CEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(td) = &self.typedef {
            match td {
                CTypedefKind::Unnamed => writeln!(f, "typedef enum {{")?,
                CTypedefKind::Explicit(_) => writeln!(f, "typedef enum {} {{", self.name)?,
                CTypedefKind::Named => writeln!(f, "typedef enum {} {{", self.name)?,
            }
        } else {
            writeln!(f, "enum {} {{", self.name)?;
        }

        for (name, def) in &self.members {
            if let Some(d) = def {
                writeln!(f, "    {} = {},", name, d)?;
            } else {
                writeln!(f, "    {},", name)?;
            }
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
    fn test_enum() {
        use super::*;
        let expected = "typedef enum enum_foo {
    ENUM_FOO_A,
    ENUM_FOO_B = 5,
    ENUM_FOO_C = 7,
    ENUM_FOO_D,
} enum2;";

        let enum_inst = CEnum::new("enum_foo")
            .as_typedef(CTypedefKind::explicit("enum2"))
            .member("ENUM_FOO_A", None)
            .member("ENUM_FOO_B", Some(5))
            .member("ENUM_FOO_C", Some(7))
            .member("ENUM_FOO_D", None);
        assert_eq!(expected.to_string(), enum_inst.to_string());
    }
}
