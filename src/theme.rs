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

        color_rule(colors, "activityBar.activeBorder", ZENBURN_FG);
        color_rule(colors, "activityBar.background", ZENBURN_BG_MINUS_05);
        color_rule(colors, "activityBar.foreground", ZENBURN_FG);
        color_rule(colors, "activityBar.inactiveForeground", ZENBURN_BG_PLUS_3);
        color_rule(
            colors,
            "activityBarBadge.background",
            blue(LightnessLevel(2)),
        );
        color_rule(colors, "activityBarBadge.foreground", ZENBURN_BG);
        color_rule(colors, "badge.background", ZENBURN_BG_PLUS_2);
        color_rule(colors, "badge.foreground", ZENBURN_FG);
        color_rule(colors, "button.background", blue(LightnessLevel(2)));
        color_rule(colors, "button.foreground", ZENBURN_BG);
        color_rule(colors, "button.hoverBackground", ZENBURN_FG);
        color_rule(
            colors,
            "debugIcon.breakpointForeground",
            red(LightnessLevel(2)),
        );
        color_rule(
            colors,
            "diffEditor.insertedTextBackground",
            (green(LightnessLevel(4)), 0x33),
        );
        color_rule(
            colors,
            "diffEditor.removedTextBackground",
            (red(LightnessLevel(2)), 0x33),
        );
        color_rule(colors, "editor.background", ZENBURN_BG);
        color_rule(
            colors,
            "editor.findMatchBackground",
            (blue(LightnessLevel(2)), 0x66),
        );
        color_rule(
            colors,
            "editor.findMatchHighlightBackground",
            (blue(LightnessLevel(2)), 0x44),
        );
        color_rule(
            colors,
            "editor.foldBackground",
            (blue(LightnessLevel(2)), 0x22),
        );
        color_rule(colors, "editor.foreground", ZENBURN_FG);
        color_rule(colors, "editor.hoverHighlightBackground", ZENBURN_BG_PLUS_1);
        color_rule(
            colors,
            "editor.lineHighlightBackground",
            ZENBURN_BG_MINUS_05,
        );
        color_rule(colors, "editor.selectionBackground", ZENBURN_BG_MINUS_1);
        color_rule(
            colors,
            "editor.selectionHighlightBackground",
            ZENBURN_BG_PLUS_2,
        );
        color_rule(
            colors,
            "editor.symbolHighlightBackground",
            ZENBURN_BG_PLUS_2,
        );
        color_rule(colors, "editor.wordHighlightBackground", ZENBURN_BG_PLUS_2);
        color_rule(
            colors,
            "editor.wordHighlightStrongBackground",
            ZENBURN_BG_PLUS_2,
        );
        color_rule(colors, "editorCursor.foreground", ZENBURN_FG_PLUS_1);
        color_rule(colors, "editorError.foreground", red(LightnessLevel(2)));
        color_rule(
            colors,
            "editorGroup.dropBackground",
            (blue(LightnessLevel(2)), 0x22),
        );
        color_rule(
            colors,
            "editorGroupHeader.noTabsBackground",
            ZENBURN_BG_PLUS_05,
        );
        color_rule(
            colors,
            "editorGroupHeader.tabsBackground",
            ZENBURN_BG_MINUS_1,
        );
        color_rule(
            colors,
            "editorGutter.addedBackground",
            green(LightnessLevel(0)),
        );
        color_rule(
            colors,
            "editorGutter.deletedBackground",
            red(LightnessLevel(2)),
        );
        color_rule(
            colors,
            "editorGutter.modifiedBackground",
            yellow(LightnessLevel(2)),
        );
        color_rule(colors, "editorGroup.border", ZENBURN_BG_PLUS_2);
        color_rule(
            colors,
            "editorIndentGuide.activeBackground",
            ZENBURN_BG_PLUS_2,
        );
        color_rule(colors, "editorIndentGuide.background", ZENBURN_BG_PLUS_1);
        color_rule(
            colors,
            "editorLightBulb.foreground",
            yellow(LightnessLevel(4)),
        );
        color_rule(colors, "editorLineNumber.foreground", ZENBURN_BG_PLUS_2);
        color_rule(
            colors,
            "editorLink.activeForeground",
            blue(LightnessLevel(2)),
        );
        color_rule(
            colors,
            "editorOverviewRuler.addedForeground",
            green(LightnessLevel(0)),
        );
        color_rule(colors, "editorOverviewRuler.border", ZENBURN_BG_PLUS_2);
        color_rule(
            colors,
            "editorOverviewRuler.deletedForeground",
            red(LightnessLevel(2)),
        );
        color_rule(
            colors,
            "editorOverviewRuler.errorForeground",
            red(LightnessLevel(0)),
        );
        color_rule(
            colors,
            "editorOverviewRuler.findMatchForeground",
            (blue(LightnessLevel(2)), 0x88),
        );
        color_rule(
            colors,
            "editorOverviewRuler.modifiedForeground",
            yellow(LightnessLevel(2)),
        );
        color_rule(
            colors,
            "editorWarning.foreground",
            orange(LightnessLevel(2)),
        );
        color_rule(colors, "editorWidget.background", ZENBURN_BG_MINUS_05);
        color_rule(colors, "editorWidget.border", ZENBURN_BG_PLUS_1);
        color_rule(colors, "focusBorder", ZENBURN_BG_PLUS_2);
        color_rule(colors, "foreground", ZENBURN_FG);
        color_rule(
            colors,
            "gitDecoration.ignoredResourceForeground",
            ZENBURN_BG_PLUS_3,
        );
        color_rule(
            colors,
            "gitDecoration.modifiedResourceForeground",
            yellow(LightnessLevel(4)),
        );
        color_rule(
            colors,
            "gitDecoration.untrackedResourceForeground",
            green(LightnessLevel(2)),
        );
        color_rule(colors, "input.background", (Oklch::WHITE, 0x0A)); // input field lightens what is behind it
        color_rule(colors, "input.foreground", ZENBURN_FG);
        color_rule(colors, "input.placeholderForeground", ZENBURN_BG_PLUS_3);
        color_rule(colors, "list.activeSelectionBackground", ZENBURN_BG_PLUS_1);
        color_rule(colors, "list.activeSelectionForeground", ZENBURN_FG);
        color_rule(colors, "list.errorForeground", red(LightnessLevel(2)));
        color_rule(colors, "list.focusBackground", ZENBURN_BG_PLUS_1);
        color_rule(colors, "list.highlightForeground", blue(LightnessLevel(4)));
        color_rule(colors, "list.hoverBackground", ZENBURN_BG);
        color_rule(
            colors,
            "list.inactiveSelectionBackground",
            ZENBURN_BG_PLUS_05,
        );
        color_rule(colors, "list.warningForeground", orange(LightnessLevel(2)));
        color_rule(colors, "minimap.errorHighlight", red(LightnessLevel(2)));
        color_rule(
            colors,
            "minimap.findMatchHighlight",
            (blue(LightnessLevel(2)), 0x66),
        );
        color_rule(
            colors,
            "minimap.selectionHighlight",
            (ZENBURN_BG_MINUS_1, 0x88),
        );
        color_rule(colors, "panel.background", ZENBURN_BG_PLUS_05);
        color_rule(colors, "panel.border", ZENBURN_BG_PLUS_2);
        color_rule(colors, "panelTitle.activeForeground", ZENBURN_FG);
        color_rule(colors, "peekView.border", ZENBURN_BG_PLUS_3);
        color_rule(colors, "peekViewEditor.background", ZENBURN_BG);
        color_rule(
            colors,
            "peekViewEditor.matchHighlightBackground",
            (blue(LightnessLevel(2)), 0x66),
        );
        color_rule(colors, "peekViewResult.background", ZENBURN_BG_MINUS_05);
        color_rule(colors, "peekViewResult.fileForeground", ZENBURN_FG);
        color_rule(colors, "peekViewResult.lineForeground", (ZENBURN_FG, 0x99));
        color_rule(
            colors,
            "peekViewResult.matchHighlightBackground",
            (blue(LightnessLevel(2)), 0x44),
        );
        color_rule(
            colors,
            "peekViewResult.selectionBackground",
            ZENBURN_BG_PLUS_1,
        );
        color_rule(colors, "peekViewResult.selectionForeground", ZENBURN_FG);
        color_rule(colors, "peekViewTitle.background", ZENBURN_BG_MINUS_05);
        color_rule(
            colors,
            "peekViewTitleDescription.foreground",
            blue(LightnessLevel(2)),
        );
        color_rule(colors, "peekViewTitleLabel.foreground", ZENBURN_FG_PLUS_1);
        color_rule(colors, "progressBar.background", blue(LightnessLevel(2)));
        color_rule(
            colors,
            "rust_analyzer.inlayHints.foreground",
            green(LightnessLevel(0)),
        );
        color_rule(colors, "scrollbar.shadow", (Oklch::BLACK, 0x88));
        color_rule(colors, "selection.background", (Oklch::WHITE, 0x55));
        color_rule(colors, "settings.headerForeground", ZENBURN_FG_PLUS_1);
        color_rule(
            colors,
            "settings.modifiedItemIndicator",
            blue(LightnessLevel(2)),
        );
        color_rule(colors, "sideBar.background", ZENBURN_BG_MINUS_05);
        color_rule(colors, "sideBar.foreground", ZENBURN_FG);
        color_rule(colors, "sideBarTitle.foreground", ZENBURN_FG_PLUS_1);
        color_rule(colors, "statusBar.background", ZENBURN_BG_MINUS_1);
        color_rule(colors, "statusBar.debuggingBackground", ZENBURN_BG_MINUS_1);
        color_rule(colors, "statusBar.foreground", green(LightnessLevel(2)));
        color_rule(
            colors,
            "statusBar.debuggingForeground",
            orange(LightnessLevel(2)),
        );
        color_rule(colors, "statusBar.noFolderBackground", ZENBURN_BG_MINUS_1);
        color_rule(colors, "tab.activeForeground", ZENBURN_FG);
        color_rule(colors, "tab.border", ZENBURN_BG);
        color_rule(colors, "tab.inactiveBackground", ZENBURN_BG_MINUS_1);
        color_rule(colors, "tab.inactiveForeground", ZENBURN_BG_PLUS_3);
        color_rule(colors, "terminal.ansiBlack", ZENBURN_BG_MINUS_1);
        color_rule(colors, "terminal.ansiBlue", blue(LightnessLevel(2)));
        color_rule(colors, "terminal.ansiBrightBlack", ZENBURN_BG_PLUS_3);
        color_rule(colors, "terminal.ansiBrightBlue", blue(LightnessLevel(4)));
        color_rule(colors, "terminal.ansiBrightCyan", cyan(LightnessLevel(3)));
        color_rule(colors, "terminal.ansiBrightGreen", green(LightnessLevel(4)));
        color_rule(
            colors,
            "terminal.ansiBrightMagenta",
            orange(LightnessLevel(2)),
        );
        color_rule(colors, "terminal.ansiBrightRed", red(LightnessLevel(2)));
        color_rule(colors, "terminal.ansiBrightWhite", ZENBURN_FG_PLUS_1);
        color_rule(
            colors,
            "terminal.ansiBrightYellow",
            yellow(LightnessLevel(4)),
        );
        color_rule(colors, "terminal.ansiCyan", cyan(LightnessLevel(1)));
        color_rule(colors, "terminal.ansiGreen", green(LightnessLevel(0)));
        color_rule(colors, "terminal.ansiMagenta", orange(LightnessLevel(2)));
        color_rule(colors, "terminal.ansiRed", red(LightnessLevel(2)));
        color_rule(colors, "terminal.ansiWhite", ZENBURN_FG);
        color_rule(colors, "terminal.ansiYellow", yellow(LightnessLevel(2)));
        color_rule(colors, "terminal.foreground", ZENBURN_FG);
        color_rule(colors, "terminal.selectionBackground", ZENBURN_BG_MINUS_05); // Lighter than normal selection background to compensate for lighter terminal background
        color_rule(colors, "terminalCursor.foreground", ZENBURN_FG_PLUS_1);
        color_rule(colors, "textLink.activeForeground", blue(LightnessLevel(2)));
        color_rule(colors, "textLink.foreground", blue(LightnessLevel(2)));
        color_rule(colors, "textPreformat.foreground", ZENBURN_FG); // inline code in e.g. Settings page
        color_rule(colors, "titleBar.activeBackground", ZENBURN_BG_MINUS_05);
        color_rule(colors, "titleBar.activeForeground", ZENBURN_FG);
        color_rule(colors, "titleBar.inactiveBackground", ZENBURN_BG_MINUS_05);
        color_rule(colors, "titleBar.inactiveForeground", ZENBURN_BG_PLUS_3);
        color_rule(colors, "widget.shadow", (Oklch::BLACK, 0x88));

        json::Value::Object(colors_owned)
    }

    fn semantic_colors(&self) -> json::Value {
        let mut colors_owned = json::Map::new();
        let colors = &mut colors_owned;

        style_rule(
            colors,
            "boolean",
            (blue(LightnessLevel(4)), FontStyle::Clear),
        );
        style_rule(colors, "comment", green(LightnessLevel(0)));
        style_rule(colors, "comment.documentation", green(LightnessLevel(2)));
        style_rule(
            colors,
            "keyword",
            (yellow(LightnessLevel(4)), FontStyle::Bold),
        );
        style_rule(colors, "*.unsafe", red(LightnessLevel(0)));
        style_rule(colors, "function.unsafe", red(LightnessLevel(0)));
        style_rule(colors, "operator.unsafe", red(LightnessLevel(0)));
        style_rule(colors, "property", green(LightnessLevel(2)));
        style_rule(colors, "function", cyan(LightnessLevel(3)));
        style_rule(colors, "namespace", green(LightnessLevel(4)));
        style_rule(colors, "macro", blue(LightnessLevel(2)));
        style_rule(colors, "formatSpecifier", blue(LightnessLevel(2)));
        style_rule(colors, "escapeSequence", blue(LightnessLevel(2)));
        style_rule(colors, "variable", ZENBURN_FG);
        style_rule(colors, "variable.static", blue(LightnessLevel(4)));
        style_rule(colors, "parameter", orange(LightnessLevel(2)));
        style_rule(colors, "struct", cyan(LightnessLevel(1)));
        style_rule(colors, "enum", cyan(LightnessLevel(1)));
        style_rule(colors, "union", cyan(LightnessLevel(1)));
        style_rule(colors, "typeAlias", cyan(LightnessLevel(1)));
        style_rule(colors, "builtinType", cyan(LightnessLevel(2)));
        style_rule(colors, "type", cyan(LightnessLevel(1)));
        style_rule(colors, "interface", cyan(LightnessLevel(2)));
        style_rule(colors, "enumMember", blue(LightnessLevel(4)));
        style_rule(colors, "typeParameter", orange(LightnessLevel(2)));
        style_rule(
            colors,
            "lifetime",
            (orange(LightnessLevel(2)), FontStyle::Italic),
        );
        style_rule(colors, "number", green(LightnessLevel(4)));
        style_rule(colors, "string", red(LightnessLevel(2)));
        style_rule(colors, "attribute", blue(LightnessLevel(2)));
        style_rule(colors, "function.attribute", blue(LightnessLevel(2)));
        style_rule(colors, "punctuation", ZENBURN_FG);
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
            green(LightnessLevel(2)),
        );

        textmate_rule(
            colors,
            &[
                "entity.name.function",
                "meta.function-call.generic.python",
                "support.function",
            ],
            cyan(LightnessLevel(3)),
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
            (green(LightnessLevel(4)), FontStyle::Clear),
        );

        textmate_rule(
            colors,
            &["entity.name.macro", "entity.name.other.preprocessor.macro"],
            blue(LightnessLevel(2)),
        );

        textmate_rule(
            colors,
            &["constant.character.escape"],
            blue(LightnessLevel(2)),
        );

        textmate_rule(colors, &["variable"], ZENBURN_FG);

        textmate_rule(
            colors,
            &["entity.name.variable.parameter", "variable.parameter"],
            orange(LightnessLevel(2)),
        );

        textmate_rule(
            colors,
            &[
                "constant",
                "entity.name.constant",
                "variable.other.enummember",
                "support.constant",
            ],
            blue(LightnessLevel(4)),
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
            (cyan(LightnessLevel(1)), FontStyle::Clear),
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
            (cyan(LightnessLevel(2)), FontStyle::Clear),
        );

        textmate_rule(
            colors,
            &["entity.name.type.parameter"],
            orange(LightnessLevel(2)),
        );

        textmate_rule(
            colors,
            &[
                "storage.modifier.lifetime.rust",
                "entity.name.lifetime.rust",
                "entity.name.type.lifetime",
                "punctuation.definition.lifetime",
            ],
            (orange(LightnessLevel(2)), FontStyle::Italic),
        );

        textmate_rule(
            colors,
            &["constant.numeric", "keyword.other.unit"],
            (green(LightnessLevel(4)), FontStyle::Clear),
        );

        textmate_rule(
            colors,
            &["comment", "punctuation.definition.comment"],
            green(LightnessLevel(0)),
        );

        textmate_rule(
            colors,
            &[
                "constant.character",
                "punctuation.definition.char",
                "punctuation.definition.string",
                "string",
            ],
            red(LightnessLevel(2)),
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
            (blue(LightnessLevel(2)), FontStyle::Clear),
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
            (yellow(LightnessLevel(4)), FontStyle::Bold),
        );

        textmate_rule(
            colors,
            &[
                "entity.other.attribute-name.class",
                "entity.other.attribute-name.id",
            ],
            cyan(LightnessLevel(1)),
        );

        textmate_rule(colors, &["keyword.other.unsafe"], red(LightnessLevel(0)));

        textmate_rule(
            colors,
            &[
                "keyword.operator.logical.rust",
                "keyword.operator",
                "punctuation",
            ],
            (ZENBURN_FG, FontStyle::Clear),
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
            green(LightnessLevel(4)),
        );
        textmate_rule(
            colors,
            &["markup.deleted.diff", "punctuation.definition.deleted.diff"],
            red(LightnessLevel(2)),
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
