pub mod c_struct;
pub use c_struct::*;
pub mod c_enum;
pub use c_enum::*;
pub mod c_fn;
pub use c_fn::*;

#[derive(Default)]
pub struct CScope {
    include_guards: Option<CIncludeGuards>,
    snippets: Vec<CSnippet>,
}

impl CScope {
    pub fn with_include_guards(mut self, guards: CIncludeGuards) -> Self {
        self.include_guards = Some(guards);
        self
    }

    /// Include library, like `#include <stdio.h>`
    pub fn include_lib(mut self, libname: impl Into<String>) -> Self {
        self.snippets.push(CSnippet::LibInclude(libname.into()));
        self
    }

    /// Include file, like `#include "myheader.h"`
    pub fn include_file(mut self, filename: impl Into<String>) -> Self {
        self.snippets.push(CSnippet::FileInclude(filename.into()));
        self
    }

    // /// Separates lib includes from file includes and optionally sorts them.
    // ///
    // /// # Example
    // ///
    // /// ```c
    // /// #include <stddef.h>
    // /// #include <stdio.h>
    // /// #include <string.h>
    // ///
    // /// #include "header_a.h"
    // /// #include "header_b.h"
    // /// ```
    // pub fn separate_includes(mut self, sort: bool) -> Self {
    //     unimplemented!();
    // }

    /// Todo: replace by `add_struct`, `add_enum` etc.
    pub fn add_snippet(&mut self, snippet: CSnippet) {
        self.snippets.push(snippet);
    }

    pub fn add_struct(&mut self, inst: CStruct) {
        self.snippets.push(CSnippet::Struct(inst));
    }

    pub fn add_enum(&mut self, inst: CEnum) {
        self.snippets.push(CSnippet::Enum(inst));
    }

    pub fn add_raw(&mut self, inst: String) {
        self.snippets.push(CSnippet::Raw(inst));
    }

    pub fn add_libinclude(&mut self, libname: String) {
        self.snippets.push(CSnippet::LibInclude(libname.into()));
    }

    pub fn add_fileinclude(&mut self, filename: String) {
        self.snippets.push(CSnippet::FileInclude(filename.into()));
    }
}

impl std::fmt::Display for CScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(guards) = &self.include_guards {
            match guards {
                CIncludeGuards::PragmaOnce => writeln!(f, "#pragma once")?,
                CIncludeGuards::String(s) => {
                    writeln!(f, "#ifndef {}", s)?;
                    writeln!(f, "#define {}", s)?;
                }
            }
        }

        if let Some(guards) = &self.include_guards {
            if let CIncludeGuards::String(s) = guards {
                writeln!(f, "#endif /* {} */", s)?;
            }
        }
        Ok(())
    }
}

pub enum CIncludeGuards {
    PragmaOnce,
    String(String),
}

pub enum CSnippet {
    LibInclude(String),
    FileInclude(String),
    Raw(String),
    Struct(CStruct),
    Enum(CEnum),
    FnDecl(CFnDecl),
    FnImpl(CFnImpl),
    GlobalVar(CVar),
}

pub struct CVar {
    name: String,
    type_: String,
    storage: Option<CStorageClass>,
    init: Option<String>,
    array: Option<CArraySize>,
    const_: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CTypedefKind {
    /// Same as struct/enum name
    ///
    /// ```c
    /// typedef struct foo {
    ///     int a;
    ///     int b;
    /// } foo;
    /// ```
    Named,
    /// Explicit name. This field is used as the typedef name.
    ///
    /// ```c
    /// typedef struct foo {
    ///     int a;
    ///     int b;
    /// } set_by_this_field;
    /// ```
    Explicit(String),
    /// Struct/enum has no typename.
    ///
    /// ```c
    /// typedef struct {
    ///     int a;
    ///     int b;
    /// } foo;
    /// ```
    Unnamed,
}

impl CTypedefKind {
    pub fn explicit(typename: impl Into<String>) -> Self {
        Self::Explicit(typename.into())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CStorageClass {
    Extern,
    Static,
}

impl std::fmt::Display for CStorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Extern => write!(f, "extern"),
            Self::Static => write!(f, "static"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CArraySize {
    Unsized,
    Sized(usize),
}

pub struct CTypeDecl {
    name: String,
    type_: String,
    array: Option<CArraySize>,
    const_: bool,
}

impl CTypeDecl {
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

impl std::fmt::Display for CTypeDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.const_ {
            write!(f, "const ")?;
        }

        write!(f, "{} ", self.type_)?;

        if let Some(a) = &self.array {
            match a {
                CArraySize::Sized(s) => write!(f, "{}[{}];", self.name, s)?,
                CArraySize::Unsized => write!(f, "{}[];", self.name)?,
            }
        } else {
            write!(f, "{};", self.name)?;
        }

        Ok(())
    }
}
