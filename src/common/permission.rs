use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Role {
    Owner,
    Organizer,
    FileOrganizer,
    Writer,
    Commenter,
    #[default]
    Reader,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Owner => write!(f, "owner"),
            Role::Organizer => write!(f, "organizer"),
            Role::FileOrganizer => write!(f, "fileOrganizer"),
            Role::Writer => write!(f, "writer"),
            Role::Commenter => write!(f, "commenter"),
            Role::Reader => write!(f, "reader"),
        }
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owner" => Ok(Role::Owner),
            "organizer" => Ok(Role::Organizer),
            "fileOrganizer" => Ok(Role::FileOrganizer),
            "writer" => Ok(Role::Writer),
            "commenter" => Ok(Role::Commenter),
            "reader" => Ok(Role::Reader),
            _ => Err(format!("'{}' is not a valid role, valid roles are: owner, organizer, fileOrganizer, writer, commenter, reader", s)),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Type {
    User,
    Group,
    Domain,
    #[default]
    Anyone,
}

impl Type {
    pub fn requires_email(&self) -> bool {
        match self {
            Type::User => true,
            Type::Group => true,
            Type::Domain => false,
            Type::Anyone => false,
        }
    }

    pub fn requires_domain(&self) -> bool {
        match self {
            Type::User => false,
            Type::Group => false,
            Type::Domain => true,
            Type::Anyone => false,
        }
    }

    pub fn supports_file_discovery(&self) -> bool {
        match self {
            Type::User => false,
            Type::Group => false,
            Type::Domain => true,
            Type::Anyone => true,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::User => write!(f, "user"),
            Type::Group => write!(f, "group"),
            Type::Domain => write!(f, "domain"),
            Type::Anyone => write!(f, "anyone"),
        }
    }
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Type::User),
            "group" => Ok(Type::Group),
            "domain" => Ok(Type::Domain),
            "anyone" => Ok(Type::Anyone),
            _ => Err(format!(
                "'{}' is not a valid type, valid types are: user, group, domain, anyone",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_from_str_valid() {
        assert_eq!("owner".parse::<Role>(), Ok(Role::Owner));
        assert_eq!("organizer".parse::<Role>(), Ok(Role::Organizer));
        assert_eq!("fileOrganizer".parse::<Role>(), Ok(Role::FileOrganizer));
        assert_eq!("writer".parse::<Role>(), Ok(Role::Writer));
        assert_eq!("commenter".parse::<Role>(), Ok(Role::Commenter));
        assert_eq!("reader".parse::<Role>(), Ok(Role::Reader));
    }

    #[test]
    fn test_role_from_str_invalid() {
        assert!("admin".parse::<Role>().is_err());
        assert!("".parse::<Role>().is_err());
    }

    #[test]
    fn test_role_display() {
        assert_eq!(Role::Owner.to_string(), "owner");
        assert_eq!(Role::Organizer.to_string(), "organizer");
        assert_eq!(Role::FileOrganizer.to_string(), "fileOrganizer");
        assert_eq!(Role::Writer.to_string(), "writer");
        assert_eq!(Role::Commenter.to_string(), "commenter");
        assert_eq!(Role::Reader.to_string(), "reader");
    }

    #[test]
    fn test_type_from_str_valid() {
        assert_eq!("user".parse::<Type>(), Ok(Type::User));
        assert_eq!("group".parse::<Type>(), Ok(Type::Group));
        assert_eq!("domain".parse::<Type>(), Ok(Type::Domain));
        assert_eq!("anyone".parse::<Type>(), Ok(Type::Anyone));
    }

    #[test]
    fn test_type_from_str_invalid() {
        assert!("public".parse::<Type>().is_err());
        assert!("".parse::<Type>().is_err());
    }

    #[test]
    fn test_type_display() {
        assert_eq!(Type::User.to_string(), "user");
        assert_eq!(Type::Group.to_string(), "group");
        assert_eq!(Type::Domain.to_string(), "domain");
        assert_eq!(Type::Anyone.to_string(), "anyone");
    }

    #[test]
    fn test_type_rules() {
        assert!(Type::User.requires_email());
        assert!(Type::Group.requires_email());
        assert!(!Type::Domain.requires_email());
        assert!(!Type::Anyone.requires_email());

        assert!(!Type::User.requires_domain());
        assert!(!Type::Group.requires_domain());
        assert!(Type::Domain.requires_domain());
        assert!(!Type::Anyone.requires_domain());

        assert!(!Type::User.supports_file_discovery());
        assert!(!Type::Group.supports_file_discovery());
        assert!(Type::Domain.supports_file_discovery());
        assert!(Type::Anyone.supports_file_discovery());
    }
}
