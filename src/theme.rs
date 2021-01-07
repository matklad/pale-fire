use crate::palette::*;
use crate::style::{Color, FontStyle, Style};
use tincture::{ColorSpace, Oklch};

pub(crate) struct Theme;

impl From<Theme> for json::Value {
    fn from(theme: Theme) -> Self {
        let mut map = json::Map::new();

        map.insert("name".to_string(), Self::String("Pale Fire".to_string()));
        map.insert("type".to_string(), Self::String("dark".to_string()));

        map.insert("colors".to_string(), theme.workspace_colors());
        map.insert("semanticHighlighting".to_string(), Self::Bool(true));
        map.insert("semanticTokenColors".to_string(), theme.semantic_colors());
        map.insert("tokenColors".to_string(), theme.textmate_colors());

        Self::Object(map)
    }
}

impl Theme {
    fn workspace_colors(&self) -> json::Value {
        let mut colors_owned = json::Map::new();
        let colors = &mut colors_owned;

        color_rule(colors, "activityBar.activeBorder", fg(7));
        color_rule(colors, "activityBar.background", bg(1));
        color_rule(colors, "activityBar.foreground", fg(7));
        color_rule(colors, "activityBar.inactiveForeground", bg(6));
        color_rule(colors, "activityBarBadge.background", blue(2));
        color_rule(colors, "activityBarBadge.foreground", bg(2));
        color_rule(colors, "badge.background", bg(5));
        color_rule(colors, "badge.foreground", fg(7));
        color_rule(colors, "button.background", blue(2));
        color_rule(colors, "button.foreground", bg(2));
        color_rule(colors, "button.hoverBackground", fg(7));
        color_rule(colors, "debugIcon.breakpointForeground", red(2));
        color_rule(
            colors,
            "diffEditor.insertedTextBackground",
            (green(ColorLightnessPreset::DiffBg), 0x33),
        );
        color_rule(
            colors,
            "diffEditor.removedTextBackground",
            (red(ColorLightnessPreset::DiffBg), 0x33),
        );
        color_rule(colors, "editor.background", bg(2));
        color_rule(colors, "editor.findMatchBackground", (blue(2), 0x66));
        color_rule(
            colors,
            "editor.findMatchHighlightBackground",
            (blue(2), 0x44),
        );
        color_rule(colors, "editor.foldBackground", (blue(2), 0x22));
        color_rule(colors, "editor.foreground", fg(7));
        color_rule(colors, "editor.hoverHighlightBackground", bg(4));
        color_rule(colors, "editor.lineHighlightBackground", bg(1));
        color_rule(colors, "editor.selectionBackground", bg(0));
        color_rule(colors, "editor.selectionHighlightBackground", bg(5));
        color_rule(colors, "editor.symbolHighlightBackground", bg(5));
        color_rule(colors, "editor.wordHighlightBackground", bg(5));
        color_rule(colors, "editor.wordHighlightStrongBackground", bg(5));
        color_rule(colors, "editorCursor.foreground", fg(8));
        color_rule(colors, "editorError.foreground", red(2));
        color_rule(colors, "editorGroup.dropBackground", (blue(2), 0x22));
        color_rule(colors, "editorGroupHeader.noTabsBackground", bg(3));
        color_rule(colors, "editorGroupHeader.tabsBackground", bg(0));
        color_rule(
            colors,
            "editorGutter.addedBackground",
            green(ColorLightnessPreset::Gutter),
        );
        color_rule(
            colors,
            "editorGutter.deletedBackground",
            red(ColorLightnessPreset::Gutter),
        );
        color_rule(
            colors,
            "editorGutter.modifiedBackground",
            yellow(ColorLightnessPreset::Gutter),
        );
        color_rule(colors, "editorGroup.border", bg(5));
        color_rule(colors, "editorIndentGuide.activeBackground", bg(5));
        color_rule(colors, "editorIndentGuide.background", bg(4));
        color_rule(colors, "editorLightBulb.foreground", yellow(4));
        color_rule(colors, "editorLineNumber.foreground", bg(5));
        color_rule(colors, "editorLink.activeForeground", blue(2));
        color_rule(
            colors,
            "editorOverviewRuler.addedForeground",
            green(ColorLightnessPreset::OverviewRuler),
        );
        color_rule(colors, "editorOverviewRuler.border", bg(5));
        color_rule(
            colors,
            "editorOverviewRuler.deletedForeground",
            red(ColorLightnessPreset::OverviewRuler),
        );
        color_rule(
            colors,
            "editorOverviewRuler.errorForeground",
            red(ColorLightnessPreset::OverviewRuler),
        );
        color_rule(
            colors,
            "editorOverviewRuler.findMatchForeground",
            (blue(ColorLightnessPreset::OverviewRuler), 0x88),
        );
        color_rule(
            colors,
            "editorOverviewRuler.modifiedForeground",
            yellow(ColorLightnessPreset::OverviewRuler),
        );
        color_rule(colors, "editorWarning.foreground", orange(2));
        color_rule(colors, "editorWidget.background", bg(1));
        color_rule(colors, "editorWidget.border", bg(4));
        color_rule(colors, "focusBorder", bg(5));
        color_rule(colors, "foreground", fg(7));
        color_rule(colors, "gitDecoration.ignoredResourceForeground", bg(6));
        color_rule(
            colors,
            "gitDecoration.modifiedResourceForeground",
            yellow(ColorLightnessPreset::GitDecoration),
        );
        color_rule(
            colors,
            "gitDecoration.untrackedResourceForeground",
            green(ColorLightnessPreset::GitDecoration),
        );
        color_rule(colors, "input.background", (Oklch::WHITE, 0x0A)); // input field lightens what is behind it
        color_rule(colors, "input.foreground", fg(7));
        color_rule(colors, "input.placeholderForeground", bg(6));
        color_rule(colors, "list.activeSelectionBackground", bg(4));
        color_rule(colors, "list.activeSelectionForeground", fg(7));
        color_rule(colors, "list.errorForeground", red(2));
        color_rule(colors, "list.focusBackground", bg(4));
        color_rule(colors, "list.highlightForeground", blue(4));
        color_rule(colors, "list.hoverBackground", bg(2));
        color_rule(colors, "list.inactiveSelectionBackground", bg(3));
        color_rule(colors, "list.warningForeground", orange(2));
        color_rule(
            colors,
            "minimap.errorHighlight",
            red(ColorLightnessPreset::Minimap),
        );
        color_rule(
            colors,
            "minimap.findMatchHighlight",
            (blue(ColorLightnessPreset::Minimap), 0x66),
        );
        color_rule(colors, "minimap.selectionHighlight", (bg(0), 0x88));
        color_rule(colors, "panel.background", bg(3));
        color_rule(colors, "panel.border", bg(5));
        color_rule(colors, "panelTitle.activeForeground", fg(7));
        color_rule(colors, "peekView.border", bg(6));
        color_rule(colors, "peekViewEditor.background", bg(2));
        color_rule(
            colors,
            "peekViewEditor.matchHighlightBackground",
            (blue(2), 0x66),
        );
        color_rule(colors, "peekViewResult.background", bg(1));
        color_rule(colors, "peekViewResult.fileForeground", fg(7));
        color_rule(colors, "peekViewResult.lineForeground", (fg(7), 0x99));
        color_rule(
            colors,
            "peekViewResult.matchHighlightBackground",
            (blue(2), 0x44),
        );
        color_rule(colors, "peekViewResult.selectionBackground", bg(4));
        color_rule(colors, "peekViewResult.selectionForeground", fg(7));
        color_rule(colors, "peekViewTitle.background", bg(1));
        color_rule(colors, "peekViewTitleDescription.foreground", blue(2));
        color_rule(colors, "peekViewTitleLabel.foreground", fg(8));
        color_rule(colors, "progressBar.background", blue(2));
        color_rule(colors, "rust_analyzer.inlayHints.foreground", green(0));
        color_rule(colors, "scrollbar.shadow", (Oklch::BLACK, 0x88));
        color_rule(colors, "selection.background", (Oklch::WHITE, 0x55));
        color_rule(colors, "settings.headerForeground", fg(8));
        color_rule(colors, "settings.modifiedItemIndicator", blue(2));
        color_rule(colors, "sideBar.background", bg(1));
        color_rule(colors, "sideBar.foreground", fg(7));
        color_rule(colors, "sideBarTitle.foreground", fg(8));
        color_rule(colors, "statusBar.background", bg(0));
        color_rule(colors, "statusBar.debuggingBackground", bg(0));
        color_rule(colors, "statusBar.foreground", green(2));
        color_rule(colors, "statusBar.debuggingForeground", orange(2));
        color_rule(colors, "statusBar.noFolderBackground", bg(0));
        color_rule(colors, "tab.activeForeground", fg(7));
        color_rule(colors, "tab.border", bg(2));
        color_rule(colors, "tab.inactiveBackground", bg(0));
        color_rule(colors, "tab.inactiveForeground", bg(6));
        color_rule(colors, "terminal.ansiBlack", bg(0));
        color_rule(
            colors,
            "terminal.ansiBlue",
            blue(ColorLightnessPreset::TerminalAnsi),
        );
        color_rule(colors, "terminal.ansiBrightBlack", bg(6));
        color_rule(
            colors,
            "terminal.ansiBrightBlue",
            blue(ColorLightnessPreset::TerminalAnsiBright),
        );
        color_rule(
            colors,
            "terminal.ansiBrightCyan",
            cyan(ColorLightnessPreset::TerminalAnsiBright),
        );
        color_rule(
            colors,
            "terminal.ansiBrightGreen",
            green(ColorLightnessPreset::TerminalAnsiBright),
        );
        color_rule(
            colors,
            "terminal.ansiBrightMagenta",
            orange(ColorLightnessPreset::TerminalAnsiBright),
        );
        color_rule(
            colors,
            "terminal.ansiBrightRed",
            red(ColorLightnessPreset::TerminalAnsiBright),
        );
        color_rule(colors, "terminal.ansiBrightWhite", fg(8));
        color_rule(
            colors,
            "terminal.ansiBrightYellow",
            yellow(ColorLightnessPreset::TerminalAnsiBright),
        );
        color_rule(
            colors,
            "terminal.ansiCyan",
            cyan(ColorLightnessPreset::TerminalAnsi),
        );
        color_rule(
            colors,
            "terminal.ansiGreen",
            green(ColorLightnessPreset::TerminalAnsi),
        );
        color_rule(
            colors,
            "terminal.ansiMagenta",
            orange(ColorLightnessPreset::TerminalAnsi),
        );
        color_rule(
            colors,
            "terminal.ansiRed",
            red(ColorLightnessPreset::TerminalAnsi),
        );
        color_rule(colors, "terminal.ansiWhite", fg(7));
        color_rule(
            colors,
            "terminal.ansiYellow",
            yellow(ColorLightnessPreset::TerminalAnsi),
        );
        color_rule(colors, "terminal.foreground", fg(7));
        color_rule(colors, "terminal.selectionBackground", bg(1)); // Lighter than normal selection background to compensate for lighter terminal background
        color_rule(colors, "terminalCursor.foreground", fg(8));
        color_rule(colors, "textLink.activeForeground", blue(2));
        color_rule(colors, "textLink.foreground", blue(2));
        color_rule(colors, "textPreformat.foreground", fg(7)); // inline code in e.g. Settings page
        color_rule(colors, "titleBar.activeBackground", bg(1));
        color_rule(colors, "titleBar.activeForeground", fg(7));
        color_rule(colors, "titleBar.inactiveBackground", bg(1));
        color_rule(colors, "titleBar.inactiveForeground", bg(6));
        color_rule(colors, "widget.shadow", (Oklch::BLACK, 0x88));

        json::Value::Object(colors_owned)
    }

