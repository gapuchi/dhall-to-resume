let Name
    : Type
    = { first  : Text, last : Text }

let User
    : Type
    = { name : Name, url : Text, email : Text, phone : Text }

let LeftRightHeader
    : Type
    = { left : Text, right : Text }

let ResumeSubSection
    : Type
    = { header : Optional LeftRightHeader, items : List Text }

let ResumeSection
    : Type
    = { title : Text
      , header : Optional LeftRightHeader
      , subSections : List ResumeSubSection
      }

let Resume
    : Type
    = { user : User, sections : List ResumeSection }

in Resume