use crate::palette::*;
use crate::style::{FontStyle, Rgb, Style};

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

        rule(colors, "activityBar.activeBorder", ZENBURN_FG);
        rule(colors, "activityBar.background", ZENBURN_BG_MINUS_05);
        rule(colors, "activityBar.foreground", ZENBURN_FG);
        rule(colors, "activityBar.inactiveForeground", ZENBURN_BG_PLUS_3);
        rule(colors, "activityBarBadge.background", ZENBURN_BLUE_MINUS_3);
        rule(colors, "activityBarBadge.foreground", ZENBURN_FG);
        rule(colors, "badge.background", ZENBURN_BG_PLUS_2);
        rule(colors, "badge.foreground", ZENBURN_FG);
        rule(colors, "button.background", ZENBURN_BLUE_PLUS_1);
        rule(colors, "button.foreground", ZENBURN_BG);
        rule(colors, "button.hoverBackground", ZENBURN_FG);
        rule(colors, "debugIcon.breakpointForeground", ZENBURN_RED);
        rule(
            colors,
            "diffEditor.insertedTextBackground",
            (ZENBURN_GREEN_PLUS_4, 0x33),
        );
        rule(
            colors,
            "diffEditor.removedTextBackground",
            (ZENBURN_RED_PLUS_2, 0x33),
        );
        rule(colors, "editor.background", ZENBURN_BG);
        rule(
            colors,
            "editor.findMatchBackground",
            (ZENBURN_BLUE_PLUS_1, 0x66),
        );
        rule(
            colors,
            "editor.findMatchHighlightBackground",
            (ZENBURN_BLUE_PLUS_1, 0x44),
        );
        rule(colors, "editor.foldBackground", (ZENBURN_BLUE_PLUS_1, 0x22));
        rule(colors, "editor.foreground", ZENBURN_FG);
        rule(colors, "editor.hoverHighlightBackground", ZENBURN_BG_PLUS_1);
        rule(
            colors,
            "editor.lineHighlightBackground",
            ZENBURN_BG_MINUS_05,
        );
        rule(colors, "editor.selectionBackground", ZENBURN_BG_MINUS_1);
        rule(
            colors,
            "editor.selectionHighlightBackground",
            ZENBURN_BG_PLUS_2,
        );
        rule(
            colors,
            "editor.symbolHighlightBackground",
            ZENBURN_BG_PLUS_2,
        );
        rule(colors, "editor.wordHighlightBackground", ZENBURN_BG_PLUS_2);
        rule(
            colors,
            "editor.wordHighlightStrongBackground",
            ZENBURN_BG_PLUS_2,
        );
        rule(colors, "editorCursor.foreground", ZENBURN_FG_PLUS_1);
        rule(colors, "editorError.foreground", ZENBURN_RED_PLUS_1);
        rule(
            colors,
            "editorGroup.dropBackground",
            (ZENBURN_BLUE_PLUS_1, 0x22),
        );
        rule(
            colors,
            "editorGroupHeader.noTabsBackground",
            ZENBURN_BG_PLUS_05,
        );
        rule(
            colors,
            "editorGroupHeader.tabsBackground",
            ZENBURN_BG_MINUS_1,
        );
        rule(colors, "editorGutter.addedBackground", ZENBURN_GREEN);
        rule(colors, "editorGutter.deletedBackground", ZENBURN_RED);
        rule(
            colors,
            "editorGutter.modifiedBackground",
            ZENBURN_YELLOW_MINUS_2,
        );
        rule(
            colors,
            "editorIndentGuide.activeBackground",
            ZENBURN_BG_PLUS_2,
        );
        rule(colors, "editorIndentGuide.background", ZENBURN_BG_PLUS_1);
        rule(colors, "editorLightBulb.foreground", ZENBURN_YELLOW);
        rule(colors, "editorLineNumber.foreground", ZENBURN_BG_PLUS_2);
        rule(colors, "editorLink.activeForeground", ZENBURN_BLUE_PLUS_1);
        rule(colors, "editorOverviewRuler.addedForeground", ZENBURN_GREEN);
        rule(colors, "editorOverviewRuler.border", ZENBURN_BG_PLUS_2);
        rule(colors, "editorOverviewRuler.deletedForeground", ZENBURN_RED);
        rule(
            colors,
            "editorOverviewRuler.errorForeground",
            ZENBURN_RED_MINUS_2,
        );
        rule(
            colors,
            "editorOverviewRuler.findMatchForeground",
            (ZENBURN_BLUE_PLUS_1, 0x88),
        );
        rule(
            colors,
            "editorOverviewRuler.modifiedForeground",
            ZENBURN_YELLOW_MINUS_2,
        );
        rule(colors, "editorWarning.foreground", ZENBURN_ORANGE);
        rule(colors, "editorWidget.background", ZENBURN_BG_MINUS_05);
        rule(colors, "editorWidget.border", ZENBURN_BG_PLUS_1);
        rule(colors, "focusBorder", ZENBURN_BG_PLUS_2);
        rule(colors, "foreground", ZENBURN_FG);
        rule(
            colors,
            "gitDecoration.ignoredResourceForeground",
            ZENBURN_BG_PLUS_3,
        );
        rule(
            colors,
            "gitDecoration.modifiedResourceForeground",
            ZENBURN_YELLOW,
        );
        rule(
            colors,
            "gitDecoration.untrackedResourceForeground",
            ZENBURN_GREEN_PLUS_3,
        );
        rule(colors, "input.background", (Rgb(0xFFFFFF), 0x0A)); // input field lightens what is behind it
        rule(colors, "input.foreground", ZENBURN_FG);
        rule(colors, "input.placeholderForeground", ZENBURN_BG_PLUS_3);
        rule(colors, "list.activeSelectionBackground", ZENBURN_BG_PLUS_1);
        rule(colors, "list.activeSelectionForeground", ZENBURN_FG);
        rule(colors, "list.errorForeground", ZENBURN_RED_PLUS_1);
        rule(colors, "list.focusBackground", ZENBURN_BG_PLUS_1);
        rule(colors, "list.highlightForeground", ZENBURN_BLUE_PLUS_3);
        rule(colors, "list.hoverBackground", ZENBURN_BG);
        rule(
            colors,
            "list.inactiveSelectionBackground",
            ZENBURN_BG_PLUS_05,
        );
        rule(colors, "list.warningForeground", ZENBURN_ORANGE);
        rule(colors, "minimap.errorHighlight", ZENBURN_RED_PLUS_2);
        rule(
            colors,
            "minimap.findMatchHighlight",
            (ZENBURN_BLUE_PLUS_1, 0x66),
        );
        rule(
            colors,
            "minimap.selectionHighlight",
            (ZENBURN_BG_MINUS_1, 0x88),
        );
        rule(colors, "panel.background", ZENBURN_BG_PLUS_05);
        rule(colors, "panel.border", ZENBURN_BG_PLUS_2);
        rule(colors, "panelTitle.activeForeground", ZENBURN_FG);
        rule(colors, "peekView.border", ZENBURN_BG_PLUS_3);
        rule(colors, "peekViewEditor.background", ZENBURN_BG);
        rule(
            colors,
            "peekViewEditor.matchHighlightBackground",
            (ZENBURN_BLUE_PLUS_1, 0x66),
        );
        rule(colors, "peekViewResult.background", ZENBURN_BG_MINUS_05);
        rule(colors, "peekViewResult.fileForeground", ZENBURN_FG);
        rule(colors, "peekViewResult.lineForeground", (ZENBURN_FG, 0x99));
        rule(
            colors,
            "peekViewResult.matchHighlightBackground",
            (ZENBURN_BLUE_PLUS_1, 0x44),
        );
        rule(
            colors,
            "peekViewResult.selectionBackground",
            ZENBURN_BG_PLUS_1,
        );
        rule(colors, "peekViewResult.selectionForeground", ZENBURN_FG);
        rule(colors, "peekViewTitle.background", ZENBURN_BG_MINUS_05);
        rule(
            colors,
            "peekViewTitleDescription.foreground",
            ZENBURN_BLUE_PLUS_1,
        );
        rule(colors, "peekViewTitleLabel.foreground", ZENBURN_FG_PLUS_1);
        rule(colors, "progressBar.background", ZENBURN_BLUE_PLUS_1);
        rule(colors, "rust_analyzer.inlayHints.foreground", ZENBURN_GREEN);
        rule(colors, "scrollbar.shadow", (Rgb(0x000000), 0x88));
        rule(colors, "selection.background", (Rgb(0xFFFFFF), 0x55));
        rule(colors, "settings.headerForeground", ZENBURN_FG_PLUS_1);
        rule(
            colors,
            "settings.modifiedItemIndicator",
            ZENBURN_BLUE_PLUS_1,
        );
        rule(colors, "sideBar.background", ZENBURN_BG_MINUS_05);
        rule(colors, "sideBar.foreground", ZENBURN_FG);
        rule(colors, "sideBarTitle.foreground", ZENBURN_FG_PLUS_1);
        rule(colors, "statusBar.background", ZENBURN_BG_MINUS_1);
        rule(colors, "statusBar.debuggingBackground", ZENBURN_BG_MINUS_1);
        rule(colors, "statusBar.debuggingBorder", ZENBURN_MAGENTA);
        rule(colors, "statusBar.foreground", ZENBURN_GREEN_PLUS_1);
        rule(colors, "statusBar.noFolderBackground", ZENBURN_BG_MINUS_1);
        rule(colors, "tab.activeForeground", ZENBURN_FG);
        rule(colors, "tab.border", ZENBURN_BG);
        rule(colors, "tab.inactiveBackground", ZENBURN_BG_MINUS_1);
        rule(colors, "tab.inactiveForeground", ZENBURN_BG_PLUS_3);
        rule(colors, "terminal.ansiBlack", ZENBURN_BG_MINUS_1);
        rule(colors, "terminal.ansiBlue", ZENBURN_BLUE_PLUS_1);
        rule(colors, "terminal.ansiBrightBlack", ZENBURN_BG_PLUS_3);
        rule(colors, "terminal.ansiBrightBlue", ZENBURN_BLUE_PLUS_3);
        rule(colors, "terminal.ansiBrightCyan", ZENBURN_CYAN);
        rule(colors, "terminal.ansiBrightGreen", ZENBURN_GREEN_PLUS_4);
        rule(colors, "terminal.ansiBrightMagenta", ZENBURN_ORANGE);
        rule(colors, "terminal.ansiBrightRed", ZENBURN_RED_PLUS_2);
        rule(colors, "terminal.ansiBrightWhite", ZENBURN_FG_PLUS_1);
        rule(colors, "terminal.ansiBrightYellow", ZENBURN_YELLOW);
        rule(colors, "terminal.ansiCyan", ZENBURN_BLUE_MINUS_1);
        rule(colors, "terminal.ansiGreen", ZENBURN_GREEN);
        rule(colors, "terminal.ansiMagenta", ZENBURN_ORANGE);
        rule(colors, "terminal.ansiRed", ZENBURN_RED);
        rule(colors, "terminal.ansiWhite", ZENBURN_FG);
        rule(colors, "terminal.ansiYellow", ZENBURN_YELLOW_MINUS_2);
        rule(colors, "terminal.foreground", ZENBURN_FG);
        rule(colors, "terminal.selectionBackground", ZENBURN_BG_MINUS_05); // Lighter than normal selection background to compensate for lighter terminal background
        rule(colors, "terminalCursor.foreground", ZENBURN_FG_PLUS_1);
        rule(colors, "textLink.activeForeground", ZENBURN_BLUE_PLUS_1);
        rule(colors, "textLink.foreground", ZENBURN_BLUE_PLUS_1);
        rule(colors, "textPreformat.foreground", ZENBURN_FG); // inline code in e.g. Settings page
        rule(colors, "titleBar.activeBackground", ZENBURN_BG_MINUS_05);
        rule(colors, "titleBar.activeForeground", ZENBURN_FG);
        rule(colors, "titleBar.inactiveBackground", ZENBURN_BG_MINUS_05);
        rule(colors, "titleBar.inactiveForeground", ZENBURN_BG_PLUS_3);
        rule(colors, "widget.shadow", (Rgb(0x000000), 0x88));

        json::Value::Object(colors_owned)
    }

    fn semantic_colors(&self) -> json::Value {
        let mut colors_owned = json::Map::new();
        let colors = &mut colors_owned;

        rule(colors, "boolean", (ZENBURN_BLUE_PLUS_3, FontStyle::Clear));
        rule(colors, "comment", ZENBURN_GREEN);
        rule(colors, "comment.documentation", ZENBURN_GREEN_PLUS_2);
        rule(colors, "keyword", (ZENBURN_YELLOW, FontStyle::Bold));
        rule(colors, "*.unsafe", ZENBURN_RED_MINUS_1);
        rule(colors, "function.unsafe", ZENBURN_RED_MINUS_1);
        rule(colors, "operator.unsafe", ZENBURN_RED_MINUS_1);
        rule(colors, "property", ZENBURN_GREEN_PLUS_3);
        rule(colors, "function", ZENBURN_CYAN);
        rule(colors, "namespace", ZENBURN_GREEN_PLUS_4);
        rule(colors, "macro", ZENBURN_BLUE_PLUS_1);
        rule(colors, "formatSpecifier", ZENBURN_BLUE_PLUS_1);
        rule(colors, "variable", ZENBURN_FG);
        rule(colors, "variable.static.constant", ZENBURN_BLUE_PLUS_3);
        rule(colors, "struct", ZENBURN_BLUE_MINUS_1);
        rule(colors, "enum", ZENBURN_BLUE_MINUS_1);
        rule(colors, "union", ZENBURN_BLUE_MINUS_1);
        rule(colors, "typeAlias", ZENBURN_BLUE_MINUS_1);
        rule(colors, "builtinType", ZENBURN_BLUE);
        rule(colors, "type", ZENBURN_BLUE_MINUS_1);
        rule(colors, "interface", ZENBURN_BLUE);
        rule(colors, "enumMember", ZENBURN_BLUE_PLUS_3);
        rule(colors, "typeParameter", ZENBURN_ORANGE);
        rule(colors, "lifetime", (ZENBURN_ORANGE, FontStyle::Italic));
        rule(colors, "number", ZENBURN_GREEN_PLUS_4);
        rule(colors, "string", ZENBURN_RED);
        rule(colors, "attribute", ZENBURN_BLUE_PLUS_1);
        rule(colors, "function.attribute", ZENBURN_BLUE_PLUS_1);
        rule(colors, "*.mutable", FontStyle::Underline);

        json::Value::Object(colors_owned)
    }

    fn textmate_colors(&self) -> json::Value {
        let mut colors_owned = Vec::new();
        let colors = &mut colors_owned;

        textmate_rule(
            colors,
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
        );

        textmate_rule(
            colors,
            Some("Function"),
            &["entity.name.function", "support.function"],
            ZENBURN_CYAN,
        );

        textmate_rule(
            colors,
            Some("Namespace"),
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
            None,
            &["entity.name.macro", "entity.name.other.preprocessor.macro"],
            ZENBURN_BLUE_PLUS_1,
        );

        textmate_rule(colors, Some("Variable"), &["variable"], ZENBURN_FG);

        textmate_rule(
            colors,
            Some("Constant"),
            &[
                "constant",
                "entity.name.constant",
                "variable.other.enummember",
                "support.constant",
            ],
            ZENBURN_BLUE_PLUS_3,
        );

        textmate_rule(colors, None, &["meta.mutable"], FontStyle::Underline);

        textmate_rule(
            colors,
            Some("Type"),
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
            (ZENBURN_BLUE, FontStyle::Clear),
        );

        textmate_rule(
            colors,
            None,
            &["entity.name.type.parameter"],
            ZENBURN_ORANGE,
        );

        textmate_rule(
            colors,
            None,
            &[
                "storage.modifier.lifetime.rust",
                "entity.name.lifetime.rust",
                "entity.name.type.lifetime",
            ],
            (ZENBURN_ORANGE, FontStyle::Italic),
        );

        textmate_rule(
            colors,
            None,
            &["constant.numeric", "keyword.other.unit"],
            (ZENBURN_GREEN_PLUS_4, FontStyle::Clear),
        );

        textmate_rule(
            colors,
            Some("Comment"),
            &["comment", "punctuation.definition.comment"],
            ZENBURN_GREEN,
        );

        textmate_rule(
            colors,
            Some("String and Character"),
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
            Some("Annotations"),
            &[
                "meta.attribute",
                "support.variable.attribute",
                "punctuation.definition.attributeentry",
            ],
            ZENBURN_BLUE_PLUS_1,
        );

        textmate_rule(
            colors,
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
            (ZENBURN_YELLOW, FontStyle::Bold),
        );

        textmate_rule(
            colors,
            Some("CSS Class/ID"),
            &[
                "entity.other.attribute-name.class",
                "entity.other.attribute-name.id",
            ],
            ZENBURN_BLUE_MINUS_1,
        );

        textmate_rule(colors, None, &["keyword.other.unsafe"], ZENBURN_RED_MINUS_1);

        textmate_rule(
            colors,
            Some("Punctuation"),
            &["keyword.operator", "punctuation"],
            (ZENBURN_FG, FontStyle::Clear),
        );

        textmate_rule(colors, None, &["markup.italic"], FontStyle::Italic);
        textmate_rule(colors, None, &["markup.bold"], FontStyle::Bold);
        textmate_rule(colors, None, &["markup.heading"], FontStyle::Underline);

        textmate_rule(
            colors,
            None,
            &["keyword.operator.logical.python"],
            (ZENBURN_YELLOW, FontStyle::Bold),
        );

        textmate_rule(
            colors,
            None,
            &["meta.function-call.generic.python", "source.python support"],
            ZENBURN_CYAN,
        );

        json::Value::Array(colors_owned)
    }
}

fn textmate_rule(
    rules: &mut Vec<json::Value>,
    name: Option<&str>,
    scopes: &[&str],
    style: impl Into<Style>,
) {
    let mut map = json::Map::new();

    if let Some(name) = name {
        map.insert("name".to_string(), json::Value::String(name.to_string()));
    }

    let scope = if let [scope] = scopes {
        json::Value::String(scope.to_string())
    } else {
        json::Value::Array(
            scopes
                .iter()
                .map(|scope| json::Value::String(scope.to_string()))
                .collect(),
        )
    };
    map.insert("scope".to_string(), scope);

    map.insert("settings".to_string(), style.into().as_json_value(true));

    rules.push(json::Value::Object(map));
}

fn rule(map: &mut json::Map<String, json::Value>, scope_name: &str, style: impl Into<Style>) {
    map.insert(scope_name.to_string(), style.into().as_json_value(false));
}
