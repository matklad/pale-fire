use crate::palette::*;
use crate::style::{Color, Rgb};
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

        write_scope(f, "activityBar.activeBorder", ZENBURN_FG)?;
        write_scope(f, "activityBar.background", ZENBURN_BG_MINUS_05)?;
        write_scope(f, "activityBar.foreground", ZENBURN_FG)?;
        write_scope(f, "activityBar.inactiveForeground", ZENBURN_BG_PLUS_3)?;
        write_scope(f, "activityBarBadge.background", ZENBURN_BLUE_MINUS_3)?;
        write_scope(f, "activityBarBadge.foreground", ZENBURN_FG)?;
        write_scope(f, "badge.background", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "badge.foreground", ZENBURN_FG)?;
        write_scope(f, "button.background", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "button.foreground", ZENBURN_BG)?;
        write_scope(f, "button.hoverBackground", ZENBURN_FG)?;
        write_scope(f, "debugIcon.breakpointForeground", ZENBURN_RED)?;
        write_scope(
            f,
            "diffEditor.insertedTextBackground",
            (ZENBURN_GREEN_PLUS_4, 0x33),
        )?;
        write_scope(
            f,
            "diffEditor.removedTextBackground",
            (ZENBURN_RED_PLUS_2, 0x33),
        )?;
        write_scope(f, "editor.background", ZENBURN_BG)?;
        write_scope(f, "editor.findMatchBackground", (ZENBURN_BLUE_PLUS_1, 0x66))?;
        write_scope(
            f,
            "editor.findMatchHighlightBackground",
            (ZENBURN_BLUE_PLUS_1, 0x44),
        )?;
        write_scope(f, "editor.foldBackground", (ZENBURN_BLUE_PLUS_1, 0x22))?;
        write_scope(f, "editor.foreground", ZENBURN_FG)?;
        write_scope(f, "editor.hoverHighlightBackground", ZENBURN_BG_PLUS_1)?;
        write_scope(f, "editor.lineHighlightBackground", ZENBURN_BG_MINUS_05)?;
        write_scope(f, "editor.selectionBackground", ZENBURN_BG_MINUS_1)?;
        write_scope(f, "editor.selectionHighlightBackground", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "editor.symbolHighlightBackground", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "editor.wordHighlightBackground", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "editor.wordHighlightStrongBackground", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "editorCursor.foreground", ZENBURN_FG_PLUS_1)?;
        write_scope(f, "editorError.foreground", ZENBURN_RED_PLUS_1)?;
        write_scope(f, "editorGroup.dropBackground", (ZENBURN_BLUE_PLUS_1, 0x22))?;
        write_scope(f, "editorGroupHeader.noTabsBackground", ZENBURN_BG_PLUS_05)?;
        write_scope(f, "editorGroupHeader.tabsBackground", ZENBURN_BG_MINUS_1)?;
        write_scope(f, "editorGutter.addedBackground", ZENBURN_GREEN)?;
        write_scope(f, "editorGutter.deletedBackground", ZENBURN_RED)?;
        write_scope(f, "editorGutter.modifiedBackground", ZENBURN_YELLOW_MINUS_2)?;
        write_scope(f, "editorIndentGuide.activeBackground", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "editorIndentGuide.background", ZENBURN_BG_PLUS_1)?;
        write_scope(f, "editorLightBulb.foreground", ZENBURN_YELLOW)?;
        write_scope(f, "editorLineNumber.foreground", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "editorLink.activeForeground", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "editorOverviewRuler.addedForeground", ZENBURN_GREEN)?;
        write_scope(f, "editorOverviewRuler.border", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "editorOverviewRuler.deletedForeground", ZENBURN_RED)?;
        write_scope(
            f,
            "editorOverviewRuler.errorForeground",
            ZENBURN_RED_MINUS_2,
        )?;
        write_scope(
            f,
            "editorOverviewRuler.findMatchForeground",
            (ZENBURN_BLUE_PLUS_1, 0x88),
        )?;
        write_scope(
            f,
            "editorOverviewRuler.modifiedForeground",
            ZENBURN_YELLOW_MINUS_2,
        )?;
        write_scope(f, "editorWarning.foreground", ZENBURN_ORANGE)?;
        write_scope(f, "editorWidget.background", ZENBURN_BG_MINUS_05)?;
        write_scope(f, "editorWidget.border", ZENBURN_BG_PLUS_1)?;
        write_scope(f, "focusBorder", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "foreground", ZENBURN_FG)?;
        write_scope(
            f,
            "gitDecoration.ignoredResourceForeground",
            ZENBURN_BG_PLUS_3,
        )?;
        write_scope(
            f,
            "gitDecoration.modifiedResourceForeground",
            ZENBURN_YELLOW,
        )?;
        write_scope(
            f,
            "gitDecoration.untrackedResourceForeground",
            ZENBURN_GREEN_PLUS_3,
        )?;
        write_scope(f, "input.background", (Rgb(0xFFFFFF), 0x0A))?; // input field lightens what is behind it
        write_scope(f, "input.foreground", ZENBURN_FG)?;
        write_scope(f, "input.placeholderForeground", ZENBURN_BG_PLUS_3)?;
        write_scope(f, "list.activeSelectionBackground", ZENBURN_BG_PLUS_1)?;
        write_scope(f, "list.activeSelectionForeground", ZENBURN_FG)?;
        write_scope(f, "list.errorForeground", ZENBURN_RED_PLUS_1)?;
        write_scope(f, "list.focusBackground", ZENBURN_BG_PLUS_1)?;
        write_scope(f, "list.highlightForeground", ZENBURN_BLUE_PLUS_3)?;
        write_scope(f, "list.hoverBackground", ZENBURN_BG)?;
        write_scope(f, "list.inactiveSelectionBackground", ZENBURN_BG_PLUS_05)?;
        write_scope(f, "list.warningForeground", ZENBURN_ORANGE)?;
        write_scope(f, "minimap.errorHighlight", ZENBURN_RED_PLUS_2)?;
        write_scope(f, "minimap.findMatchHighlight", (ZENBURN_BLUE_PLUS_1, 0x66))?;
        write_scope(f, "minimap.selectionHighlight", (ZENBURN_BG_MINUS_1, 0x88))?;
        write_scope(f, "panel.background", ZENBURN_BG_PLUS_05)?;
        write_scope(f, "panel.border", ZENBURN_BG_PLUS_2)?;
        write_scope(f, "panelTitle.activeForeground", ZENBURN_FG)?;
        write_scope(f, "peekView.border", ZENBURN_BG_PLUS_3)?;
        write_scope(f, "peekViewEditor.background", ZENBURN_BG)?;
        write_scope(
            f,
            "peekViewEditor.matchHighlightBackground",
            (ZENBURN_BLUE_PLUS_1, 0x66),
        )?;
        write_scope(f, "peekViewResult.background", ZENBURN_BG_MINUS_05)?;
        write_scope(f, "peekViewResult.fileForeground", ZENBURN_FG)?;
        write_scope(f, "peekViewResult.lineForeground", (ZENBURN_FG, 0x99))?;
        write_scope(
            f,
            "peekViewResult.matchHighlightBackground",
            (ZENBURN_BLUE_PLUS_1, 0x44),
        )?;
        write_scope(f, "peekViewResult.selectionBackground", ZENBURN_BG_PLUS_1)?;
        write_scope(f, "peekViewResult.selectionForeground", ZENBURN_FG)?;
        write_scope(f, "peekViewTitle.background", ZENBURN_BG_MINUS_05)?;
        write_scope(
            f,
            "peekViewTitleDescription.foreground",
            ZENBURN_BLUE_PLUS_1,
        )?;
        write_scope(f, "peekViewTitleLabel.foreground", ZENBURN_FG_PLUS_1)?;
        write_scope(f, "progressBar.background", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "rust_analyzer.inlayHints.foreground", ZENBURN_GREEN)?;
        write_scope(f, "scrollbar.shadow", (Rgb(0x000000), 0x88))?;
        write_scope(f, "selection.background", (Rgb(0xFFFFFF), 0x55))?;
        write_scope(f, "settings.headerForeground", ZENBURN_FG_PLUS_1)?;
        write_scope(f, "settings.modifiedItemIndicator", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "sideBar.background", ZENBURN_BG_MINUS_05)?;
        write_scope(f, "sideBar.foreground", ZENBURN_FG)?;
        write_scope(f, "sideBarTitle.foreground", ZENBURN_FG_PLUS_1)?;
        write_scope(f, "statusBar.background", ZENBURN_BG_MINUS_1)?;
        write_scope(f, "statusBar.debuggingBackground", ZENBURN_BG_MINUS_1)?;
        write_scope(f, "statusBar.debuggingBorder", ZENBURN_MAGENTA)?;
        write_scope(f, "statusBar.foreground", ZENBURN_GREEN_PLUS_1)?;
        write_scope(f, "statusBar.noFolderBackground", ZENBURN_BG_MINUS_1)?;
        write_scope(f, "tab.activeForeground", ZENBURN_FG)?;
        write_scope(f, "tab.border", ZENBURN_BG)?;
        write_scope(f, "tab.inactiveBackground", ZENBURN_BG_MINUS_1)?;
        write_scope(f, "tab.inactiveForeground", ZENBURN_BG_PLUS_3)?;
        write_scope(f, "terminal.ansiBlack", ZENBURN_BG_MINUS_1)?;
        write_scope(f, "terminal.ansiBlue", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "terminal.ansiBrightBlack", ZENBURN_BG_PLUS_3)?;
        write_scope(f, "terminal.ansiBrightBlue", ZENBURN_BLUE_PLUS_3)?;
        write_scope(f, "terminal.ansiBrightCyan", ZENBURN_CYAN)?;
        write_scope(f, "terminal.ansiBrightGreen", ZENBURN_GREEN_PLUS_4)?;
        write_scope(f, "terminal.ansiBrightMagenta", ZENBURN_ORANGE)?;
        write_scope(f, "terminal.ansiBrightRed", ZENBURN_RED_PLUS_2)?;
        write_scope(f, "terminal.ansiBrightWhite", ZENBURN_FG_PLUS_1)?;
        write_scope(f, "terminal.ansiBrightYellow", ZENBURN_YELLOW)?;
        write_scope(f, "terminal.ansiCyan", ZENBURN_BLUE_MINUS_1)?;
        write_scope(f, "terminal.ansiGreen", ZENBURN_GREEN)?;
        write_scope(f, "terminal.ansiMagenta", ZENBURN_ORANGE)?;
        write_scope(f, "terminal.ansiRed", ZENBURN_RED)?;
        write_scope(f, "terminal.ansiWhite", ZENBURN_FG)?;
        write_scope(f, "terminal.ansiYellow", ZENBURN_YELLOW_MINUS_2)?;
        write_scope(f, "terminal.foreground", ZENBURN_FG)?;
        write_scope(f, "terminal.selectionBackground", ZENBURN_BG_MINUS_05)?; // Lighter than normal selection background to compensate for lighter terminal background
        write_scope(f, "terminalCursor.foreground", ZENBURN_FG_PLUS_1)?;
        write_scope(f, "textLink.activeForeground", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "textLink.foreground", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "textPreformat.foreground", ZENBURN_FG)?; // inline code in e.g. Settings page
        write_scope(f, "titleBar.activeBackground", ZENBURN_BG_MINUS_05)?;
        write_scope(f, "titleBar.activeForeground", ZENBURN_FG)?;
        write_scope(f, "titleBar.inactiveBackground", ZENBURN_BG_MINUS_05)?;
        write_scope(f, "titleBar.inactiveForeground", ZENBURN_BG_PLUS_3)?;
        write_scope(f, "widget.shadow", (Rgb(0x000000), 0x88))?;

        writeln!(f, "\t}},")?;

        Ok(())
    }

    fn semantic_colors(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t\"semanticHighlighting\": true,")?;
        writeln!(f, "\t\"semanticTokenColors\": {{")?;

        write_scope(f, "boolean", ZENBURN_BLUE_PLUS_3)?;
        write_scope(f, "comment", ZENBURN_GREEN)?;
        write_scope(f, "comment.documentation", ZENBURN_GREEN_PLUS_2)?;
        write_scope(f, "keyword", ZENBURN_YELLOW)?;
        write_scope(f, "*.unsafe", ZENBURN_RED_MINUS_1)?;
        write_scope(f, "function.unsafe", ZENBURN_RED_MINUS_1)?;
        write_scope(f, "operator.unsafe", ZENBURN_RED_MINUS_1)?;
        write_scope(f, "property", ZENBURN_GREEN_PLUS_3)?;
        write_scope(f, "function", ZENBURN_CYAN)?;
        write_scope(f, "namespace", ZENBURN_GREEN_PLUS_4)?;
        write_scope(f, "macro", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "formatSpecifier", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "variable", ZENBURN_FG)?;
        write_scope(f, "variable.static.constant", ZENBURN_BLUE_PLUS_3)?;
        write_scope(f, "struct", ZENBURN_BLUE_MINUS_1)?;
        write_scope(f, "enum", ZENBURN_BLUE_MINUS_1)?;
        write_scope(f, "union", ZENBURN_BLUE_MINUS_1)?;
        write_scope(f, "typeAlias", ZENBURN_BLUE_MINUS_1)?;
        write_scope(f, "builtinType", ZENBURN_BLUE)?;
        write_scope(f, "type", ZENBURN_BLUE_MINUS_1)?;
        write_scope(f, "interface", ZENBURN_BLUE)?;
        write_scope(f, "enumMember", ZENBURN_BLUE_PLUS_3)?;
        write_scope(f, "typeParameter", ZENBURN_ORANGE)?;
        write_scope(f, "lifetime", ZENBURN_ORANGE)?;
        write_scope(f, "number", ZENBURN_GREEN_PLUS_4)?;
        write_scope(f, "string", ZENBURN_RED)?;
        write_scope(f, "attribute", ZENBURN_BLUE_PLUS_1)?;
        write_scope(f, "function.attribute", ZENBURN_BLUE_PLUS_1)?;

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
            ZENBURN_GREEN_PLUS_3,
        )?;

        write_textmate(
            f,
            Some("Function"),
            &["entity.name.function", "support.function"],
            ZENBURN_CYAN,
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
            ZENBURN_GREEN_PLUS_4,
        )?;

        write_textmate(
            f,
            None,
            &["entity.name.macro", "entity.name.other.preprocessor.macro"],
            ZENBURN_BLUE_PLUS_1,
        )?;

        write_textmate(f, Some("Variable"), &["variable"], ZENBURN_FG)?;

        write_textmate(
            f,
            Some("Constant"),
            &[
                "constant",
                "entity.name.constant",
                "variable.other.enummember",
                "support.constant",
            ],
            ZENBURN_BLUE_PLUS_3,
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
            ZENBURN_BLUE_MINUS_1,
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
            ZENBURN_BLUE,
        )?;

        write_textmate(f, None, &["entity.name.type.parameter"], ZENBURN_ORANGE)?;

        write_textmate(
            f,
            None,
            &[
                "storage.modifier.lifetime.rust",
                "entity.name.lifetime.rust",
                "entity.name.type.lifetime",
            ],
            ZENBURN_ORANGE,
        )?;

        write_textmate(
            f,
            None,
            &["constant.numeric", "keyword.other.unit"],
            ZENBURN_GREEN_PLUS_4,
        )?;

        write_textmate(
            f,
            Some("Comment"),
            &["comment", "punctuation.definition.comment"],
            ZENBURN_GREEN,
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
            ZENBURN_RED,
        )?;

        write_textmate(
            f,
            Some("Annotations"),
            &[
                "meta.attribute",
                "support.variable.attribute",
                "punctuation.definition.attributeentry",
            ],
            ZENBURN_BLUE_PLUS_1,
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
            ZENBURN_YELLOW,
        )?;

        write_textmate(
            f,
            Some("CSS Class/ID"),
            &[
                "entity.other.attribute-name.class",
                "entity.other.attribute-name.id",
            ],
            ZENBURN_BLUE_MINUS_1,
        )?;

        write_textmate(f, None, &["keyword.other.unsafe"], ZENBURN_RED_MINUS_1)?;

        write_textmate(
            f,
            Some("Punctuation"),
            &["keyword.operator", "punctuation"],
            ZENBURN_FG,
        )?;

        write_textmate(
            f,
            None,
            &["keyword.operator.logical.python"],
            ZENBURN_YELLOW,
        )?;

        write_textmate(
            f,
            None,
            &["meta.function-call.generic.python", "source.python support"],
            ZENBURN_CYAN,
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
