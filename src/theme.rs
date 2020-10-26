use crate::color::{Color, Rgb};
use std::fmt;

pub(crate) struct Theme;

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        writeln!(f, "\t\"name\": \"Pale Fire\",")?;
        writeln!(f, "\t\"type\": \"dark\",")?;

        self.workspace_colors(f)?;
        self.semantic_colors(f)?;
        self.textmate_colors(f)?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

impl Theme {
    fn workspace_colors(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t\"colors\": {{")?;

        write_scope(f, "activityBar.activeBorder", Rgb(0xDCDCCC))?;
        write_scope(f, "activityBar.background", Rgb(0x383838))?;
        write_scope(f, "activityBar.foreground", Rgb(0xDCDCCC))?;
        write_scope(f, "activityBar.inactiveForeground", Rgb(0x6F6F6F))?;
        write_scope(f, "activityBarBadge.background", Rgb(0x5C888B))?;
        write_scope(f, "activityBarBadge.foreground", Rgb(0xDCDCCC))?;
        write_scope(f, "badge.background", Rgb(0x5F5F5F))?;
        write_scope(f, "badge.foreground", Rgb(0xDCDCCC))?;
        write_scope(f, "button.background", Rgb(0x94BFF3))?;
        write_scope(f, "button.foreground", Rgb(0x3F3F3F))?;
        write_scope(f, "button.hoverBackground", Rgb(0xDCDCCC))?;
        write_scope(f, "debugIcon.breakpointForeground", Rgb(0xCC9393))?;
        write_scope(
            f,
            "diffEditor.insertedTextBackground",
            (Rgb(0xBFEBBF), 0x33),
        )?;
        write_scope(f, "diffEditor.removedTextBackground", (Rgb(0xECB3B3), 0x33))?;
        write_scope(f, "editor.background", Rgb(0x3F3F3F))?;
        write_scope(f, "editor.findMatchBackground", (Rgb(0x94BFF3), 0x66))?;
        write_scope(
            f,
            "editor.findMatchHighlightBackground",
            (Rgb(0x94BFF3), 0x44),
        )?;
        write_scope(f, "editor.foldBackground", (Rgb(0x94BFF3), 0x22))?;
        write_scope(f, "editor.foreground", Rgb(0xDCDCCC))?;
        write_scope(f, "editor.hoverHighlightBackground", Rgb(0x4F4F4F))?;
        write_scope(f, "editor.lineHighlightBackground", Rgb(0x383838))?;
        write_scope(f, "editor.selectionBackground", Rgb(0x2B2B2B))?;
        write_scope(f, "editor.selectionHighlightBackground", Rgb(0x5F5F5F))?;
        write_scope(f, "editor.symbolHighlightBackground", Rgb(0x5F5F5F))?;
        write_scope(f, "editor.wordHighlightBackground", Rgb(0x5F5F5F))?;
        write_scope(f, "editor.wordHighlightStrongBackground", Rgb(0x5F5F5F))?;
        write_scope(f, "editorCursor.foreground", Rgb(0xFFFFEF))?;
        write_scope(f, "editorError.foreground", Rgb(0xDCA3A3))?;
        write_scope(f, "editorGroup.dropBackground", (Rgb(0x94BFF3), 0x22))?;
        write_scope(f, "editorGroupHeader.noTabsBackground", Rgb(0x494949))?;
        write_scope(f, "editorGroupHeader.tabsBackground", Rgb(0x2B2B2B))?;
        write_scope(f, "editorGutter.addedBackground", Rgb(0x7F9F7F))?;
        write_scope(f, "editorGutter.deletedBackground", Rgb(0xCC9393))?;
        write_scope(f, "editorGutter.modifiedBackground", Rgb(0xD0BF8F))?;
        write_scope(f, "editorIndentGuide.activeBackground", Rgb(0x5F5F5F))?;
        write_scope(f, "editorIndentGuide.background", Rgb(0x4F4F4F))?;
        write_scope(f, "editorLightBulb.foreground", Rgb(0xF0DFAF))?;
        write_scope(f, "editorLineNumber.foreground", Rgb(0x5F5F5F))?;
        write_scope(f, "editorLink.activeForeground", Rgb(0x94BFF3))?;
        write_scope(f, "editorOverviewRuler.addedForeground", Rgb(0x7F9F7F))?;
        write_scope(f, "editorOverviewRuler.border", Rgb(0x5F5F5F))?;
        write_scope(f, "editorOverviewRuler.deletedForeground", Rgb(0xCC9393))?;
        write_scope(f, "editorOverviewRuler.errorForeground", Rgb(0xAC7373))?;
        write_scope(
            f,
            "editorOverviewRuler.findMatchForeground",
            (Rgb(0x94BFF3), 0x88),
        )?;
        write_scope(f, "editorOverviewRuler.modifiedForeground", Rgb(0xD0BF8F))?;
        write_scope(f, "editorWarning.foreground", Rgb(0xDFAF8F))?;
        write_scope(f, "editorWidget.background", Rgb(0x383838))?;
        write_scope(f, "editorWidget.border", Rgb(0x4F4F4F))?;
        write_scope(f, "focusBorder", Rgb(0x5F5F5F))?;
        write_scope(f, "foreground", Rgb(0xDCDCCC))?;
        write_scope(f, "gitDecoration.ignoredResourceForeground", Rgb(0x6F6F6F))?;
        write_scope(f, "gitDecoration.modifiedResourceForeground", Rgb(0xF0DFAF))?;
        write_scope(
            f,
            "gitDecoration.untrackedResourceForeground",
            Rgb(0xAFD8AF),
        )?;
        write_scope(f, "input.background", (Rgb(0xFFFFFF), 0x0A))?; // input field lightens what is behind it
        write_scope(f, "input.foreground", Rgb(0xDCDCCC))?;
        write_scope(f, "input.placeholderForeground", Rgb(0x6F6F6F))?;
        write_scope(f, "list.activeSelectionBackground", Rgb(0x4F4F4F))?;
        write_scope(f, "list.activeSelectionForeground", Rgb(0xDCDCCC))?;
        write_scope(f, "list.errorForeground", Rgb(0xDCA3A3))?;
        write_scope(f, "list.focusBackground", Rgb(0x4F4F4F))?;
        write_scope(f, "list.highlightForeground", Rgb(0xBDE0F3))?;
        write_scope(f, "list.hoverBackground", Rgb(0x3F3F3F))?;
        write_scope(f, "list.inactiveSelectionBackground", Rgb(0x494949))?;
        write_scope(f, "list.warningForeground", Rgb(0xDFAF8F))?;
        write_scope(f, "minimap.errorHighlight", Rgb(0xECB3B3))?;
        write_scope(f, "minimap.findMatchHighlight", (Rgb(0x94BFF3), 0x66))?;
        write_scope(f, "minimap.selectionHighlight", (Rgb(0x2B2B2B), 0x88))?;
        write_scope(f, "panel.background", Rgb(0x494949))?;
        write_scope(f, "panel.border", Rgb(0x5F5F5F))?;
        write_scope(f, "panelTitle.activeForeground", Rgb(0xDCDCCC))?;
        write_scope(f, "peekView.border", Rgb(0x6F6F6F))?;
        write_scope(f, "peekViewEditor.background", Rgb(0x3F3F3F))?;
        write_scope(
            f,
            "peekViewEditor.matchHighlightBackground",
            (Rgb(0x94BFF3), 0x66),
        )?;
        write_scope(f, "peekViewResult.background", Rgb(0x383838))?;
        write_scope(f, "peekViewResult.fileForeground", Rgb(0xDCDCCC))?;
        write_scope(f, "peekViewResult.lineForeground", (Rgb(0xDCDCCC), 0x99))?;
        write_scope(
            f,
            "peekViewResult.matchHighlightBackground",
            (Rgb(0x94BFF3), 0x44),
        )?;
        write_scope(f, "peekViewResult.selectionBackground", Rgb(0x4F4F4F))?;
        write_scope(f, "peekViewResult.selectionForeground", Rgb(0xDCDCCC))?;
        write_scope(f, "peekViewTitle.background", Rgb(0x383838))?;
        write_scope(f, "peekViewTitleDescription.foreground", Rgb(0x94BFF3))?;
        write_scope(f, "peekViewTitleLabel.foreground", Rgb(0xFFFFEF))?;
        write_scope(f, "progressBar.background", Rgb(0x94BFF3))?;
        write_scope(f, "rust_analyzer.inlayHints.foreground", Rgb(0x7F9F7F))?;
        write_scope(f, "scrollbar.shadow", (Rgb(0x000000), 0x88))?;
        write_scope(f, "selection.background", (Rgb(0xFFFFFF), 0x55))?;
        write_scope(f, "settings.headerForeground", Rgb(0xFFFFEF))?;
        write_scope(f, "settings.modifiedItemIndicator", Rgb(0x94BFF3))?;
        write_scope(f, "sideBar.background", Rgb(0x383838))?;
        write_scope(f, "sideBar.foreground", Rgb(0xDCDCCC))?;
        write_scope(f, "sideBarTitle.foreground", Rgb(0xFFFFEF))?;
        write_scope(f, "statusBar.background", Rgb(0x2B2B2B))?;
        write_scope(f, "statusBar.debuggingBackground", Rgb(0x2B2B2B))?;
        write_scope(f, "statusBar.debuggingBorder", Rgb(0xDC8CC3))?;
        write_scope(f, "statusBar.foreground", Rgb(0x8FB28F))?;
        write_scope(f, "statusBar.noFolderBackground", Rgb(0x2B2B2B))?;
        write_scope(f, "tab.activeForeground", Rgb(0xDCDCCC))?;
        write_scope(f, "tab.border", Rgb(0x3F3F3F))?;
        write_scope(f, "tab.inactiveBackground", Rgb(0x2B2B2B))?;
        write_scope(f, "tab.inactiveForeground", Rgb(0x6F6F6F))?;
        write_scope(f, "terminal.ansiBlack", Rgb(0x2B2B2B))?;
        write_scope(f, "terminal.ansiBlue", Rgb(0x94BFF3))?;
        write_scope(f, "terminal.ansiBrightBlack", Rgb(0x6F6F6F))?;
        write_scope(f, "terminal.ansiBrightBlue", Rgb(0xBDE0F3))?;
        write_scope(f, "terminal.ansiBrightCyan", Rgb(0x93E0E3))?;
        write_scope(f, "terminal.ansiBrightGreen", Rgb(0xBFEBBF))?;
        write_scope(f, "terminal.ansiBrightMagenta", Rgb(0xDFAF8F))?;
        write_scope(f, "terminal.ansiBrightRed", Rgb(0xECB3B3))?;
        write_scope(f, "terminal.ansiBrightWhite", Rgb(0xFFFFEF))?;
        write_scope(f, "terminal.ansiBrightYellow", Rgb(0xF0DFAF))?;
        write_scope(f, "terminal.ansiCyan", Rgb(0x7CB8BB))?;
        write_scope(f, "terminal.ansiGreen", Rgb(0x7F9F7F))?;
        write_scope(f, "terminal.ansiMagenta", Rgb(0xDFAF8F))?;
        write_scope(f, "terminal.ansiRed", Rgb(0xCC9393))?;
        write_scope(f, "terminal.ansiWhite", Rgb(0xDCDCCC))?;
        write_scope(f, "terminal.ansiYellow", Rgb(0xD0BF8F))?;
        write_scope(f, "terminal.foreground", Rgb(0xDCDCCC))?;
        write_scope(f, "terminal.selectionBackground", Rgb(0x383838))?; // Lighter than normal selection background to compensate for lighter terminal background
        write_scope(f, "terminalCursor.foreground", Rgb(0xFFFFEF))?;
        write_scope(f, "textLink.activeForeground", Rgb(0x94BFF3))?;
        write_scope(f, "textLink.foreground", Rgb(0x94BFF3))?;
        write_scope(f, "textPreformat.foreground", Rgb(0xDCDCCC))?; // inline code in e.g. Settings page
        write_scope(f, "titleBar.activeBackground", Rgb(0x383838))?;
        write_scope(f, "titleBar.activeForeground", Rgb(0xDCDCCC))?;
        write_scope(f, "titleBar.inactiveBackground", Rgb(0x383838))?;
        write_scope(f, "titleBar.inactiveForeground", Rgb(0x6F6F6F))?;
        write_scope(f, "widget.shadow", (Rgb(0x000000), 0x88))?;

