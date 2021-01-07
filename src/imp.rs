use tincture::{ColorSpace, Oklch};

use crate::palette::*;
use crate::style::FontStyle;
use crate::theme::Scope::*;
use crate::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder) {
    workspace_colors(builder);
    syntax_highlighting(builder);
}

fn workspace_colors(builder: &mut ThemeBuilder) {
    builder.add_workspace_rule("activityBar.activeBorder", fg());
    builder.add_workspace_rule("activityBar.background", greyscale(-1));
    builder.add_workspace_rule("activityBar.foreground", fg());
    builder.add_workspace_rule("activityBar.inactiveForeground", greyscale(4));
    builder.add_workspace_rule("activityBarBadge.background", blue(2));
    builder.add_workspace_rule("activityBarBadge.foreground", greyscale(0));
    builder.add_workspace_rule("badge.background", greyscale(3));
    builder.add_workspace_rule("badge.foreground", fg());
    builder.add_workspace_rule("button.background", blue(2));
    builder.add_workspace_rule("button.foreground", greyscale(0));
    builder.add_workspace_rule("button.hoverBackground", fg());
    builder.add_workspace_rule("debugIcon.breakpointForeground", red(2));
    builder.add_workspace_rule(
        "diffEditor.insertedTextBackground",
        (green(ColorLightnessPreset::DiffBg), 0x33),
    );
    builder.add_workspace_rule(
        "diffEditor.removedTextBackground",
        (red(ColorLightnessPreset::DiffBg), 0x33),
    );
    builder.add_workspace_rule("editor.background", greyscale(0));
    builder.add_workspace_rule("editor.findMatchBackground", (blue(2), 0x66));
    builder.add_workspace_rule("editor.findMatchHighlightBackground", (blue(2), 0x44));
    builder.add_workspace_rule("editor.foldBackground", (blue(2), 0x22));
    builder.add_workspace_rule("editor.foreground", fg());
    builder.add_workspace_rule("editor.hoverHighlightBackground", greyscale(2));
    builder.add_workspace_rule("editor.lineHighlightBackground", greyscale(-1));
    builder.add_workspace_rule("editor.selectionBackground", greyscale(-2));
    builder.add_workspace_rule("editor.selectionHighlightBackground", greyscale(3));
    builder.add_workspace_rule("editor.symbolHighlightBackground", greyscale(3));
    builder.add_workspace_rule("editor.wordHighlightBackground", greyscale(3));
    builder.add_workspace_rule("editor.wordHighlightStrongBackground", greyscale(3));
    builder.add_workspace_rule("editorCursor.foreground", bright_fg());
    builder.add_workspace_rule("editorError.foreground", red(2));
    builder.add_workspace_rule("editorGroup.dropBackground", (blue(2), 0x22));
    builder.add_workspace_rule("editorGroupHeader.noTabsBackground", greyscale(1));
    builder.add_workspace_rule("editorGroupHeader.tabsBackground", greyscale(-2));
    builder.add_workspace_rule(
        "editorGutter.addedBackground",
        green(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule(
        "editorGutter.deletedBackground",
        red(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule(
        "editorGutter.modifiedBackground",
        yellow(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule("editorGroup.border", greyscale(3));
    builder.add_workspace_rule("editorIndentGuide.activeBackground", greyscale(3));
    builder.add_workspace_rule("editorIndentGuide.background", greyscale(2));
    builder.add_workspace_rule("editorLightBulb.foreground", yellow(4));
    builder.add_workspace_rule("editorLineNumber.foreground", greyscale(3));
    builder.add_workspace_rule("editorLink.activeForeground", blue(2));
    builder.add_workspace_rule(
        "editorOverviewRuler.addedForeground",
        green(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule("editorOverviewRuler.border", greyscale(3));
    builder.add_workspace_rule(
        "editorOverviewRuler.deletedForeground",
        red(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.errorForeground",
        red(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.findMatchForeground",
        (blue(ColorLightnessPreset::OverviewRuler), 0x88),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.modifiedForeground",
        yellow(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule("editorWarning.foreground", orange(2));
    builder.add_workspace_rule("editorWidget.background", greyscale(-1));
    builder.add_workspace_rule("editorWidget.border", greyscale(2));
    builder.add_workspace_rule("focusBorder", greyscale(3));
    builder.add_workspace_rule("foreground", fg());
    builder.add_workspace_rule("gitDecoration.ignoredResourceForeground", greyscale(4));
    builder.add_workspace_rule(
        "gitDecoration.modifiedResourceForeground",
        yellow(ColorLightnessPreset::GitDecoration),
    );
    builder.add_workspace_rule(
        "gitDecoration.untrackedResourceForeground",
        green(ColorLightnessPreset::GitDecoration),
    );
    builder.add_workspace_rule("input.background", (Oklch::WHITE, 0x0A)); // input field lightens what is behind it
    builder.add_workspace_rule("input.foreground", fg());
    builder.add_workspace_rule("input.placeholderForeground", greyscale(4));
    builder.add_workspace_rule("list.activeSelectionBackground", greyscale(2));
    builder.add_workspace_rule("list.activeSelectionForeground", fg());
    builder.add_workspace_rule("list.errorForeground", red(2));
    builder.add_workspace_rule("list.focusBackground", greyscale(2));
    builder.add_workspace_rule("list.highlightForeground", blue(4));
    builder.add_workspace_rule("list.hoverBackground", greyscale(0));
    builder.add_workspace_rule("list.inactiveSelectionBackground", greyscale(1));
    builder.add_workspace_rule("list.warningForeground", orange(2));
    builder.add_workspace_rule("minimap.errorHighlight", red(ColorLightnessPreset::Minimap));
    builder.add_workspace_rule(
        "minimap.findMatchHighlight",
        (blue(ColorLightnessPreset::Minimap), 0x66),
    );
    builder.add_workspace_rule("minimap.selectionHighlight", (greyscale(-2), 0x88));
    builder.add_workspace_rule("panel.background", greyscale(1));
    builder.add_workspace_rule("panel.border", greyscale(3));
    builder.add_workspace_rule("panelTitle.activeForeground", fg());
    builder.add_workspace_rule("peekView.border", greyscale(4));
    builder.add_workspace_rule("peekViewEditor.background", greyscale(0));
    builder.add_workspace_rule("peekViewEditor.matchHighlightBackground", (blue(2), 0x66));
    builder.add_workspace_rule("peekViewResult.background", greyscale(-1));
    builder.add_workspace_rule("peekViewResult.fileForeground", fg());
    builder.add_workspace_rule("peekViewResult.lineForeground", (fg(), 0x99));
    builder.add_workspace_rule("peekViewResult.matchHighlightBackground", (blue(2), 0x44));
    builder.add_workspace_rule("peekViewResult.selectionBackground", greyscale(2));
    builder.add_workspace_rule("peekViewResult.selectionForeground", fg());
    builder.add_workspace_rule("peekViewTitle.background", greyscale(-1));
    builder.add_workspace_rule("peekViewTitleDescription.foreground", blue(2));
    builder.add_workspace_rule("peekViewTitleLabel.foreground", bright_fg());
    builder.add_workspace_rule("progressBar.background", blue(2));
    builder.add_workspace_rule("rust_analyzer.inlayHints.foreground", green(0));
    builder.add_workspace_rule("scrollbar.shadow", (Oklch::BLACK, 0x88));
    builder.add_workspace_rule("selection.background", (Oklch::WHITE, 0x55));
    builder.add_workspace_rule("settings.headerForeground", bright_fg());
    builder.add_workspace_rule("settings.modifiedItemIndicator", blue(2));
    builder.add_workspace_rule("sideBar.background", greyscale(-1));
    builder.add_workspace_rule("sideBar.foreground", fg());
    builder.add_workspace_rule("sideBarTitle.foreground", bright_fg());
    builder.add_workspace_rule("statusBar.background", greyscale(-2));
    builder.add_workspace_rule("statusBar.debuggingBackground", greyscale(-2));
    builder.add_workspace_rule(
        "statusBar.foreground",
        green(ColorLightnessPreset::StatusBar),
    );
    builder.add_workspace_rule(
        "statusBar.debuggingForeground",
        orange(ColorLightnessPreset::StatusBar),
    );
    builder.add_workspace_rule("statusBar.noFolderBackground", greyscale(-2));
    builder.add_workspace_rule("tab.activeForeground", fg());
    builder.add_workspace_rule("tab.border", greyscale(0));
    builder.add_workspace_rule("tab.inactiveBackground", greyscale(-2));
    builder.add_workspace_rule("tab.inactiveForeground", greyscale(4));
    builder.add_workspace_rule("terminal.ansiBlack", greyscale(-2));
    builder.add_workspace_rule(
        "terminal.ansiBlue",
        blue(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.ansiBrightBlack", greyscale(4));
    builder.add_workspace_rule(
        "terminal.ansiBrightBlue",
        blue(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightCyan",
        cyan(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightGreen",
        green(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightMagenta",
        orange(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightRed",
        red(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule("terminal.ansiBrightWhite", bright_fg());
    builder.add_workspace_rule(
        "terminal.ansiBrightYellow",
        yellow(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiCyan",
        cyan(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule(
        "terminal.ansiGreen",
        green(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule(
        "terminal.ansiMagenta",
        orange(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.ansiRed", red(ColorLightnessPreset::TerminalAnsi));
    builder.add_workspace_rule("terminal.ansiWhite", fg());
    builder.add_workspace_rule(
        "terminal.ansiYellow",
        yellow(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.foreground", fg());
    builder.add_workspace_rule("terminal.selectionBackground", greyscale(-1)); // Lighter than normal selection background to compensate for lighter terminal background
    builder.add_workspace_rule("terminalCursor.foreground", bright_fg());
    builder.add_workspace_rule("textLink.activeForeground", blue(2));
    builder.add_workspace_rule("textLink.foreground", blue(2));
    builder.add_workspace_rule("textPreformat.foreground", fg()); // inline code in e.g. Settings page
    builder.add_workspace_rule("titleBar.activeBackground", greyscale(-1));
    builder.add_workspace_rule("titleBar.activeForeground", fg());
    builder.add_workspace_rule("titleBar.inactiveBackground", greyscale(-1));
    builder.add_workspace_rule("titleBar.inactiveForeground", greyscale(4));
    builder.add_workspace_rule("widget.shadow", (Oklch::BLACK, 0x88));
}

fn syntax_highlighting(builder: &mut ThemeBuilder) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Textmate("constant.language.null"),
            Textmate("entity.name.tag"),
            Textmate("keyword.operator.expression"),
            Textmate("keyword.operator.logical"),
            Textmate("keyword.operator.new"),
            Textmate("keyword.operator.wordlike"),
            Textmate("keyword.type.go"),
            Textmate("keyword"),
            Textmate("markup.heading.marker"),
            Textmate("punctuation.definition.heading"),
            Textmate("storage.modifier"),
            Textmate("storage.type.class"),
            Textmate("storage.type.enum"),
            Textmate("storage.type.function.python"),
            Textmate("storage.type.function.ts"),
            Textmate("storage.type.function"),
            Textmate("storage.type.js"),
            Textmate("storage.type.rust"),
            Textmate("storage.type.struct"),
            Textmate("storage.type.ts"),
            Textmate("variable.language.this"),
        ],
        (yellow(4), FontStyle::Bold),
    );

    builder.add_rules(
        &[
            Semantic("number"),
            Textmate("constant.numeric"),
            Textmate("keyword.other.unit"),
        ],
        cyan(4),
    );

    builder.add_rules(
        &[
            Semantic("string"),
            Textmate("constant.character"),
            Textmate("punctuation.definition.char"),
            Textmate("punctuation.definition.string"),
            Textmate("string"),
        ],
        red(1),
    );

    builder.add_rules(&[Semantic("variable"), Textmate("variable")], fg());

    builder.add_rules(
        &[
            Semantic("parameter"),
            Textmate("entity.name.variable.parameter"),
            Textmate("variable.parameter"),
        ],
        fg(),
    );

    builder.add_rules(
        &[
            Semantic("boolean"),
            Semantic("enumMember"),
            Semantic("variable.static"),
            Textmate("constant"),
            Textmate("entity.name.constant"),
            Textmate("variable.other.enummember"),
            Textmate("support.constant"),
        ],
        blue(4),
    );

    builder.add_rules(
        &[
            Semantic("function"),
            Textmate("entity.name.function"),
            Textmate("meta.function-call.generic.python"),
            Textmate("support.function"),
        ],
        cyan(3),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("typeAlias"),
            Textmate("entity.name.type"),
            Textmate("storage.type"),
            Textmate("support.class"),
            Textmate("support.type"),
        ],
        cyan(1),
    );

    builder.add_rules(
        &[
            Semantic("builtinType"),
            Textmate("keyword.type"),
            Textmate("storage.type.boolean.go"),
            Textmate("storage.type.built-in"),
            Textmate("storage.type.byte.go"),
            Textmate("storage.type.error.go"),
            Textmate("storage.type.numeric.go"),
            Textmate("storage.type.primitive"),
            Textmate("storage.type.rune.go"),
            Textmate("storage.type.string.go"),
            Textmate("storage.type.uintptr.go"),
            Textmate("support.type"),
        ],
        cyan(2),
    );

    builder.add_rules(
        &[
            Semantic("typeParameter"),
            Textmate("entity.name.type.parameter"),
        ],
        orange(2),
    );

    builder.add_rules(
        &[
            Semantic("property"),
            Textmate("entity.name.field"),
            Textmate("entity.name.variable.field"),
            Textmate("punctuation.support.type.property-name"),
            Textmate("support.type.property-name"),
            Textmate("support.type.vendored.property-name"),
            Textmate("variable.other.member"),
            Textmate("variable.other.object.property"),
            Textmate("variable.other.property"),
        ],
        green(3),
    );

    builder.add_rule(Semantic("interface"), cyan(2));

    builder.add_rules(
        &[
            Semantic("namespace"),
            Textmate("entity.name.module"),
            Textmate("entity.name.namespace"),
            Textmate("entity.name.type.namespace"),
            Textmate("storage.modifier.import"),
            Textmate("storage.modifier.package"),
        ],
        green(4),
    );

    builder.add_rules(
        &[
            Semantic("macro"),
            Textmate("entity.name.macro"),
            Textmate("entity.name.other.preprocessor.macro"),
        ],
        blue(2),
    );

    builder.add_rules(
        &[
            Semantic("lifetime"),
            Textmate("storage.modifier.lifetime.rust"),
            Textmate("entity.name.lifetime.rust"),
            Textmate("entity.name.type.lifetime"),
            Textmate("punctuation.definition.lifetime"),
        ],
        (orange(2), FontStyle::Italic),
    );

    builder.add_rules(
        &[
            Semantic("escapeSequence"),
            Textmate("constant.character.escape"),
        ],
        blue(2),
    );

    builder.add_rule(Semantic("formatSpecifier"), blue(2));

    builder.add_rules(
        &[
            Semantic("comment"),
            Textmate("comment"),
            Textmate("punctuation.definition.comment"),
        ],
        green(0),
    );

    builder.add_rules(
        &[
            Semantic("comment.documentation"),
            Textmate("comment.line.documentation"),
        ],
        green(1),
    );

    builder.add_rules(
        &[
            Semantic("attribute"),
            Semantic("function.attribute"),
            Textmate("entity.name.function.decorator"),
            Textmate("meta.attribute"),
            Textmate("punctuation.brackets.attribute"),
            Textmate("punctuation.definition.annotation"),
            Textmate("punctuation.definition.attribute"),
            Textmate("punctuation.definition.decorator"),
            Textmate("storage.modifier.attribute"),
            Textmate("storage.type.annotation"),
        ],
        blue(2),
    );

    // CSS classes and IDs.
    builder.add_rules(
        &[
            Textmate("entity.other.attribute-name.class"),
            Textmate("entity.other.attribute-name.id"),
        ],
        cyan(1),
    );

    builder.add_rules(
        &[
            Semantic("*.unsafe"),
            Semantic("function.unsafe"),
            Semantic("operator.unsafe"),
            Textmate("keyword.other.unsafe"),
        ],
        red(0),
    );

    builder.add_rules(
        &[
            Semantic("punctuation"),
            Textmate("keyword.operator.logical.rust"),
            Textmate("keyword.operator"),
            Textmate("punctuation"),
        ],
        fg(),
    );

    builder.add_rule(Textmate("markup.italic"), FontStyle::Italic);
    builder.add_rule(Textmate("markup.bold"), FontStyle::Bold);
    builder.add_rule(Textmate("markup.heading"), FontStyle::Underline);

    builder.add_rules(
        &[
            Textmate("markup.inserted.diff"),
            Textmate("punctuation.definition.inserted.diff"),
        ],
        green(ColorLightnessPreset::DiffFg),
    );
    builder.add_rules(
        &[
            Textmate("markup.deleted.diff"),
            Textmate("punctuation.definition.deleted.diff"),
        ],
        red(ColorLightnessPreset::DiffFg),
    );

    builder.add_rules(
        &[
            Textmate("punctuation.definition.range.diff"),
            Textmate("meta.diff.range"),
        ],
        blue(2),
    );

    builder.add_rules(
        &[Semantic("*.mutable"), Textmate("meta.mutable")],
        FontStyle::Underline,
    );
    builder.add_rule(Semantic("*.consuming"), FontStyle::Italic);

    builder.add_rule(Semantic("unresolvedReference"), red(0));

    builder.add_rule(Semantic("magit-ref-name"), (cyan(3), FontStyle::Bold));
    builder.add_rule(
        Semantic("magit-remote-ref-name"),
        (green(0), FontStyle::Bold),
    );
    builder.add_rule(Textmate("magit.subheader"), (greyscale(5), FontStyle::Bold));
}
