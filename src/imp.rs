use crate::palette::{ColorLightnessPreset, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;
use tincture::{ColorSpace, Oklch};

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: Palette) {
    workspace_colors(builder, &palette);
    syntax_highlighting(builder, &palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("activityBar.activeBorder", palette.fg());
    builder.add_workspace_rule("activityBar.background", palette.greyscale(-1));
    builder.add_workspace_rule("activityBar.foreground", palette.fg());
    builder.add_workspace_rule("activityBar.inactiveForeground", palette.greyscale(4));
    builder.add_workspace_rule("activityBarBadge.background", palette.blue(0));
    builder.add_workspace_rule("activityBarBadge.foreground", palette.greyscale(0));
    builder.add_workspace_rule("badge.background", palette.greyscale(3));
    builder.add_workspace_rule("badge.foreground", palette.fg());
    builder.add_workspace_rule("button.background", palette.blue(0));
    builder.add_workspace_rule("button.foreground", palette.greyscale(0));
    builder.add_workspace_rule("button.hoverBackground", palette.fg());
    builder.add_workspace_rule("checkbox.background", palette.greyscale(-2));
    builder.add_workspace_rule("checkbox.border", palette.greyscale(2));
    builder.add_workspace_rule("debugIcon.breakpointForeground", palette.red(0));
    builder.add_workspace_rule(
        "diffEditor.insertedTextBackground",
        (palette.green(ColorLightnessPreset::DiffBg), 0x33),
    );
    builder.add_workspace_rule(
        "diffEditor.removedTextBackground",
        (palette.red(ColorLightnessPreset::DiffBg), 0x33),
    );
    builder.add_workspace_rule("dropdown.border", palette.greyscale(2));
    builder.add_workspace_rule("editor.background", palette.greyscale(0));
    builder.add_workspace_rule("editor.findMatchBackground", (palette.blue(0), 0x66));
    builder.add_workspace_rule(
        "editor.findMatchHighlightBackground",
        (palette.blue(0), 0x44),
    );
    builder.add_workspace_rule("editor.foldBackground", (palette.blue(0), 0x22));
    builder.add_workspace_rule("editor.foreground", palette.fg());
    builder.add_workspace_rule("editor.hoverHighlightBackground", palette.greyscale(2));
    builder.add_workspace_rule("editor.lineHighlightBackground", palette.greyscale(-1));
    builder.add_workspace_rule("editor.rangeHighlightBackground", (palette.blue(0), 0x22));
    builder.add_workspace_rule("editor.selectionBackground", palette.greyscale(-2));
    builder.add_workspace_rule("editor.selectionHighlightBackground", palette.greyscale(3));
    builder.add_workspace_rule("editor.symbolHighlightBackground", palette.greyscale(3));
    builder.add_workspace_rule("editor.wordHighlightBackground", palette.greyscale(3));
    builder.add_workspace_rule("editor.wordHighlightStrongBackground", palette.greyscale(3));
    builder.add_workspace_rule("editorCursor.foreground", palette.bright_fg());
    builder.add_workspace_rule("editorError.foreground", palette.red(0));
    builder.add_workspace_rule("editorGroup.dropBackground", (palette.blue(0), 0x22));
    builder.add_workspace_rule("editorGroupHeader.noTabsBackground", palette.greyscale(1));
    builder.add_workspace_rule("editorGroupHeader.tabsBackground", palette.greyscale(-2));
    builder.add_workspace_rule("editorGroup.border", palette.greyscale(3));
    builder.add_workspace_rule(
        "editorGutter.addedBackground",
        palette.green(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule(
        "editorGutter.deletedBackground",
        palette.red(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule(
        "editorGutter.modifiedBackground",
        palette.yellow(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule("editorGutter.background", palette.greyscale(1));
    builder.add_workspace_rule("editorIndentGuide.activeBackground", palette.greyscale(3));
    builder.add_workspace_rule("editorIndentGuide.background", palette.greyscale(2));
    builder.add_workspace_rule("editorLightBulb.foreground", palette.yellow(2));
    builder.add_workspace_rule("editorLineNumber.activeForeground", palette.greyscale(5));
    builder.add_workspace_rule("editorLineNumber.foreground", palette.greyscale(3));
    builder.add_workspace_rule("editorLink.activeForeground", palette.blue(0));
    builder.add_workspace_rule(
        "editorOverviewRuler.addedForeground",
        palette.green(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule("editorOverviewRuler.border", palette.greyscale(3));
    builder.add_workspace_rule(
        "editorOverviewRuler.deletedForeground",
        palette.red(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.errorForeground",
        palette.red(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.findMatchForeground",
        (palette.blue(ColorLightnessPreset::OverviewRuler), 0x88),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.modifiedForeground",
        palette.yellow(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.rangeHighlightForeground",
        (palette.blue(ColorLightnessPreset::OverviewRuler), 0x33),
    );
    builder.add_workspace_rule("editorWarning.foreground", palette.orange(0));
    builder.add_workspace_rule("editorWidget.background", palette.greyscale(-1));
    builder.add_workspace_rule("editorWidget.border", palette.greyscale(2));
    builder.add_workspace_rule("focusBorder", palette.greyscale(3));
    builder.add_workspace_rule("foreground", palette.fg());
    builder.add_workspace_rule(
        "gitDecoration.ignoredResourceForeground",
        palette.greyscale(4),
    );
    builder.add_workspace_rule(
        "gitDecoration.modifiedResourceForeground",
        palette.yellow(ColorLightnessPreset::GitDecoration),
    );
    builder.add_workspace_rule(
        "gitDecoration.untrackedResourceForeground",
        palette.green(ColorLightnessPreset::GitDecoration),
    );
    builder.add_workspace_rule("input.background", palette.greyscale(-2));
    builder.add_workspace_rule("input.border", palette.greyscale(2));
    builder.add_workspace_rule("input.foreground", palette.fg());
    builder.add_workspace_rule("input.placeholderForeground", palette.greyscale(4));
    builder.add_workspace_rule("list.activeSelectionBackground", palette.greyscale(2));
    builder.add_workspace_rule("list.activeSelectionForeground", palette.fg());
    builder.add_workspace_rule("list.errorForeground", palette.red(0));
    builder.add_workspace_rule("list.focusBackground", palette.greyscale(2));
    builder.add_workspace_rule("list.highlightForeground", palette.blue(2));
    builder.add_workspace_rule("list.hoverBackground", palette.greyscale(0));
    builder.add_workspace_rule("list.inactiveSelectionBackground", palette.greyscale(1));
    builder.add_workspace_rule("list.warningForeground", palette.orange(0));
    builder.add_workspace_rule(
        "minimap.errorHighlight",
        palette.red(ColorLightnessPreset::Minimap),
    );
    builder.add_workspace_rule(
        "minimap.findMatchHighlight",
        (palette.blue(ColorLightnessPreset::Minimap), 0x66),
    );
    builder.add_workspace_rule("minimap.selectionHighlight", (palette.greyscale(-2), 0x88));
    builder.add_workspace_rule("panel.background", palette.greyscale(1));
    builder.add_workspace_rule("panel.border", palette.greyscale(3));
    builder.add_workspace_rule("panelTitle.activeForeground", palette.fg());
    builder.add_workspace_rule("peekView.border", palette.greyscale(4));
    builder.add_workspace_rule("peekViewEditor.background", palette.greyscale(0));
    builder.add_workspace_rule(
        "peekViewEditor.matchHighlightBackground",
        (palette.blue(0), 0x66),
    );
    builder.add_workspace_rule("peekViewResult.background", palette.greyscale(-1));
    builder.add_workspace_rule("peekViewResult.fileForeground", palette.fg());
    builder.add_workspace_rule("peekViewResult.lineForeground", (palette.fg(), 0x99));
    builder.add_workspace_rule(
        "peekViewResult.matchHighlightBackground",
        (palette.blue(0), 0x44),
    );
    builder.add_workspace_rule("peekViewResult.selectionBackground", palette.greyscale(2));
    builder.add_workspace_rule("peekViewResult.selectionForeground", palette.fg());
    builder.add_workspace_rule("peekViewTitle.background", palette.greyscale(-1));
    builder.add_workspace_rule("peekViewTitleDescription.foreground", palette.blue(0));
    builder.add_workspace_rule("peekViewTitleLabel.foreground", palette.bright_fg());
    builder.add_workspace_rule("progressBar.background", palette.blue(0));
    builder.add_workspace_rule("rust_analyzer.inlayHints.foreground", palette.green(-2));
    builder.add_workspace_rule("scrollbar.shadow", (Oklch::BLACK, 0x88));
    builder.add_workspace_rule("selection.background", (Oklch::WHITE, 0x55));
    builder.add_workspace_rule("settings.headerForeground", palette.bright_fg());
    builder.add_workspace_rule("settings.modifiedItemIndicator", palette.blue(0));
    builder.add_workspace_rule("sideBar.background", palette.greyscale(-1));
    builder.add_workspace_rule("sideBar.foreground", palette.fg());
    builder.add_workspace_rule("sideBarTitle.foreground", palette.bright_fg());
    builder.add_workspace_rule("statusBar.background", palette.greyscale(-2));
    builder.add_workspace_rule("statusBar.debuggingBackground", palette.greyscale(-2));
    builder.add_workspace_rule(
        "statusBar.foreground",
        palette.green(ColorLightnessPreset::StatusBar),
    );
    builder.add_workspace_rule(
        "statusBar.debuggingForeground",
        palette.orange(ColorLightnessPreset::StatusBar),
    );
    builder.add_workspace_rule("statusBar.noFolderBackground", palette.greyscale(-2));
    builder.add_workspace_rule("tab.activeForeground", palette.fg());
    builder.add_workspace_rule("tab.border", palette.greyscale(0));
    builder.add_workspace_rule("tab.inactiveBackground", palette.greyscale(-2));
    builder.add_workspace_rule("tab.inactiveForeground", palette.greyscale(4));
    builder.add_workspace_rule("terminal.ansiBlack", palette.greyscale(-2));
    builder.add_workspace_rule(
        "terminal.ansiBlue",
        palette.blue(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.ansiBrightBlack", palette.greyscale(5));
    builder.add_workspace_rule(
        "terminal.ansiBrightBlue",
        palette.blue(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightCyan",
        palette.cyan(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightGreen",
        palette.green(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightMagenta",
        palette.orange(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightRed",
        palette.red(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule("terminal.ansiBrightWhite", palette.bright_fg());
    builder.add_workspace_rule(
        "terminal.ansiBrightYellow",
        palette.yellow(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiCyan",
        palette.cyan(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule(
        "terminal.ansiGreen",
        palette.green(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule(
        "terminal.ansiMagenta",
        palette.orange(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule(
        "terminal.ansiRed",
        palette.red(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.ansiWhite", palette.fg());
    builder.add_workspace_rule(
        "terminal.ansiYellow",
        palette.yellow(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.foreground", palette.fg());
    builder.add_workspace_rule("terminal.selectionBackground", palette.greyscale(-3));
    builder.add_workspace_rule("terminalCursor.foreground", palette.bright_fg());
    builder.add_workspace_rule("textLink.activeForeground", palette.blue(0));
    builder.add_workspace_rule("textLink.foreground", palette.blue(0));
    builder.add_workspace_rule("textPreformat.foreground", palette.fg()); // inline code in e.g. Settings page
    builder.add_workspace_rule("titleBar.activeBackground", palette.greyscale(-1));
    builder.add_workspace_rule("titleBar.activeForeground", palette.fg());
    builder.add_workspace_rule("titleBar.inactiveBackground", palette.greyscale(-1));
    builder.add_workspace_rule("titleBar.inactiveForeground", palette.greyscale(4));
    builder.add_workspace_rule("widget.shadow", (Oklch::BLACK, 0x88));
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
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
            Textmate("variable.language.self"),
            Textmate("variable.language.this"),
        ],
        (palette.yellow(2), FontStyle::Bold),
    );

    builder.add_rules(
        &[
            Semantic("number"),
            Textmate("constant.numeric"),
            Textmate("keyword.other.unit"),
        ],
        palette.green(1),
    );

    builder.add_rules(
        &[
            Semantic("string"),
            Textmate("constant.character"),
            Textmate("punctuation.definition.char"),
            Textmate("punctuation.definition.string"),
            Textmate("string"),
        ],
        palette.red(-1),
    );

    builder.add_rules(&[Semantic("variable"), Textmate("variable")], palette.fg());

    builder.add_rules(
        &[
            Semantic("parameter"),
            Textmate("entity.name.variable.parameter"),
            Textmate("variable.parameter"),
        ],
        palette.green(0),
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
        (palette.blue(2), FontStyle::Inherit),
    );

    builder.add_rules(
        &[
            Semantic("function"),
            Semantic("method"),
            Textmate("entity.name.function"),
            Textmate("meta.function-call.generic.python"),
            Textmate("support.function"),
        ],
        palette.cyan(1),
    );
    builder.add_rules(
        &[
            Semantic("function.public.declaration"),
            Semantic("method.public.declaration"),
        ],
        (palette.purple(1), FontStyle::Inherit),
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
        palette.cyan(-1),
    );
    builder.add_rules(
        &[
            Semantic("type.public.declaration"),
            Semantic("class.public.declaration"),
            Semantic("struct.public.declaration"),
            Semantic("enum.public.declaration"),
            Semantic("union.public.declaration"),
            Semantic("typeAlias.public.declaration"),
        ],
        (palette.purple(-1), FontStyle::Inherit),
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
        palette.cyan(0),
    );

    builder.add_rules(
        &[
            Semantic("typeParameter"),
            Textmate("entity.name.type.parameter"),
        ],
        palette.orange(0),
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
        palette.orange(0),
    );

    builder.add_rule(Semantic("interface"), (palette.cyan(0), FontStyle::Italic));
    builder.add_rule(
        Semantic("interface.public.declaration"),
        (palette.purple(0), FontStyle::Inherit),
    );
    builder.add_rule(Semantic("*.trait"), FontStyle::Italic);

    builder.add_rules(
        &[
            Semantic("namespace"),
            Textmate("entity.name.module"),
            Textmate("entity.name.namespace"),
            Textmate("entity.name.type.namespace"),
            Textmate("storage.modifier.import"),
            Textmate("storage.modifier.package"),
        ],
        palette.green(2),
    );

    builder.add_rules(
        &[
            Semantic("macro"),
            Textmate("entity.name.macro"),
            Textmate("entity.name.other.preprocessor.macro"),
        ],
        palette.blue(0),
    );

    builder.add_rules(
        &[
            Semantic("lifetime"),
            Textmate("storage.modifier.lifetime.rust"),
            Textmate("entity.name.lifetime.rust"),
            Textmate("entity.name.type.lifetime"),
            Textmate("punctuation.definition.lifetime"),
        ],
        (palette.orange(0), FontStyle::Italic),
    );

    builder.add_rules(
        &[
            Semantic("escapeSequence"),
            Textmate("constant.character.escape"),
        ],
        palette.blue(0),
    );

    builder.add_rule(Semantic("formatSpecifier"), palette.blue(0));

    builder.add_rules(
        &[
            Semantic("comment"),
            Textmate("comment"),
            Textmate("punctuation.definition.comment"),
        ],
        palette.green(-2),
    );

    builder.add_rules(
        &[
            Semantic("comment.documentation"),
            Textmate("comment.line.documentation"),
        ],
        palette.green(-1),
    );

    builder.add_rules(
        &[
            Semantic("*.attribute"),
            Textmate("entity.name.function.decorator"),
            Textmate("meta.attribute"),
            Textmate("punctuation.brackets.attribute"),
            Textmate("punctuation.definition.annotation"),
            Textmate("punctuation.definition.attribute"),
            Textmate("punctuation.definition.decorator"),
            Textmate("storage.modifier.attribute"),
            Textmate("storage.type.annotation"),
        ],
        palette.blue(0),
    );

    // CSS classes and IDs.
    builder.add_rules(
        &[
            Textmate("entity.other.attribute-name.class"),
            Textmate("entity.other.attribute-name.id"),
        ],
        palette.cyan(-1),
    );

    builder.add_rules(
        &[
            Semantic("*.unsafe"),
            Semantic("function.unsafe"),
            Semantic("operator.unsafe"),
            Textmate("keyword.other.unsafe"),
        ],
        (palette.red(-2), FontStyle::Inherit),
    );

    builder.add_rules(
        &[
            Semantic("punctuation"),
            Textmate("keyword.operator.logical.rust"),
            Textmate("keyword.operator"),
            Textmate("punctuation"),
        ],
        palette.fg(),
    );

    builder.add_rule(Textmate("markup.italic"), FontStyle::Italic);
    builder.add_rule(Textmate("markup.bold"), FontStyle::Bold);
    builder.add_rule(Textmate("markup.heading"), FontStyle::Underline);

    builder.add_rules(
        &[
            Textmate("markup.inserted.diff"),
            Textmate("punctuation.definition.inserted.diff"),
        ],
        palette.green(ColorLightnessPreset::DiffFg),
    );
    builder.add_rules(
        &[
            Textmate("markup.deleted.diff"),
            Textmate("punctuation.definition.deleted.diff"),
        ],
        palette.red(ColorLightnessPreset::DiffFg),
    );

    builder.add_rules(
        &[
            Textmate("punctuation.definition.range.diff"),
            Textmate("meta.diff.range"),
        ],
        palette.blue(0),
    );

    builder.add_rules(
        &[Semantic("*.mutable"), Textmate("meta.mutable")],
        FontStyle::Underline,
    );
    builder.add_rule(
        Semantic("*.public.declaration"),
        (palette.purple(1), FontStyle::Inherit),
    );

    builder.add_rule(
        Semantic("unresolvedReference"),
        (palette.red(-1), FontStyle::Underline),
    );

    builder.add_rule(
        Semantic("magit-ref-name"),
        (palette.cyan(1), FontStyle::Bold),
    );
    builder.add_rule(
        Semantic("magit-remote-ref-name"),
        (palette.green(-2), FontStyle::Bold),
    );
    builder.add_rule(
        Textmate("magit.header"),
        (palette.yellow(2), FontStyle::Bold),
    );
    builder.add_rule(Textmate("magit.subheader"), FontStyle::Bold);

    // Over 50 characters, the recommended limit.
    builder.add_rule(
        Textmate("invalid.deprecated.line-too-long.git-commit"),
        palette.orange(0),
    );

    // Over 72 characters, the hard limit.
    builder.add_rule(
        Textmate("invalid.illegal.line-too-long.git-commit"),
        palette.red(0),
    );
}