        writeln!(f, "\t}},")?;

        Ok(())
    }

    fn semantic_colors(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t\"semanticHighlighting\": true,")?;
        writeln!(f, "\t\"semanticTokenColors\": {{")?;

        write_scope(f, "boolean", Rgb(0xBDE0F3))?;
        write_scope(f, "comment", Rgb(0x7F9F7F))?;
        write_scope(f, "comment.documentation", Rgb(0x9FC59F))?;
        write_scope(f, "keyword", Rgb(0xF0DFAF))?;
        write_scope(f, "*.unsafe", Rgb(0xBC8383))?;
        write_scope(f, "function.unsafe", Rgb(0xBC8383))?;
        write_scope(f, "operator.unsafe", Rgb(0xBC8383))?;
        write_scope(f, "property", Rgb(0xAFD8AF))?;
        write_scope(f, "function", Rgb(0x93E0E3))?;
        write_scope(f, "namespace", Rgb(0xBFEBBF))?;
        write_scope(f, "macro", Rgb(0x94BFF3))?;
        write_scope(f, "formatSpecifier", Rgb(0x94BFF3))?;
        write_scope(f, "variable", Rgb(0xDCDCCC))?;
        write_scope(f, "variable.static.constant", Rgb(0xBDE0F3))?;
        write_scope(f, "struct", Rgb(0x7CB8BB))?;
        write_scope(f, "enum", Rgb(0x7CB8BB))?;
        write_scope(f, "union", Rgb(0x7CB8BB))?;
        write_scope(f, "typeAlias", Rgb(0x7CB8BB))?;
        write_scope(f, "builtinType", Rgb(0x8CD0D3))?;
        write_scope(f, "type", Rgb(0x7CB8BB))?;
        write_scope(f, "interface", Rgb(0x8CD0D3))?;
        write_scope(f, "enumMember", Rgb(0xBDE0F3))?;
        write_scope(f, "typeParameter", Rgb(0xDFAF8F))?;
        write_scope(f, "lifetime", Rgb(0xDFAF8F))?;
        write_scope(f, "number", Rgb(0xBFEBBF))?;
        write_scope(f, "string", Rgb(0xCC9393))?;
        write_scope(f, "attribute", Rgb(0x94BFF3))?;
        write_scope(f, "function.attribute", Rgb(0x94BFF3))?;

        writeln!(f, "\t}},")?;

        Ok(())
    }

    fn textmate_colors(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t\"tokenColors\": [")?;

        write_textmate(
            f,
            Some("Property"),
            &[
                "entity.name.field",
                "entity.name.variable.field",
                "punctuation.support.type.property-name",
                "support.type.property-name",
                "support.type.vendored.property-name",
                "variable.other.member",
                "variable.other.object.property",
                "variable.other.property",
            ],
            Rgb(0xAFD8AF),
        )?;

        write_textmate(
            f,
            Some("Function"),
            &["entity.name.function", "support.function"],
            Rgb(0x93E0E3),
        )?;

        write_textmate(
            f,
            Some("Namespace"),
            &[
                "entity.name.module",
                "entity.name.namespace",
                "entity.name.type.namespace",
                "storage.modifier.import",
                "storage.modifier.package",
            ],
            Rgb(0xBFEBBF),
        )?;

        write_textmate(
            f,
            None,
            &["entity.name.macro", "entity.name.other.preprocessor.macro"],
            Rgb(0x94BFF3),
        )?;

        write_textmate(f, Some("Variable"), &["variable"], Rgb(0xDCDCCC))?;

        write_textmate(
            f,
            Some("Constant"),
            &[
                "constant",
                "entity.name.constant",
                "variable.other.enummember",
                "support.constant",
            ],
            Rgb(0xBDE0F3),
        )?;

        write_textmate(
            f,
            Some("Type"),
            &[
                "entity.name.type",
                "storage.type",
                "support.class",
                "support.type",
            ],
            Rgb(0x7CB8BB),
        )?;

        write_textmate(
            f,
            Some("Primitive"),
            &[
                "keyword.type",
                "storage.type.boolean.go",
                "storage.type.built-in",
                "storage.type.byte.go",
                "storage.type.error.go",
                "storage.type.numeric.go",
                "storage.type.primitive",
                "storage.type.rune.go",
                "storage.type.string.go",
                "storage.type.uintptr.go",
                "support.type",
            ],
            Rgb(0x8CD0D3),
        )?;

        write_textmate(f, None, &["entity.name.type.parameter"], Rgb(0xDFAF8F))?;

        write_textmate(
            f,
            None,
            &[
                "storage.modifier.lifetime.rust",
                "entity.name.lifetime.rust",
                "entity.name.type.lifetime",
            ],
            Rgb(0xDFAF8F),
        )?;

        write_textmate(
            f,
            None,
            &["constant.numeric", "keyword.other.unit"],
            Rgb(0xBFEBBF),
        )?;

        write_textmate(
            f,
            Some("Comment"),
            &["comment", "punctuation.definition.comment"],
            Rgb(0x7F9F7F),
        )?;

        write_textmate(
            f,
            Some("String and Character"),
            &[
                "constant.character",
                "punctuation.definition.char",
                "punctuation.definition.string",
                "string",
            ],
            Rgb(0xCC9393),
        )?;

        write_textmate(
            f,
            Some("Annotations"),
            &[
                "meta.attribute",
                "support.variable.attribute",
                "punctuation.definition.attributeentry",
            ],
            Rgb(0x94BFF3),
        )?;

        write_textmate(
            f,
            Some("Keyword"),
            &[
                "constant.language.null",
                "entity.name.tag",
                "keyword.operator.expression",
                "keyword.operator.new",
                "keyword.type.go",
                "keyword",
                "markup.heading.marker",
                "punctuation.definition.heading",
                "storage.modifier",
                "storage.type.class",
                "storage.type.enum",
                "storage.type.function.python",
                "storage.type.function.ts",
                "storage.type.function",
                "storage.type.js",
                "storage.type.rust",
                "storage.type.struct",
                "storage.type.ts",
                "variable.language.this",
            ],
            Rgb(0xF0DFAF),
        )?;

        write_textmate(
            f,
            Some("CSS Class/ID"),
            &[
                "entity.other.attribute-name.class",
                "entity.other.attribute-name.id",
            ],
            Rgb(0x7CB8BB),
        )?;

        write_textmate(f, None, &["keyword.other.unsafe"], Rgb(0xBC8383))?;

        write_textmate(
            f,
            Some("Punctuation"),
            &["keyword.operator", "punctuation"],
            Rgb(0xDCDCCC),
        )?;

        write_textmate(f, None, &["keyword.operator.logical.python"], Rgb(0xF0DFAF))?;

        write_textmate(
            f,
            None,
            &["meta.function-call.generic.python", "source.python support"],
            Rgb(0x93E0E3),
        )?;

        writeln!(f, "\t]")?;

        Ok(())
    }
}