    fn semantic_colors(&self) -> json::Value {
        let mut colors_owned = json::Map::new();
        let colors = &mut colors_owned;

        style_rule(colors, "boolean", (blue(4), FontStyle::Clear));
        style_rule(colors, "comment", green(0));
        style_rule(colors, "comment.documentation", green(2));
        style_rule(colors, "keyword", (yellow(4), FontStyle::Bold));
        style_rule(colors, "*.unsafe", red(0));
        style_rule(colors, "function.unsafe", red(0));
        style_rule(colors, "operator.unsafe", red(0));
        style_rule(colors, "property", green(2));
        style_rule(colors, "function", cyan(3));
        style_rule(colors, "namespace", green(4));
        style_rule(colors, "macro", blue(2));
        style_rule(colors, "formatSpecifier", blue(2));
        style_rule(colors, "escapeSequence", blue(2));
        style_rule(colors, "variable", fg(7));
        style_rule(colors, "variable.static", blue(4));
        style_rule(colors, "parameter", orange(2));
        style_rule(colors, "struct", cyan(1));
        style_rule(colors, "enum", cyan(1));
        style_rule(colors, "union", cyan(1));
        style_rule(colors, "typeAlias", cyan(1));
        style_rule(colors, "builtinType", cyan(2));
        style_rule(colors, "type", cyan(1));
        style_rule(colors, "interface", cyan(2));
        style_rule(colors, "enumMember", blue(4));
        style_rule(colors, "typeParameter", orange(2));
        style_rule(colors, "lifetime", (orange(2), FontStyle::Italic));
        style_rule(colors, "number", green(4));
        style_rule(colors, "string", red(2));
        style_rule(colors, "attribute", blue(2));
        style_rule(colors, "function.attribute", blue(2));
        style_rule(colors, "punctuation", fg(7));
        style_rule(colors, "*.mutable", FontStyle::Underline);
        style_rule(colors, "*.consuming", FontStyle::Italic);

        json::Value::Object(colors_owned)
    }

