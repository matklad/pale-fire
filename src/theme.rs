use crate::palette::*;
use crate::style::{Color, FontStyle, Rgb, Style};

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
        color_rule(colors, "activityBarBadge.background", ZENBURN_BLUE_MINUS_3);
        color_rule(colors, "activityBarBadge.foreground", ZENBURN_FG);
        color_rule(colors, "badge.background", ZENBURN_BG_PLUS_2);
        color_rule(colors, "badge.foreground", ZENBURN_FG);
        color_rule(colors, "button.background", ZENBURN_BLUE_PLUS_1);
        color_rule(colors, "button.foreground", ZENBURN_BG);
        color_rule(colors, "button.hoverBackground", ZENBURN_FG);
        color_rule(colors, "debugIcon.breakpointForeground", ZENBURN_RED);
        color_rule(
            colors,
            "diffEditor.insertedTextBackground",
            (ZENBURN_GREEN_PLUS_4, 0x33),
        );
        color_rule(
            colors,
            "diffEditor.removedTextBackground",
            (ZENBURN_RED_PLUS_2, 0x33),
        );
        color_rule(colors, "editor.background", ZENBURN_BG);
        color_rule(
            colors,
            "editor.findMatchBackground",
            (ZENBURN_BLUE_PLUS_1, 0x66),
        );
        color_rule(
            colors,
            "editor.findMatchHighlightBackground",
            (ZENBURN_BLUE_PLUS_1, 0x44),
        );
        color_rule(colors, "editor.foldBackground", (ZENBURN_BLUE_PLUS_1, 0x22));
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
        color_rule(colors, "editorError.foreground", ZENBURN_RED_PLUS_1);
        color_rule(
            colors,
            "editorGroup.dropBackground",
            (ZENBURN_BLUE_PLUS_1, 0x22),
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
        color_rule(colors, "editorGutter.addedBackground", ZENBURN_GREEN);
        color_rule(colors, "editorGutter.deletedBackground", ZENBURN_RED);
        color_rule(
            colors,
            "editorGutter.modifiedBackground",
            ZENBURN_YELLOW_MINUS_2,
        );
        color_rule(colors, "editorGroup.border", ZENBURN_BG_PLUS_2);
        color_rule(
            colors,
            "editorIndentGuide.activeBackground",
            ZENBURN_BG_PLUS_2,
        );
        color_rule(colors, "editorIndentGuide.background", ZENBURN_BG_PLUS_1);
        color_rule(colors, "editorLightBulb.foreground", ZENBURN_YELLOW);
        color_rule(colors, "editorLineNumber.foreground", ZENBURN_BG_PLUS_2);
        color_rule(colors, "editorLink.activeForeground", ZENBURN_BLUE_PLUS_1);
        color_rule(colors, "editorOverviewRuler.addedForeground", ZENBURN_GREEN);
        color_rule(colors, "editorOverviewRuler.border", ZENBURN_BG_PLUS_2);
        color_rule(colors, "editorOverviewRuler.deletedForeground", ZENBURN_RED);
        color_rule(
            colors,
            "editorOverviewRuler.errorForeground",
            ZENBURN_RED_MINUS_2,
        );
        color_rule(
            colors,
            "editorOverviewRuler.findMatchForeground",
            (ZENBURN_BLUE_PLUS_1, 0x88),
        );
        color_rule(
            colors,
            "editorOverviewRuler.modifiedForeground",
            ZENBURN_YELLOW_MINUS_2,
        );
        color_rule(colors, "editorWarning.foreground", ZENBURN_ORANGE);
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
            ZENBURN_YELLOW,
        );
        color_rule(
            colors,
            "gitDecoration.untrackedResourceForeground",
            ZENBURN_GREEN_PLUS_3,
        );
        color_rule(colors, "input.background", (Rgb(0xFFFFFF), 0x0A)); // input field lightens what is behind it
        color_rule(colors, "input.foreground", ZENBURN_FG);
        color_rule(colors, "input.placeholderForeground", ZENBURN_BG_PLUS_3);
        color_rule(colors, "list.activeSelectionBackground", ZENBURN_BG_PLUS_1);
        color_rule(colors, "list.activeSelectionForeground", ZENBURN_FG);
        color_rule(colors, "list.errorForeground", ZENBURN_RED_PLUS_1);
        color_rule(colors, "list.focusBackground", ZENBURN_BG_PLUS_1);
        color_rule(colors, "list.highlightForeground", ZENBURN_BLUE_PLUS_3);
        color_rule(colors, "list.hoverBackground", ZENBURN_BG);
        color_rule(
            colors,
            "list.inactiveSelectionBackground",
            ZENBURN_BG_PLUS_05,
        );
        color_rule(colors, "list.warningForeground", ZENBURN_ORANGE);
        color_rule(colors, "minimap.errorHighlight", ZENBURN_RED_PLUS_2);
        color_rule(
            colors,
            "minimap.findMatchHighlight",
            (ZENBURN_BLUE_PLUS_1, 0x66),
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
            (ZENBURN_BLUE_PLUS_1, 0x66),
        );
        color_rule(colors, "peekViewResult.background", ZENBURN_BG_MINUS_05);
        color_rule(colors, "peekViewResult.fileForeground", ZENBURN_FG);
        color_rule(colors, "peekViewResult.lineForeground", (ZENBURN_FG, 0x99));
        color_rule(
            colors,
            "peekViewResult.matchHighlightBackground",
            (ZENBURN_BLUE_PLUS_1, 0x44),
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
            ZENBURN_BLUE_PLUS_1,
        );
        color_rule(colors, "peekViewTitleLabel.foreground", ZENBURN_FG_PLUS_1);
        color_rule(colors, "progressBar.background", ZENBURN_BLUE_PLUS_1);
        color_rule(colors, "rust_analyzer.inlayHints.foreground", ZENBURN_GREEN);
        color_rule(colors, "scrollbar.shadow", (Rgb(0x000000), 0x88));
        color_rule(colors, "selection.background", (Rgb(0xFFFFFF), 0x55));
        color_rule(colors, "settings.headerForeground", ZENBURN_FG_PLUS_1);
        color_rule(
            colors,
            "settings.modifiedItemIndicator",
            ZENBURN_BLUE_PLUS_1,
        );
        color_rule(colors, "sideBar.background", ZENBURN_BG_MINUS_05);
        color_rule(colors, "sideBar.foreground", ZENBURN_FG);
        color_rule(colors, "sideBarTitle.foreground", ZENBURN_FG_PLUS_1);
        color_rule(colors, "statusBar.background", ZENBURN_BG_MINUS_1);
        color_rule(colors, "statusBar.debuggingBackground", ZENBURN_BG_MINUS_1);
        color_rule(colors, "statusBar.foreground", ZENBURN_GREEN_PLUS_1);
        color_rule(colors, "statusBar.debuggingForeground", ZENBURN_ORANGE);
        color_rule(colors, "statusBar.noFolderBackground", ZENBURN_BG_MINUS_1);
        color_rule(colors, "tab.activeForeground", ZENBURN_FG);
        color_rule(colors, "tab.border", ZENBURN_BG);
        color_rule(colors, "tab.inactiveBackground", ZENBURN_BG_MINUS_1);
        color_rule(colors, "tab.inactiveForeground", ZENBURN_BG_PLUS_3);
        color_rule(colors, "terminal.ansiBlack", ZENBURN_BG_MINUS_1);
        color_rule(colors, "terminal.ansiBlue", ZENBURN_BLUE_PLUS_1);
        color_rule(colors, "terminal.ansiBrightBlack", ZENBURN_BG_PLUS_3);
        color_rule(colors, "terminal.ansiBrightBlue", ZENBURN_BLUE_PLUS_3);
        color_rule(colors, "terminal.ansiBrightCyan", ZENBURN_CYAN);
        color_rule(colors, "terminal.ansiBrightGreen", ZENBURN_GREEN_PLUS_4);
        color_rule(colors, "terminal.ansiBrightMagenta", ZENBURN_ORANGE);
        color_rule(colors, "terminal.ansiBrightRed", ZENBURN_RED_PLUS_2);
        color_rule(colors, "terminal.ansiBrightWhite", ZENBURN_FG_PLUS_1);
        color_rule(colors, "terminal.ansiBrightYellow", ZENBURN_YELLOW);
        color_rule(colors, "terminal.ansiCyan", ZENBURN_BLUE_MINUS_1);
        color_rule(colors, "terminal.ansiGreen", ZENBURN_GREEN);
        color_rule(colors, "terminal.ansiMagenta", ZENBURN_ORANGE);
        color_rule(colors, "terminal.ansiRed", ZENBURN_RED);
        color_rule(colors, "terminal.ansiWhite", ZENBURN_FG);
        color_rule(colors, "terminal.ansiYellow", ZENBURN_YELLOW_MINUS_2);
        color_rule(colors, "terminal.foreground", ZENBURN_FG);
        color_rule(colors, "terminal.selectionBackground", ZENBURN_BG_MINUS_05); // Lighter than normal selection background to compensate for lighter terminal background
        color_rule(colors, "terminalCursor.foreground", ZENBURN_FG_PLUS_1);
        color_rule(colors, "textLink.activeForeground", ZENBURN_BLUE_PLUS_1);
        color_rule(colors, "textLink.foreground", ZENBURN_BLUE_PLUS_1);
        color_rule(colors, "textPreformat.foreground", ZENBURN_FG); // inline code in e.g. Settings page
        color_rule(colors, "titleBar.activeBackground", ZENBURN_BG_MINUS_05);
        color_rule(colors, "titleBar.activeForeground", ZENBURN_FG);
        color_rule(colors, "titleBar.inactiveBackground", ZENBURN_BG_MINUS_05);
        color_rule(colors, "titleBar.inactiveForeground", ZENBURN_BG_PLUS_3);
        color_rule(colors, "widget.shadow", (Rgb(0x000000), 0x88));

        json::Value::Object(colors_owned)
    }

    fn semantic_colors(&self) -> json::Value {
        let mut colors_owned = json::Map::new();
        let colors = &mut colors_owned;

        style_rule(colors, "boolean", (ZENBURN_BLUE_PLUS_3, FontStyle::Clear));
        style_rule(colors, "comment", ZENBURN_GREEN);
        style_rule(colors, "comment.documentation", ZENBURN_GREEN_PLUS_2);
        style_rule(colors, "keyword", (ZENBURN_YELLOW, FontStyle::Bold));
        style_rule(colors, "*.unsafe", ZENBURN_RED_MINUS_1);
        style_rule(colors, "function.unsafe", ZENBURN_RED_MINUS_1);
        style_rule(colors, "operator.unsafe", ZENBURN_RED_MINUS_1);
        style_rule(colors, "property", ZENBURN_GREEN_PLUS_3);
        style_rule(colors, "function", ZENBURN_CYAN);
        style_rule(colors, "namespace", ZENBURN_GREEN_PLUS_4);
        style_rule(colors, "macro", ZENBURN_BLUE_PLUS_1);
        style_rule(colors, "formatSpecifier", ZENBURN_BLUE_PLUS_1);
        style_rule(colors, "escapeSequence", ZENBURN_BLUE_PLUS_1);
        style_rule(colors, "variable", ZENBURN_FG);
        style_rule(colors, "variable.static", ZENBURN_BLUE_PLUS_3);
        style_rule(colors, "parameter", ZENBURN_ORANGE);
        style_rule(colors, "struct", ZENBURN_BLUE_MINUS_1);
        style_rule(colors, "enum", ZENBURN_BLUE_MINUS_1);
        style_rule(colors, "union", ZENBURN_BLUE_MINUS_1);
        style_rule(colors, "typeAlias", ZENBURN_BLUE_MINUS_1);
        style_rule(colors, "builtinType", ZENBURN_BLUE);
        style_rule(colors, "type", ZENBURN_BLUE_MINUS_1);
        style_rule(colors, "interface", ZENBURN_BLUE);
        style_rule(colors, "enumMember", ZENBURN_BLUE_PLUS_3);
        style_rule(colors, "typeParameter", ZENBURN_ORANGE);
        style_rule(colors, "lifetime", (ZENBURN_ORANGE, FontStyle::Italic));
        style_rule(colors, "number", ZENBURN_GREEN_PLUS_4);
        style_rule(colors, "string", ZENBURN_RED);
        style_rule(colors, "attribute", ZENBURN_BLUE_PLUS_1);
        style_rule(colors, "function.attribute", ZENBURN_BLUE_PLUS_1);
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
            ZENBURN_GREEN_PLUS_3,
        );

        textmate_rule(
            colors,
            &[
                "entity.name.function",
                "meta.function-call.generic.python",
                "support.function",
            ],
            ZENBURN_CYAN,
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
            (ZENBURN_GREEN_PLUS_4, FontStyle::Clear),
        );

        textmate_rule(
            colors,
            &["entity.name.macro", "entity.name.other.preprocessor.macro"],
            ZENBURN_BLUE_PLUS_1,
        );

        textmate_rule(colors, &["constant.character.escape"], ZENBURN_BLUE_PLUS_1);

        textmate_rule(colors, &["variable"], ZENBURN_FG);

        textmate_rule(
            colors,
            &["entity.name.variable.parameter", "variable.parameter"],
            ZENBURN_ORANGE,
        );

        textmate_rule(
            colors,
            &[
                "constant",
                "entity.name.constant",
                "variable.other.enummember",
                "support.constant",
            ],
            ZENBURN_BLUE_PLUS_3,
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
            (ZENBURN_BLUE_MINUS_1, FontStyle::Clear),
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
            (ZENBURN_BLUE, FontStyle::Clear),
        );

        textmate_rule(colors, &["entity.name.type.parameter"], ZENBURN_ORANGE);

        textmate_rule(
            colors,
            &[
                "storage.modifier.lifetime.rust",
                "entity.name.lifetime.rust",
                "entity.name.type.lifetime",
                "punctuation.definition.lifetime",
            ],
            (ZENBURN_ORANGE, FontStyle::Italic),
        );

        textmate_rule(
            colors,
            &["constant.numeric", "keyword.other.unit"],
            (ZENBURN_GREEN_PLUS_4, FontStyle::Clear),
        );

        textmate_rule(
            colors,
            &["comment", "punctuation.definition.comment"],
            ZENBURN_GREEN,
        );

        textmate_rule(
            colors,
            &[
                "constant.character",
                "punctuation.definition.char",
                "punctuation.definition.string",
                "string",
            ],
            ZENBURN_RED,
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
            (ZENBURN_BLUE_PLUS_1, FontStyle::Clear),
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
            (ZENBURN_YELLOW, FontStyle::Bold),
        );

        textmate_rule(
            colors,
            &[
                "entity.other.attribute-name.class",
                "entity.other.attribute-name.id",
            ],
            ZENBURN_BLUE_MINUS_1,
        );

        textmate_rule(colors, &["keyword.other.unsafe"], ZENBURN_RED_MINUS_1);

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

        json::Value::Array(colors_owned)
    }
}

fn textmate_rule(rules: &mut Vec<json::Value>, scopes: &[&str], style: impl Into<Style>) {
    let mut map = json::Map::new();

    map.insert(
        "scope".to_string(),
        json::Value::Array(
            scopes
                .iter()
                .map(|s| s.to_string())
                .map(json::Value::String)
                .collect(),
        ),
    );

    map.insert("settings".to_string(), style.into().as_json_value(true));

    rules.push(json::Value::Object(map));
}

fn color_rule(map: &mut json::Map<String, json::Value>, scope_name: &str, color: impl Into<Color>) {
    map.insert(scope_name.to_string(), (&color.into()).into());
}

fn style_rule(map: &mut json::Map<String, json::Value>, scope_name: &str, style: impl Into<Style>) {
    map.insert(scope_name.to_string(), style.into().as_json_value(false));
}