fn write_textmate(
    f: &mut fmt::Formatter,
    name: Option<&str>,
    scopes: &[&str],
    color: impl Into<Color>,
) -> fmt::Result {
    writeln!(f, "\t\t{{")?;

    if let Some(name) = name {
        writeln!(f, "\t\t\t\"name\": \"{}\",", name)?;
    }

    let num_scopes = scopes.len();

    write!(f, "\t\t\t\"scope\":")?;

    if num_scopes == 1 {
        writeln!(f, " \"{}\",", scopes[0])?;
    } else {
        writeln!(f, " [")?;

        for (idx, scope) in scopes.iter().enumerate() {
            write!(f, "\t\t\t\t\"{}\"", scope)?;

            if idx == num_scopes - 1 {
                writeln!(f)?;
            } else {
                writeln!(f, ",")?;
            }
        }

        writeln!(f, "\t\t\t],")?;
    }

    writeln!(f, "\t\t\t\"settings\": {{")?;
    writeln!(f, "\t\t\t\t\"foreground\": {}", color.into())?;
    writeln!(f, "\t\t\t}}")?;

    writeln!(f, "\t\t}},")?;

    Ok(())
}

fn write_scope(
    f: &mut fmt::Formatter<'_>,
    scope_name: &str,
    color: impl Into<Color>,
) -> fmt::Result {
    writeln!(f, "\t\t\"{}\": {},", scope_name, color.into())
}
