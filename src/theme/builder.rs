use crate::style::{Color, Style};

use super::{Rule, Theme, Type, WorkspaceRule};

pub(crate) struct ThemeBuilder {
    name: String,
    ty: Type,
    workspace_rules: Vec<WorkspaceRule>,
    semantic_rules: Vec<Rule>,
    textmate_rules: Vec<Rule>,
}

impl ThemeBuilder {
    pub(crate) fn new(name: String, ty: Type) -> Self {
        Self {
            name,
            ty,
            workspace_rules: Vec::new(),
            semantic_rules: Vec::new(),
            textmate_rules: Vec::new(),
        }
    }

    pub(crate) fn build(self) -> Theme {
        Theme {
            name: self.name,
            ty: self.ty,
            workspace_rules: self.workspace_rules,
            semantic_rules: self.semantic_rules,
            textmate_rules: self.textmate_rules,
        }
    }

    pub(crate) fn add_rule(&mut self, scope: Scope, style: impl Into<Style>) {
        self.add_rules(&[scope], style);
    }

    pub(crate) fn add_rules(&mut self, scopes: &[Scope], style: impl Into<Style>) {
        let style = style.into();

        for scope in scopes {
            let (scope_name, rules) = match scope {
                Scope::Semantic(s) => (s, &mut self.semantic_rules),
                Scope::Textmate(s) => (s, &mut self.textmate_rules),
            };

            rules.push(Rule {
                scope_name: scope_name.to_string(),
                style,
            });
        }
    }

    pub(crate) fn add_workspace_rule(&mut self, scope: &str, color: impl Into<Color>) {
        self.workspace_rules.push(WorkspaceRule {
            scope_name: scope.to_string(),
            color: color.into(),
        });
    }
}

pub(crate) enum Scope {
    Semantic(&'static str),
    Textmate(&'static str),
}