    fn textmate_colors(&self) -> json::Value {
        let mut colors_owned = Vec::new();
        let colors = &mut colors_owned;

        textmate_rule(
            colors,
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
            green(2),
        );

        textmate_rule(
            colors,
            &[
                "entity.name.function",
                "meta.function-call.generic.python",
                "support.function",
            ],
            cyan(3),
        );

        textmate_rule(
            colors,
            &[
                "entity.name.module",
                "entity.name.namespace",
                "entity.name.type.namespace",
                "storage.modifier.import",
                "storage.modifier.package",
            ],
            (green(4), FontStyle::Clear),
        );

        textmate_rule(
            colors,
            &["entity.name.macro", "entity.name.other.preprocessor.macro"],
            blue(2),
        );

        textmate_rule(colors, &["constant.character.escape"], blue(2));

        textmate_rule(colors, &["variable"], fg(7));

        textmate_rule(
            colors,
            &["entity.name.variable.parameter", "variable.parameter"],
            orange(2),
        );

        textmate_rule(
            colors,
            &[
                "constant",
                "entity.name.constant",
                "variable.other.enummember",
                "support.constant",
            ],
            blue(4),
        );

        textmate_rule(colors, &["meta.mutable"], FontStyle::Underline);

        textmate_rule(
            colors,
            &[
                "entity.name.type",
                "storage.type",
                "support.class",
                "support.type",
            ],
            (cyan(1), FontStyle::Clear),
        );

        textmate_rule(
            colors,
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
            (cyan(2), FontStyle::Clear),
        );

        textmate_rule(colors, &["entity.name.type.parameter"], orange(2));

        textmate_rule(
            colors,
            &[
                "storage.modifier.lifetime.rust",
                "entity.name.lifetime.rust",
                "entity.name.type.lifetime",
                "punctuation.definition.lifetime",
            ],
            (orange(2), FontStyle::Italic),
        );

        textmate_rule(
            colors,
            &["constant.numeric", "keyword.other.unit"],
            (green(4), FontStyle::Clear),
        );

        textmate_rule(
            colors,
            &["comment", "punctuation.definition.comment"],
            green(0),
        );

        textmate_rule(
            colors,
            &[
                "constant.character",
                "punctuation.definition.char",
                "punctuation.definition.string",
                "string",
            ],
            red(2),
        );

        textmate_rule(
            colors,
            &[
                "entity.name.function.decorator",
                "meta.attribute",
                "punctuation.brackets.attribute",
                "punctuation.definition.annotation",
                "punctuation.definition.attribute",
                "punctuation.definition.decorator",
                "storage.modifier.attribute",
                "storage.type.annotation",
            ],
            (blue(2), FontStyle::Clear),
        );

        textmate_rule(
            colors,
            &[
                "constant.language.null",
                "entity.name.tag",
                "keyword.operator.expression",
                "keyword.operator.logical",
                "keyword.operator.new",
                "keyword.operator.wordlike",
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
            (yellow(4), FontStyle::Bold),
        );

        textmate_rule(
            colors,
            &[
                "entity.other.attribute-name.class",
                "entity.other.attribute-name.id",
            ],
            cyan(1),
        );

        textmate_rule(colors, &["keyword.other.unsafe"], red(0));

        textmate_rule(
            colors,
            &[
                "keyword.operator.logical.rust",
                "keyword.operator",
                "punctuation",
            ],
            (fg(7), FontStyle::Clear),
        );

        textmate_rule(colors, &["markup.italic"], FontStyle::Italic);
        textmate_rule(colors, &["markup.bold"], FontStyle::Bold);
        textmate_rule(colors, &["markup.heading"], FontStyle::Underline);

        textmate_rule(
            colors,
            &[
                "markup.inserted.diff",
                "punctuation.definition.inserted.diff",
            ],
            green(ColorLightnessPreset::DiffFg),
        );
        textmate_rule(
            colors,
            &["markup.deleted.diff", "punctuation.definition.deleted.diff"],
            red(ColorLightnessPreset::DiffFg),
        );

        json::Value::Array(colors_owned)
    }
}

fn textmate_rule(rules: &mut Vec<json::Value>, scopes: &[&str], style: impl Into<Style>) {
    let mut map = json::Map::new();

    map.insert("scope".to_string(), scopes.iter().copied().collect());
    map.insert("settings".to_string(), style.into().as_json_value(true));

    rules.push(json::Value::Object(map));
}

fn color_rule(map: &mut json::Map<String, json::Value>, scope_name: &str, color: impl Into<Color>) {
    map.insert(scope_name.to_string(), (&color.into()).into());
}

fn style_rule(map: &mut json::Map<String, json::Value>, scope_name: &str, style: impl Into<Style>) {
    map.insert(scope_name.to_string(), style.into().as_json_value(false));
}
